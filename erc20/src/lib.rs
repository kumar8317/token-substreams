mod abi;
#[path = "./db_out.rs"]
mod db;
#[path = "./graph_out.rs"]
mod graph;
#[allow(dead_code)]
mod pb;
mod utils;

use common::remove_0x;
use pb::zdexer::eth::erc20::v1::{Approvals, Contracts, Transfers, Address};
use substreams::store::{ Deltas, StoreNew, StoreSetIfNotExistsProto, StoreSetIfNotExists, DeltaProto};
use substreams::{errors::Error};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use utils::helper::{get_approvals, get_contracts};
use utils::helper::get_transfers;


#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    Ok(Transfers {
        items: get_transfers(&blk).collect(),
    })
}

#[substreams::handlers::store]
fn store_address(transfers: Transfers, output: StoreSetIfNotExistsProto<Address>) {
    for transfer in transfers.items {
        output.set_if_not_exists(
            transfer.log_ordinal,
            &transfer.token_address.clone(),
            &Address { address: transfer.token_address.clone() },
        );
    }
}

#[substreams::handlers::map]
fn map_contracts(deltas: Deltas<DeltaProto<Address>>) -> Result<Contracts, Error> {
    
    let mut array_addresses = vec![];
    for delta in deltas.deltas {
        let token_address = delta.new_value.address;
        array_addresses.push(remove_0x(&token_address));
    }
    let contracts = get_contracts(array_addresses);
    Ok(contracts)
}

#[substreams::handlers::map]
fn map_approvals(blk: eth::Block) -> Result<Approvals, Error> {
    Ok(Approvals {
        items: get_approvals(&blk).collect(),
    })
}

// #[substreams::handlers::store]
// fn store_balance(transfers: Transfers, output: StoreAddBigInt) {
//     log::info!("Stored events {}", transfers.items.len());
//     for transfer in transfers.items {
//         output.add(
//             transfer.log_ordinal,
//             keyer::balance_key(&transfer.to, &transfer.token_address,transfer.block_number),
//             &BigInt::from_str((transfer.quantity).as_str()).unwrap(),
//         );

//         if Hex::decode(remove_0x(&transfer.from.clone())).unwrap() != NULL_ADDRESS {
//             output.add(
//                 transfer.log_ordinal,
//                 keyer::balance_key(&transfer.from, &transfer.token_address,transfer.block_number),
//                 &BigInt::from_str((transfer.quantity).as_str())
//                     .unwrap()
//                     .neg(),
//             );
//         }
//     }
// }

#[substreams::handlers::map]
fn map_contract_entities(
    contracts: Contracts,
    // ownership_transfers: OwnershipTransfers,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    graph::contract_entity_changes(&mut entity_changes, contracts);
    // graph::contract_ownership_update_entity_change(&mut entity_changes, ownership_transfers);
    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_transfer_entities(transfers: Transfers) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    graph::transfer_entity_changes(&mut entity_changes, transfers);

    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_approval_entities(approvals: Approvals) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::approval_entity_changes(&mut entity_changes, approvals);
    Ok(entity_changes)
}

#[substreams::handlers::map]
fn graph_out(
    contract_entities: EntityChanges,
    transfer_entities: EntityChanges,
    approval_entities: EntityChanges,
) -> Result<EntityChanges, Error> {
    Ok(EntityChanges {
        entity_changes: [
            contract_entities.entity_changes,
            transfer_entities.entity_changes,
            approval_entities.entity_changes,
        ]
        .concat(),
    })
}

#[substreams::handlers::map]
fn map_contracts_db(
    contracts: Contracts,
    //ownership_transfers: OwnershipTransfers,
) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::contract_db_changes(&mut database_changes, contracts);
    //db::contract_ownership_update_db_changes(&mut database_changes, ownership_transfers);
    Ok(database_changes)
}

#[substreams::handlers::map]
fn map_transfers_db(transfers: Transfers) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::transfer_db_changes(&mut database_changes, transfers);

    Ok(database_changes)
}

#[substreams::handlers::map]
fn map_approvals_db(approvals: Approvals) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::approval_db_changes(&mut database_changes, approvals);

    Ok(database_changes)
}

#[substreams::handlers::map]
fn db_out(
    contracts_db: DatabaseChanges,
    transfers_db: DatabaseChanges,
    approvals_db: DatabaseChanges,
    // balances_db: DatabaseChanges,
) -> Result<DatabaseChanges, Error> {
    Ok(DatabaseChanges {
        table_changes: [
            contracts_db.table_changes,
            transfers_db.table_changes,
            approvals_db.table_changes,
            // balances_db.table_changes,
        ]
        .concat(),
    })
}