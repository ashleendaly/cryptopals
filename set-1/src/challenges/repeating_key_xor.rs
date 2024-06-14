use std::string::FromUtf8Error;
extern crate hex;

pub fn repeating_key_xor_encrypt(pt: &str, key: &str) -> String {
    let pt_bytes: &[u8] = pt.as_bytes();
    let key_bytes: &[u8] = key.as_bytes();
    let ciphertext_bytes: Vec<u8> = pt_bytes
        .iter()
        .zip(key_bytes.iter().cycle())
        .map(|(&p, &k)| p ^ k)
        .collect();
    return hex::encode(ciphertext_bytes);
}

pub fn repeating_key_xor_decrypt(ct: &str, key: &str) -> Result<String, FromUtf8Error> {
    let ct_bytes: Vec<u8> = hex::decode(ct).expect("invald string");
    let key_bytes = key.as_bytes();
    let plaintext_bytes = ct_bytes
        .iter()
        .zip(key_bytes.iter().cycle())
        .map(|(&c, &k)| c ^ k)
        .collect();
    return String::from_utf8(plaintext_bytes);
}
