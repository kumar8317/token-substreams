use std::str::FromStr;
use substreams::{scalar::BigInt, Hex, store::{Deltas, DeltaBigInt}};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};

use crate::{
    pb::zdexer::eth::erc1155::v1::{ Mints, Operators, Tokens, Transfers},
    utils::{
        keyer::{operator_key, token_store_key, transfer_key},
    },
};
use common::{ZERO_ADDRESS,format_with_0x};

pub fn account_create_entity_change(changes: &mut EntityChanges, account_address: String) {
    changes
        .push_change("Account", &account_address, 1, Operation::Create)
        .change("id", account_address);
}

pub fn transfer_entity_change(changes: &mut EntityChanges, transfers: Transfers) {
    for transfer in transfers.items {
        account_create_entity_change(changes, transfer.from.clone());
        account_create_entity_change(changes, transfer.to.clone());
        let key = transfer_key(transfer.block_number, transfer.log_index, &transfer.token_id);
        changes
            .push_change(
                "ERC1155Transfer",
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
            .change("quantity", BigInt::from_str(&transfer.quantity).unwrap())
            .change("log_index", transfer.log_index)
            .change("block_number", transfer.block_number)
            .change("block_hash", transfer.block_hash)
            .change("timestamp", transfer.timestamp)
            .change("transaction_index", transfer.transaction_index as u64)
            .change("transaction_type", transfer.transaction_type);
    }
}

pub fn token_create_entity_changes(changes: &mut EntityChanges, tokens: Tokens) {

    for token in tokens.items {
        
        let key = token_store_key(&token.token_address, &token.token_id);
        changes
            .push_change("ERC1155Token", &key, 1, Operation::Create)
            .change("id", key)
            .change("collection", token.token_address)
            .change("token_id", token.token_id)
            .change("metadata_uri", token.metadata_uri)
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
            .push_change("ERC1155Token", &key, 1, Operation::Update)
            .change("minter_address", mint.minter_address)
            .change("block_number_minted", mint.mint_block)
            .change("mint_trx", mint.min_trx);
    }
}

pub fn approval_operator_entity_changes(changes: &mut EntityChanges, approvals: Operators) {
    for approval in approvals.items {
        account_create_entity_change(changes, approval.owner_address.clone());
        account_create_entity_change(changes, approval.operator_address.clone());
        let key = operator_key(&approval.operator_address, &approval.token_address, &approval.trx_hash,&approval.owner_address);
        changes
            .push_change("ERC1155Operator", &key, 1, Operation::Create)
            .change("id", key)
            .change("collection", approval.token_address)
            .change("owner", approval.owner_address)
            .change("operator", approval.operator_address)
            .change("approved", approval.approved);
    }
}

pub fn balance_entity_changes(
    changes:&mut EntityChanges,
    deltas: Deltas<DeltaBigInt>
){
    use substreams::pb::substreams::store_delta::Operation as OperationDelta;

    for delta in deltas.deltas {
        let keyclone=delta.key.clone();
        let account_address = keyclone.as_str().split('/').next().unwrap().to_string();
        let token_address = keyclone.as_str().split('/').nth(1).unwrap().to_string();
        let token_id = keyclone.as_str().split('/').nth(2).unwrap().to_string();

        match delta.operation {
            OperationDelta::Create =>{
                changes
                    .push_change("ERC1155Balance", &delta.key, delta.ordinal, Operation::Create)
                    .change("id", delta.key)
                    .change("collection", token_address.clone())
                    .change("token", token_store_key(&token_address, &token_id))
                    .change("account", account_address)
                    .change("quantity", delta.new_value);
            },
            OperationDelta::Update =>{
                changes
                    .push_change("ERC1155Balance", &delta.key, delta.ordinal, Operation::Update)
                    .change("quantity", (delta.old_value,delta.new_value));
            }
            x=> panic!("unsupported operation {:?}",x),
        }
    }
}
