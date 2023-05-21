pub fn transfer_key(block_number: u64, log_index: u64) -> String {
    format!("{}/{}", block_number, log_index)
}

pub fn operator_key(trx_hash: &String, log_index: u64) -> String {
    format!("{}/{}", trx_hash, log_index)
}

// pub fn balance_key(
//     account_address: &String,
//     contract_address: &String,
//     block_number: u64
// ) -> String {
//     format!("{}/{}/{}", account_address, contract_address,block_number)
// }