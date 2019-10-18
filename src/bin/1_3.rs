extern crate cryptopals;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let string: String = args.next().expect("Failed to read input.");

    let bytes = cryptopals::htob(&string.into_bytes());
    let key = cryptopals::solve_single_byte_xor(&bytes);
    let decrypted = bytes.iter().map(|b| b ^ key).collect();

    println!("{}", String::from_utf8(decrypted).unwrap());
}
