extern crate hex;
use data_encoding::BASE64;

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).expect("invalid string");
    return BASE64.encode(&bytes);
}
