extern crate hex;

pub fn fixed_xor(m1: &str, m2: &str) -> String {
    let m1_decoded = hex::decode(m1).expect("invalid string");
    let m2_decoded = hex::decode(m2).expect("invalid string");

    let xored: Vec<u8> = m1_decoded
        .iter()
        .zip(m2_decoded)
        .map(|(_m1, _m2)| _m1 ^ _m2)
        .collect();

    return hex::encode(xored);
}
