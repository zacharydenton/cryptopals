extern crate cryptopals;

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut max_score: f32 = 0.;
    let mut max_decrypted: Vec<u8> = vec![0; 0];
    for line in stdin.lock().lines() {
        let string = line.expect("Failed to read input.");
        let bytes = cryptopals::htob(&string.into_bytes());
        let mut decrypted: Vec<u8> = vec![0; bytes.len()];
        for c in ('0' as u8)..(127 as u8) {
            for i in 0..bytes.len() {
                decrypted[i] = bytes[i] ^ c;
            }
            let score = cryptopals::english_score(&decrypted);
            if score > max_score {
                max_score = score;
                max_decrypted = decrypted.clone();
            }
        }
    }
    println!("{}", String::from_utf8(max_decrypted).unwrap());
}
