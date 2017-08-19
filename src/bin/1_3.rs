extern crate cryptopals;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let string: String = args.next()
        .expect("Failed to read input.");

    let bytes = cryptopals::htob(string.into_bytes());
    let mut decrypted: Vec<u8> = vec![0; bytes.len()];
    let mut max_score: f32 = 0.;
    let mut max_c = 0;
    for c in ('0' as u8)..(127 as u8) {
        for i in 0..bytes.len() {
            decrypted[i] = bytes[i] ^ c;
        }
        let score = cryptopals::english_score(decrypted.clone());
        if score > max_score {
            max_score = score;
            max_c = c;
        }
    }
    for i in 0..bytes.len() {
        decrypted[i] = bytes[i] ^ max_c;
    }
    println!("{}", String::from_utf8(decrypted).unwrap());
}
