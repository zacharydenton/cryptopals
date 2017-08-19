extern crate cryptopals;

use std::io::{self, Read, Write};

fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input)
        .expect("Failed to read input.");

    let bytes = cryptopals::htob(input);
    let base64 = cryptopals::btoa(bytes);

    io::stdout().write(&base64)
        .expect("Failed to write output.");
}
