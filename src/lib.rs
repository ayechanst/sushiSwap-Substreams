#[path = "./abi/uniswapv3factory.rs"]
mod uniswapv3factory;
mod helpers;
mod pb;

use pb::schema::{Pool, Pools};
// use pb::sf::substreams::v1::Clock;
// use crate::pb::schema::{SushiWethPool, SushiWethPools};
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
fn map_weth_pools(block: eth::v2::Block) -> Result<Pools, substreams::errors::Error> {
    // make sure this is grabbing the right pools
    let pools = block
        .logs()
        .filter_map(|log| {
            // this is the filtering part
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                // this is the mapping part
                if let Some(pool_creation) = PoolCreated::match_and_decode(log) {
                    let token0 = format_hex(&pool_creation.token0);
                    let token1 = format_hex(&pool_creation.token1);
                    if token0 == WRAPPED_ETH_ADDRESS.to_lowercase() || token1 == WRAPPED_ETH_ADDRESS.to_lowercase() {
                        // then it means its a sushi weth pool
                        // the pool itself also has events and logs
                        substreams::log::info!("passed token0: {:?}", token0);
                        substreams::log::info!("passed token1: {:?}", token1);
                        Some(pool_creation)
                    } else {
                        substreams::log::info!("failed token0: {:?}", token0);
                        substreams::log::info!("failed token1: {:?}", token1);
                        None
                        }
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
