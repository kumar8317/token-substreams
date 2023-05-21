mod abi;
mod db;

use common::pb::zdexer::eth::events::v1::{OwnershipTransfer,OwnershipTransfers, CollectionOwners, CollectionOwner};
use common::format_with_0x;
use substreams::{errors::Error, Hex, hex};
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
const INITIALIZE_METHOD_HASH: [u8; 4] = hex!("1459457a");

#[substreams::handlers::map]
pub fn map_ownership_transfers(blk: eth::Block) -> Result<OwnershipTransfers, Error> {
    
   let ownership_transfers:Vec<OwnershipTransfer> =  blk.receipts().flat_map(|view| {
        view.receipt.logs.iter().filter_map(|log| {
            if let Some(event) = abi::ownable::events::OwnershipTransferred::match_and_decode(log) {
                return Some(
                    OwnershipTransfer {
                        contract_address: format_with_0x(Hex::encode(&log.address).to_string()),
                        previous_owner: format_with_0x(Hex::encode(&event.previous_owner).to_string()),
                        new_owner: format_with_0x(Hex::encode(&event.new_owner).to_string())
                    }
                )
            }
            None
        })
    }).collect();

    Ok(OwnershipTransfers { items: ownership_transfers })
}

#[substreams::handlers::map]
fn map_collection_owners(blk: eth::Block)-> Result<CollectionOwners,Error> {
    let mut collection_owners = CollectionOwners{items:vec![]};
    for call_view in blk.calls() {
        let tx_hash = Hex(&call_view.transaction.hash).to_string();
        let from = Hex(&call_view.transaction.from).to_string();

        let call = call_view.call;
        if call.call_type == eth::CallType::Create as i32 {
            let call_input_len = call.input.len();
            if call.call_type == eth::CallType::Call as i32
                && (call_input_len < 4 || call.input[0..4] != INITIALIZE_METHOD_HASH)
            {
                // this will check if a proxy contract has been called to create a contract.
                // if that is the case the Proxy contract will call the initialize function on the contract
                // this is part of the OpenZeppelin Proxy contract standard
                //log::debug!("{:?}if false--- ", INITIALIZE_METHOD_HASH);
                continue;
            }
            if !call.state_reverted {
                let address = Hex(&call.address).to_string();
                collection_owners.items.push(CollectionOwner {
                    token_address: format_with_0x(address),
                    owner_address: format_with_0x(from),
                    deploy_trx: format_with_0x(tx_hash),
                })
            } 
           
        }
    }
    Ok(collection_owners)
}

#[substreams::handlers::map]
fn map_collection_owner_entities(collection_owners: CollectionOwners, ownership_transfers: OwnershipTransfers)-> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    db::collection_owner_entity_change(&mut entity_changes, collection_owners);
    db::collection_ownership_update_entity_change(&mut entity_changes, ownership_transfers);
    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_collections_owners_db(collection_owners: CollectionOwners,ownership_transfers: OwnershipTransfers) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::collection_owner_db_changes(&mut database_changes, collection_owners);
    db::collection_ownership_update_db_changes(&mut database_changes, ownership_transfers);

    Ok(database_changes)
}