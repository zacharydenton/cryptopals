extern crate cryptopals;

use std::io::{self, Write};

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let string1: String = args.next()
        .expect("Failed to read input.");
    let string2: String = args.next()
        .expect("Failed to read input.");

    let bytes1 = cryptopals::htob(&string1.into_bytes());
    let bytes2 = cryptopals::htob(&string2.into_bytes());
    let result = cryptopals::xor(&bytes1, &bytes2);
    let hex = cryptopals::btoh(&result);

    io::stdout().write(&hex)
        .expect("Failed to write output.");
}
