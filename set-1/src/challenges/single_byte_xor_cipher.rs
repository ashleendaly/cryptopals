extern crate hex;

fn is_printable(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|&b| b.is_ascii_graphic() || b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
}

pub fn single_byte_xor_cipher(c1: &str) {
    let c1_bytes = hex::decode(c1).expect("invalid string");
    for key in 0..=255 {
        let xored: Vec<u8> = c1_bytes.iter().map(|&c| c ^ key).collect();
        if is_printable(&xored) {
            if let Ok(decoded_str) = String::from_utf8(xored) {
                println!("{}", c1);
                println!("Key: {}, Decoded: {}", key as char, decoded_str);
            }
        }
    }
}
