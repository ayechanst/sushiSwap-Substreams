#[path = "./abi/uniswapv3factory.rs"]
mod uniswapv3factory;
mod helpers;
mod pb;

use pb::schema::{Pool, Pools};
use pb::sf::substreams::v1::Clock;
use crate::pb::schema::{SushiWethPool, SushiWethPools};
// use substreams::store::{StoreSetProto, StoreSet, StoreGetProto}; // for store pools
use substreams::store::{StoreSetProto, StoreSet}; // for store pools minus StoreGetProto
use substreams::store::StoreNew; // for store pools
use substreams_ethereum::{pb::eth, Event};
// use substreams_ethereum::pb::eth;

use helpers::*;
use uniswapv3factory::events::PoolCreated;

// from substreams scaffolding
// use erc721::events::{Approval as ApprovalEvent, Transfer as TransferEvent};

// for db_out or graph_out
// use substreams::pb::substreams::Clock;
// use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
//

pub const ADDRESS: &str = "0xbACEB8eC6b9355Dfc0269C18bac9d6E2Bdc29C4F";
pub const WRAPPED_ETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const TRANSFER_EVENT_SIGNATURE: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
// const START_BLOCK: u64 = 18532170;

#[substreams::handlers::map]
fn map_pools_created(block: eth::v2::Block) -> Result<Pools, substreams::errors::Error> {
    // make sure this is grabbing the right pools
    let pools = block
        .logs()
        .filter_map(|log| {
            // this is the filtering part
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                // this is the mapping part
                if let Some(pool_creation) = PoolCreated::match_and_decode(log) {
                    Some(pool_creation)
                } else {
                    None
                }
            } else {
                None
            }
        })
        // this is a different map doing something else
        .map(|pool_created| Pool {
            token_0: format_hex(&pool_created.token0),
            token_1: format_hex(&pool_created.token1),
            pool: format_hex(&pool_created.pool),
        })
        .collect::<Vec<Pool>>();

    Ok(Pools { pools })
}

#[substreams::handlers::map]
fn map_sushi_weth_pools (block: eth::v2::Block, pools: Pools, clock: Clock) -> Result<SushiWethPools, substreams::errors::Error> {
    let sushi_weth_pools = block
        .logs()
        .filter_map(|log| {
            // if the log from the block is coming from the wrapped eth address
            if format_hex(log.address()) == WRAPPED_ETH_ADDRESS.to_lowercase() {
                let topic_0 = format_hex(&log.log.topics[0]);
                // then get the topics from that log
                    if topic_0 == TRANSFER_EVENT_SIGNATURE {
                        let padded_topic_2 = &log.log.topics[2];
                        let topic_2 = format_hex(&padded_topic_2[12..]);
                        // then if topic_2 from that log is going to the address of my sushi-pool
                        let sushi_pools = &pools.pools;
                        if !sushi_pools.is_empty() {
                            let sushi_pool = &sushi_pools[0];
                            let sushi_pool_address = &sushi_pool.pool;
                                if topic_2 == *sushi_pool_address {
                                    let block_num = clock.number;
                                    Some(SushiWethPool {
                                        pool: sushi_pool_address.to_string(),
                                        topic_2,
                                        weth_amount: String::from("0"),
                                        block_num: block_num.to_string(),
                                        // and listen to money going into that pool (WETH);
                                        // listen to the tx and events of those pools
                                    })
                                } else {
                                    substreams::log::info!("topic 2: {:?}", topic_2);
                                    substreams::log::info!("pool address: {:?}", sushi_pool_address);
                                    None
                                }
                        } else {
                            // substreams::log::info!("sushi_pool does not have at least 1 pool: {:?}", sushi_pools);
                            None
                        }
                    } else {
                        None
                    }
            } else {
                // substreams::log::info!("log is not weth");
                None
            }
        })
        .collect::<Vec<SushiWethPool>>();

    Ok(SushiWethPools { sushi_weth_pools })
}


#[substreams::handlers::store]
fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        let key = format!("pool:{}", pool.pool);
        store.set(0, &key, &pool)
    }
}

// #[substreams::handlers::map]
// fn map_pools_transactions(block: eth::v2::Block, pools: Pools) {

// }

// #[substreams::handlers::map]
// pub fn graph_out(
//     clock: Clock,
//     transfers: Transfers,
//     approvals: Approvals,
// ) -> Result<EntityChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();

//     if clock.number == START_BLOCK {
//         // Create the collection, we only need to do this once
//         tables.create_row("Collection", ADDRESS.to_string());
//     }

//     transfers_to_table_changes(&mut tables, &transfers);

//     approvals_to_table_changes(&mut tables, &approvals);

//     Ok(tables.to_entity_changes())
// }
