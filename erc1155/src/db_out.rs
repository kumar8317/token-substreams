use substreams::{scalar::BigInt, store::{Deltas, DeltaBigInt}, Hex};
use substreams::prelude::DeltaProto;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use crate::{pb::zdexer::eth::erc1155::v1::{Transfers, Operators, Mints, Token}, utils::keyer::{transfer_key, operator_key, token_store_key}};
use common::{ZERO_ADDRESS,format_with_0x};
use std::str::FromStr;

pub fn transfer_db_changes(
    changes:&mut DatabaseChanges,
    transfers: Transfers
) {
    for transfer in transfers.items {
       // account_db_changes(changes, transfer.from.clone());
        //account_db_changes(changes, transfer.to.clone());

        let key = transfer_key(transfer.block_number,transfer.log_index, &transfer.token_id);
        changes
            .push_change("erc1155_transfer", &key, transfer.log_ordinal, Operation::Create)
            .change("collection", (None,transfer.token_address.clone()))
            .change("token", (None,token_store_key(&transfer.token_address, &transfer.token_id)))
            .change("trx_hash", (None,transfer.trx_hash))
            .change("from_address", (None,transfer.from))
            .change("to_address", (None,transfer.to))
            .change("quantity", (None,BigInt::from_str(&transfer.quantity).unwrap()))
            .change("log_index", (None,transfer.log_index))
            .change("block_number", (None,transfer.block_number))
            .change("block_hash", (None,transfer.block_hash))
            .change("timestamp", (None,transfer.timestamp))
            .change("transaction_index", (None,transfer.transaction_index))
            .change("transaction_type", (None,transfer.transaction_type))
            .change("value", (None,transfer.value))
            .change("operator", (None,transfer.operator));
    }
}


pub fn token_db_changes(
    changes:&mut DatabaseChanges,
    token_deltas: Deltas<DeltaProto<Token>>
) {
    use substreams::pb::substreams::store_delta::Operation as OperationDelta;

    for delta in token_deltas.deltas {
        match delta.operation {
            OperationDelta::Create =>{
                let token = delta.new_value;
                changes
                .push_change("erc1155_token",&delta.key ,delta.ordinal, Operation::Create)
                    .change("collection", (None,token.token_address))
                    .change("token_id", (None,token.token_id))
                    .change("metadata_uri", (None,token.metadata_uri))
                    .change("owner_address", (None,token.owner_address))
                    .change("block_number", (None,token.block_number))
                    .change(
                        "minter_address",
                        (None,format_with_0x(Hex::encode(ZERO_ADDRESS).to_string())),
                    )
                    .change("block_number_minted", (None,BigInt::zero()))
                    .change(
                        "mint_trx",
                        (None,format_with_0x(Hex::encode(ZERO_ADDRESS).to_string())),
                    );
            },
            OperationDelta::Update =>{
                let token = delta.new_value;
                let old_token=delta.old_value;
                changes
                    .push_change("erc1155_token", &delta.key, delta.ordinal, Operation::Update)
                    .change("owner_address", (old_token.owner_address,token.owner_address))
                    .change("block_number", (old_token.block_number,token.block_number));
            },
            x=> panic!("unsupported operation {:?}",x),
        }
    }
}

pub fn mints_token_db_changes(changes: &mut DatabaseChanges, mints: Mints) {
    for mint in mints.items {
        let key = token_store_key(&mint.token_address, &mint.token_id);
       // account_db_changes(changes, mint.minter_address.clone());
        changes
            .push_change("erc1155_token", &key, 1, Operation::Update)
            .change("minter_address", (None,mint.minter_address))
            .change("block_number_minted", (None,mint.mint_block))
            .change("mint_trx", (None,mint.min_trx));
    }
}

pub fn approval_operator_db_changes(
    changes:&mut DatabaseChanges,
    approvals: Operators
) {
    for approval in approvals.items {
        //account_db_changes(changes, approval.owner_address.clone());
       // account_db_changes(changes, approval.operator_address.clone());

        let key = operator_key(&approval.operator_address, &approval.token_address, &approval.trx_hash, &approval.owner_address);

        changes
            .push_change("erc1155_operator", &key, 1, Operation::Create)
            .change("collection", (None,approval.token_address))
            .change("owner", (None,approval.owner_address))
            .change("operator", (None,approval.operator_address))
            .change("approved", (None,approval.approved));
    }
}

pub fn balance_db_changes(
    changes:&mut DatabaseChanges,
    deltas: Deltas<DeltaBigInt>
) {
    use substreams::pb::substreams::store_delta::Operation as OperationDelta;

    for delta in deltas.deltas {
        let keyclone = delta.key.clone();
        let account_address = keyclone.as_str().split('/').next().unwrap().to_string();
        let token_address = keyclone.as_str().split('/').nth(1).unwrap().to_string();
        let token_id = keyclone.as_str().split('/').nth(2).unwrap().to_string();

        //account_db_changes(changes, account_address.clone());

        match delta.operation {
            OperationDelta::Create =>{
                changes
                    .push_change("erc1155_balance", &delta.key, delta.ordinal, Operation::Create)
                    .change("collection", (None,token_address.clone()))
                    .change("token", (None,token_store_key(&token_address, &token_id)))
                    .change("account", (None,account_address))
                    .change("quantity", (None,delta.new_value));
            },
            OperationDelta::Update => {
                changes
                    .push_change("erc1155_balance", &delta.key, delta.ordinal, Operation::Update)
                    .change("quantity", (delta.old_value,delta.new_value));
            },
            x => panic!("unsupported operation {:?}",x),
        }
    }
}