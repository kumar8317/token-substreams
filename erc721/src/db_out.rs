use substreams::{scalar::BigInt, Hex, store::{Deltas, DeltaProto}};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use common::{pb::zdexer::eth::events::v1::OwnershipTransfers, sanitize_string};
use crate::{pb::zdexer::eth::erc721::v1::{Collections, Transfers, Approvals, Mints, Token}, utils::{keyer::{transfer_key, operator_key, token_store_key, approval_key}}};
use common::{ZERO_ADDRESS,format_with_0x};

pub fn collection_db_changes(
    changes:&mut DatabaseChanges,
    collections: Collections
) {
    for collection in collections.items {
       // account_db_changes(changes, collection.owner_address.clone());
        let key = collection.token_address.clone();
        changes
            .push_change("erc721_collection", &key, 1, Operation::Create)
            .change("owner_address", (None,collection.owner_address))
            .change("name", (None,sanitize_string(&collection.name)))
            .change("symbol", (None,sanitize_string(&collection.symbol)))
            .change("supports_metadata", (None,collection.supports_metadata));
    }
}
pub fn collection_ownership_update_db_changes(changes: &mut DatabaseChanges, ownership_transfers: OwnershipTransfers) {
    for ownership_transfer in ownership_transfers.items {
       // account_db_changes(changes, ownership_transfer.new_owner.clone());
        changes
            .push_change("erc721_collection", &ownership_transfer.contract_address, 1, Operation::Update)
            .change("owner_address", (ownership_transfer.previous_owner,ownership_transfer.new_owner));
    }
}
// pub fn account_db_changes(
//     changes:&mut DatabaseChanges,
//     account_address: String
// ) {
//     changes
//         .push_change("account", &account_address, 1, Operation::Create);
// }

pub fn transfer_db_changes(
    changes:&mut DatabaseChanges,
    transfers: Transfers
) {
    for transfer in transfers.items {
       // account_db_changes(changes, transfer.from.clone());
       // account_db_changes(changes, transfer.to.clone());

        let key = transfer_key(transfer.block_number,transfer.log_index);
        changes
            .push_change("erc721_transfer", &key, transfer.log_ordinal, Operation::Create)
            .change("collection", (None,transfer.token_address.clone()))
            .change("token", (None,token_store_key(&transfer.token_address, &transfer.token_id)))
            .change("trx_hash", (None,transfer.trx_hash))
            .change("from_address", (None,transfer.from))
            .change("to_address", (None,transfer.to))
            .change("log_index", (None,transfer.log_index))
            .change("block_number", (None,transfer.block_number))
            .change("block_hash", (None,transfer.block_hash))
            .change("timestamp", (None,transfer.timestamp))
            .change("transaction_index", (None,transfer.transaction_index))
            .change("transaction_type", (None,transfer.transaction_type));
    }
}

// pub fn token_db_changes(
//     changes:&mut DatabaseChanges,
//     tokens: Tokens
// ) {
//     for token in tokens.items {
//         //account_db_changes(changes, token.owner_address.clone());
//         let key = token_store_key(&token.token_address, &token.token_id);

//         changes
//             .push_change("erc721_token", &key, 1, Operation::Create)
//             .change("collection", (None,token.token_address))
//             .change("token_id", (None,token.token_id))
//             .change("metadata_uri", (None,token.metadata_uri))
//             .change("owner_address", (None,token.owner_address))
//             .change("block_number", (None,token.block_number))
//             .change(
//                 "minter_address",
//                 (None,format_with_0x(Hex::encode(ZERO_ADDRESS).to_string())),
//             )
//             .change("block_number_minted", (None,BigInt::zero()))
//             .change(
//                 "approval",
//                 (None,format_with_0x(Hex::encode(ZERO_ADDRESS).to_string())),
//             )
//             .change(
//                 "mint_trx",
//                 (None,format_with_0x(Hex::encode(ZERO_ADDRESS).to_string())),
//             );

//             changes
//                 .push_change("erc721_token", &key, 1, Operation::Update);
//     }
// }

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
                    .push_change("erc721_token", &delta.key, delta.ordinal, Operation::Create)
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
                    .push_change("erc721_token", &delta.key, delta.ordinal, Operation::Update)
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
            .push_change("erc721_token", &key, 1, Operation::Update)
            .change("minter_address", (None,mint.minter_address))
            .change("block_number_minted", (None,mint.mint_block))
            .change("mint_trx", (None,mint.min_trx));
    }
}

pub fn approval_operator_db_changes(
    changes:&mut DatabaseChanges,
    approvals: Approvals
) {
    for approval in approvals.items {
        if approval.token_id.is_empty() {
           // account_db_changes(changes, approval.owner_address.clone());
          //  account_db_changes(changes, approval.operator_address.clone());
            let key = operator_key(&approval.operator_address, &approval.token_address, &approval.trx_hash, &approval.owner_address);
            changes
                .push_change("erc721_operator", &key, 1, Operation::Create)
                .change("collection", (None,approval.token_address))
                .change("owner", (None,approval.owner_address))
                .change("operator", (None,approval.operator_address))
                .change("approved", (None,approval.approved));
        }else{
            let token_key = token_store_key(&approval.token_address, &approval.token_id);
            //account_db_changes(changes, approval.operator_address.clone());
            let key = approval_key(&approval.operator_address, &approval.token_address,&approval.token_id,&approval.trx_hash, &approval.owner_address);
            changes
                .push_change("erc721_approval", &key, 1, Operation::Create)
                .change("owner", (None,approval.owner_address))
                .change("approval", (None,approval.operator_address))
                .change("token", (None,token_key));
        }
    }
}