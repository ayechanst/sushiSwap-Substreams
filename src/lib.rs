#[path = "./abi/uniswapv3factory.rs"]
mod uniswapv3factory;

mod helpers;
mod pb;

use pb::schema::{Pool, Pools};
use substreams_ethereum::{pb::eth, Event};

use helpers::*;
use uniswapv3factory::events::PoolCreated;

// from substreams scaffolding
// use erc721::events::{Approval as ApprovalEvent, Transfer as TransferEvent};

// for db_out or graph_out
// use substreams::pb::substreams::Clock;
// use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

pub const ADDRESS: &str = "0xbACEB8eC6b9355Dfc0269C18bac9d6E2Bdc29C4F";
const START_BLOCK: u64 = 18532170;

#[substreams::handlers::map]
fn map_pools_created(block: eth::v2::Block) -> Result<Pools, substreams::errors::Error> {
    let pools = block
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                if let Some(transfer) = PoolCreated::match_and_decode(log) {
                    Some((transfer, format_hex(&log.receipt.transaction.hash)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(pool_created, hash)| Pool {
            token_0: format_hex(&pool_created.token0),
            token_1: format_hex(&pool_created.token1),
            pool: format_hex(&pool_created.pool),
        })
        .collect::<Vec<Pool>>();

    Ok(Pools { pools })
}


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
