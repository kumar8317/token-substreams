use common::pb::zdexer::eth::events::v1::OwnershipTransfers;
use substreams::{Hex, scalar::BigInt};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};

use crate::{
    pb::zdexer::eth::erc721::v1::{
        Approvals, Collections, Mints, Tokens, Transfers,
    },
    utils::{
        keyer::{operator_key, transfer_key, token_store_key},
    },
};
use common::{ZERO_ADDRESS,format_with_0x};

pub fn collection_entity_change(changes: &mut EntityChanges, collections: Collections) {
    for collection in collections.items {
        account_create_entity_change(changes, collection.owner_address.clone());

        let key = collection.token_address.clone();
        changes
            .push_change("ERC721Collection", &key, 1, Operation::Create)
            .change("id", collection.token_address)
            .change("name", collection.name)
            .change("symbol", collection.symbol)
            .change("supports_metadata", collection.supports_metadata)
            .change("owner_address", collection.owner_address);
    }
}

pub fn collection_ownership_update_entity_change(changes: &mut EntityChanges, ownership_transfers: OwnershipTransfers) {
    for ownership_transfer in ownership_transfers.items {
        account_create_entity_change(changes, ownership_transfer.new_owner.clone());
        changes
            .push_change("ERC721Collection", &ownership_transfer.contract_address, 1, Operation::Update)
            .change("owner_address", ownership_transfer.new_owner);
    }
}

pub fn account_create_entity_change(changes: &mut EntityChanges, account_address: String) {
    changes
        .push_change("Account", &account_address, 1, Operation::Create)
        .change("id", account_address);
}

pub fn transfer_entity_change(changes: &mut EntityChanges, transfers: Transfers) {
    for transfer in transfers.items {
        account_create_entity_change(changes, transfer.from.clone());
        account_create_entity_change(changes, transfer.to.clone());
        let key = transfer_key(transfer.block_number, transfer.log_index);
        changes
            .push_change(
                "ERC721Transfer",
                &key,
                transfer.log_ordinal,
                Operation::Create,
            )
            .change("id", key)
            .change("collection", transfer.token_address.clone())
            .change(
                "token",
                token_store_key(&transfer.token_address, &transfer.token_id),
            )
            .change("trx_hash", transfer.trx_hash)
            .change("from_address", transfer.from)
            .change("to_address", transfer.to)
            .change("log_index", transfer.log_index)
            .change("block_number", transfer.block_number)
            .change("block_hash", transfer.block_hash)
            .change("timestamp", transfer.timestamp)
            .change("transaction_index", transfer.transaction_index as u64)
            .change("transaction_type", transfer.transaction_type);
    }
}

pub fn token_create_entity_changes(changes: &mut EntityChanges, tokens:Tokens) {
    // use substreams::pb::substreams::store_delta::Operation as OperationDelta;

    for token in tokens.items {
        account_create_entity_change(changes, token.owner_address.clone());
        let key = token_store_key(&token.token_address, &token.token_id);
        changes
            .push_change("ERC721Token", &key, 1, Operation::Create)
            .change("id", key)
            .change("collection", token.token_address)
            .change("token_id", token.token_id)
            .change("metadata_uri", token.metadata_uri)
            .change("owner_address", token.owner_address)
            .change("block_number", token.block_number)
            .change(
                "minter_address",
                format_with_0x(Hex::encode(ZERO_ADDRESS).to_string()),
            )
            .change("block_number_minted", BigInt::zero())
            .change(
                "approval",
                format_with_0x(Hex::encode(ZERO_ADDRESS).to_string()),
            )
            .change(
                "mint_trx",
                format_with_0x(Hex::encode(ZERO_ADDRESS).to_string()),
            );
    }
}

pub fn mints_token_entity_changes(changes: &mut EntityChanges, mints: Mints) {
    for mint in mints.items {
        let key = token_store_key(&mint.token_address, &mint.token_id);
        account_create_entity_change(changes, mint.minter_address.clone());
        changes
            .push_change("ERC721Token", &key, 1, Operation::Update)
            .change("minter_address", mint.minter_address)
            .change("block_number_minted", mint.mint_block)
            .change("mint_trx", mint.min_trx);
    }
}

pub fn approval_operator_entity_changes(changes: &mut EntityChanges, approvals: Approvals) {
    for approval in approvals.items {
        let key = operator_key(&approval.trx_hash, approval.log_index);
        if approval.token_id.is_empty() {
            account_create_entity_change(changes, approval.owner_address.clone());
            account_create_entity_change(changes, approval.operator_address.clone());
            changes
                .push_change("ERC721Operator", &key, 1, Operation::Create)
                .change("id", key)
                .change("collection", approval.token_address)
                .change("owner", approval.owner_address)
                .change("operator", approval.operator_address)
                .change("approved", approval.approved);
        }else{
            let token_key = token_store_key(&approval.token_address, &approval.token_id);
            account_create_entity_change(changes, approval.operator_address.clone());
            changes
                .push_change("ERC721Approval", &key, 1, Operation::Create)
                .change("id", key)
                .change("owner", approval.owner_address)
                .change("approval", approval.operator_address)
                .change("token", token_key);
        }
    }
}
