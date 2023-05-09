#[allow(unused_imports)]
#[allow(dead_code)]
#[rustfmt::skip]
pub mod pb;

use substreams::hex;

pub fn format_with_0x(address: String) -> String {
    if !address.is_empty() {
        format!("0x{}", address)
    } else {
        address
    }
}

pub fn remove_0x(hex_string: &str) -> String {
    if let Some(s) = hex_string.strip_prefix("0x") {
        s.to_string()
    } else {
        hex_string.to_string()
    }
}

pub const ZERO_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");

pub fn add_single_quote(s: &str) -> String {
    if s.contains('\'') {
        s.replace('\'', "''")
    } else {
        s.to_string()
    }
}

pub fn sanitize_string(s: &str) -> String {
    let result = s.replace('\u{0}', "");
    add_single_quote(&result)
}