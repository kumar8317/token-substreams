use common::pb::zdexer::eth::events::v1::{CollectionOwners, OwnershipTransfers};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn account_create_entity_change(changes: &mut EntityChanges, account_address: String) {
    changes
        .push_change("Account", &account_address, 1, Operation::Create)
        .change("id", account_address);
}

pub fn collection_owner_entity_change(changes: &mut EntityChanges, collection_owners: CollectionOwners) {
    for collection in collection_owners.items {
        account_create_entity_change(changes, collection.owner_address.clone());

        let key = collection.token_address.clone();
        changes
            .push_change("CollectionOwner", &key, 1, Operation::Create)
            .change("id", collection.token_address)
            .change("owner_address", collection.owner_address)
            .change("deploy_trx",collection.deploy_trx);
    }
}


pub fn collection_ownership_update_entity_change(changes: &mut EntityChanges, ownership_transfers: OwnershipTransfers) {
    for ownership_transfer in ownership_transfers.items {
        account_create_entity_change(changes, ownership_transfer.new_owner.clone());
        changes
            .push_change("CollectionOwner", &ownership_transfer.contract_address, 1, Operation::Update)
            .change("owner_address", ownership_transfer.new_owner);
    }
}

pub fn collection_owner_db_changes(
    changes:&mut DatabaseChanges,
    collection_owners: CollectionOwners
) {
    for collection in collection_owners.items {
       // account_db_changes(changes, collection.owner_address.clone());
        let key = collection.token_address.clone();
        changes
            .push_change("collection_owner", &key, 1, table_change::Operation::Create)
            .change("owner_address", (None,collection.owner_address))
            .change("deploy_trx", (None,collection.deploy_trx));
    }
}
pub fn collection_ownership_update_db_changes(changes: &mut DatabaseChanges, ownership_transfers: OwnershipTransfers) {
    for ownership_transfer in ownership_transfers.items {
       // account_db_changes(changes, ownership_transfer.new_owner.clone());
        changes
            .push_change("collection_owner", &ownership_transfer.contract_address, 1, table_change::Operation::Update)
            .change("owner_address", (ownership_transfer.previous_owner,ownership_transfer.new_owner));
    }
}