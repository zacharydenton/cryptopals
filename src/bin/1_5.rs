extern crate cryptopals;

use std::io::{self, Read, Write};

fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input)
        .expect("Failed to read input.");

    let encrypted = cryptopals::repeating_xor(input, "ICE".as_bytes().to_vec());
    let result = cryptopals::btoh(encrypted);

    io::stdout().write(&result)
        .expect("Failed to write output.");
}

