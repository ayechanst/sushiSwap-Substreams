#[path = "./abi/uniswapv3factory.rs"]
mod uniswapv3factory;
mod helpers;
mod pb;

use pb::schema::{Pool, Pools, TransferInfo, TransferInfos };
// use pb::sf::substreams::v1::Clock;
// use crate::pb::schema::{SushiWethPool, SushiWethPools};
// use substreams::store::{StoreSetProto, StoreSet, StoreGetProto}; // for store pools
use substreams::store::{StoreSetProto, StoreSet}; // for store pools minus StoreGetProto
use substreams::store::StoreNew;
// for store pools
use substreams_ethereum::{pb::eth, Event};
use substreams::scalar::BigInt;
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
pub const SWAP_EVENT_SIGNATURE: &str = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67";
// const START_BLOCK: u64 = 18532170;

#[substreams::handlers::map]
fn map_weth_pools(block: eth::v2::Block) -> Result<Pools, substreams::errors::Error> {
    let pools = block
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                if let Some(pool_creation) = PoolCreated::match_and_decode(log) {
                    let token0 = format_hex(&pool_creation.token0);
                    let token1 = format_hex(&pool_creation.token1);
                    if token0 == WRAPPED_ETH_ADDRESS.to_lowercase() || token1 == WRAPPED_ETH_ADDRESS.to_lowercase() {
                        let _value = &pool_creation.pool;
                        Some(pool_creation)
                    } else {
                        None
                        }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|pool_created| Pool {
            token_0: format_hex(&pool_created.token0),
            token_1: format_hex(&pool_created.token1),
            pool: format_hex(&pool_created.pool),
        })
        .collect::<Vec<Pool>>();

    Ok(Pools { pools })
}

#[substreams::handlers::map]
fn map_weth_transfers(block: eth::v2::Block, pools: Pools) -> Result<TransferInfos, substreams::errors::Error> {
    let pools_length = &pools.pools.len();
    if pools_length > &0 {
        substreams::log::info!("pool length: {:?}", pools_length);
        let transfer_infos = block
            .calls()
            .filter_map(|callview| {
                let pool_address = &pools.pools[0].pool;
                // substreams::log::info!("pool address: {:?}", pool_address);
                if &format_hex(&callview.transaction.from) == pool_address
                    // && &format_hex(&callview.transaction.to) == pool_address
                {
                        if let Some(value) = &callview.transaction.value {
                            Some(TransferInfo {
                                pool: pools.pools[0].pool.to_string(),
                                from: format_hex(&callview.transaction.from),
                                to: format_hex(&callview.transaction.to),
                                amount: BigInt::from_unsigned_bytes_be(&value.bytes).to_string(),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
            })
        .collect::<Vec<TransferInfo>>();
    Ok(TransferInfos { transfer_infos })
    } else {
        substreams::log::info!("pool length: {:?}", pools_length);
        Ok(TransferInfos { transfer_infos: Vec::new() })
    }
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
