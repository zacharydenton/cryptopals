extern crate cryptopals;

use std::io::{self, Read};

fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut input)
        .expect("Failed to read input.");

    // The input file is base-64 encoded. Decode to bytes.
    let ciphertext = cryptopals::atob(&input);

    // Try KEYSIZEs from 2-40; sort by normalized edit distance between the
    // first KEYSIZE worth of bytes and the second KEYSIZE worth of bytes.
    let mut keysizes_by_distance: Vec<(f32, usize)> = (2..40)
        .map(|keysize| {
            let first = &ciphertext[0..keysize];
            let second = &ciphertext[keysize..2 * keysize];
            let distance = cryptopals::hamming_distance(first, second);
            (distance as f32 / keysize as f32, keysize)
        })
        .collect();
    keysizes_by_distance.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let (_score, plaintext) = keysizes_by_distance
        .iter()
        .map(|&(_distance, keysize)| {
            // Break the ciphertext into blocks of length KEYSIZE.
            // Then, transpose them (the first transpose will contain the first
            // byte from each block, with KEYSIZE transposed blocks in total).
            let mut transposed: Vec<Vec<u8>> = vec![vec![]; keysize];
            for block in ciphertext.chunks(keysize) {
                for (i, &c) in block.iter().enumerate() {
                    transposed[i].push(c);
                }
            }

            // Then solve each transposed block as if it were a single-byte XOR
            // cipher. Since the key is repeating, we can test it one byte at a
            // time. The overall key is then the concatenation of each
            // single-byte XOR key.
            let key: Vec<u8> = transposed
                .iter()
                .map(|block| cryptopals::solve_single_byte_xor(&block))
                .collect();
            let plaintext = cryptopals::repeating_xor(&ciphertext, &key);

            // Take the plaintext that looks most like English.
            (cryptopals::english_score(&plaintext), plaintext)
        })
        .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("{}", String::from_utf8(plaintext).unwrap());
}
