pub fn transfer_key(block_number: u64, log_index: u64) -> String {
    format!("{}/{}", block_number, log_index)
}

pub fn operator_key(operator_address:  &String, token_address:  &String, trx_hash: &String, owner_address: &String) -> String {
    format!("{}/{}/{}/{}", trx_hash, operator_address, owner_address ,token_address)
}

pub fn balance_key(
    account_address: &String,
    contract_address: &String,
) -> String {
    format!("{}/{}", account_address, contract_address, )
}