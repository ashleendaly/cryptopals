mod challenges;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // --------- challenge 1
    // let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    // let base64_encoding = hex_to_base64::hex_to_base64(hex);
    // println!("{:?}", base64_encoding);

    // --------- challenge 2
    // let m1 = "1c0111001f010100061a024b53535009181c";
    // let m2 = "686974207468652062756c6c277320657965";
    // let xored = fixed_xor::fixed_xor(m1, m2);
    // println!("{:?}", xored);

    // --------- challenge 3
    // single_byte_xor_cipher::single_byte_xor_cipher(
    //     "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    // );

    // --------- challenge 4
    // let file = File::open("data/4.txt")?;
    // let reader = BufReader::new(file);
    // for line in reader.lines() {
    //     let line = line?;
    //     single_byte_xor_cipher::single_byte_xor_cipher(&line);
    // }

    // --------- challenge 5
    // let ciphertext = challenges::repeating_key_xor::repeating_key_xor_encrypt(
    //     "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
    //     "ICE",
    // );
    // println!("{:?}", ciphertext);
    // let plaintext =
    //     challenges::repeating_key_xor::repeating_key_xor_decrypt(&ciphertext, "ICE").unwrap();
    // println!("{:?}", plaintext);

    // --------- challenge 6
    let file = File::open("data/6.txt")?;
    let mut reader = BufReader::new(file);
    let mut ciphertext = String::new();
    reader.read_to_string(&mut ciphertext)?;
    let mut keysize: usize;
    for n in 2..41 {
        keysize = n;
    }

    Ok(())
}
