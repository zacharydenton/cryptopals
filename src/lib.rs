pub fn htob(hex: &[u8]) -> Vec<u8> {
    hex.chunks(2)
        .map(|pair| {
            let a = h2b(pair[0]);
            let b = h2b(pair[1]);
            (a << 4) + b
        })
        .collect()
}

pub fn btoh(bytes: &[u8]) -> Vec<u8> {
    let mut hex: Vec<u8> = vec![];
    for byte in bytes {
        hex.push(b2h(byte >> 4));
        hex.push(b2h(byte & 0xf));
    }
    return hex;
}

pub fn xor(vec1: &[u8], vec2: &[u8]) -> Vec<u8> {
    vec1.iter().zip(vec2.iter()).map(|(a, b)| *a ^ *b).collect()
}

pub fn repeating_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| *a ^ *b)
        .collect()
}

pub fn solve_single_byte_xor(bytes: &[u8]) -> u8 {
    let mut decrypted: Vec<u8> = vec![0; bytes.len()];
    ((std::u8::MIN)..(std::u8::MAX))
        .max_by_key(|key| {
            for i in 0..bytes.len() {
                decrypted[i] = bytes[i] ^ key;
            }
            english_score(&decrypted)
        })
        .unwrap()
}

pub fn hamming_distance(vec1: &[u8], vec2: &[u8]) -> u32 {
    let mut distance = 0;
    let difference = xor(vec1, vec2);
    for byte in difference {
        distance += byte.count_ones();
    }
    distance
}

pub fn btoa(bytes: &[u8]) -> Vec<u8> {
    let mut base64: Vec<u8> = vec![];
    for i in 0..(bytes.len() * 8 / 6) {
        let mut value: u8 = 0;
        for j in 0..6 {
            let byte_index = (i * 6 + j) / 8;
            let bit_index = (i * 6 + j) % 8;
            let shift = 7 - bit_index;
            let bit = (bytes[byte_index] >> shift) & 1;
            value |= bit << (5 - j);
        }
        base64.push(match value {
            0...25 => ('A' as u8) + value,
            26...51 => ('a' as u8) + (value - 26),
            52...61 => ('0' as u8) + (value - 52),
            62 => ('+' as u8),
            63 => ('/' as u8),
            _ => panic!("{} is out of base64 range.", value),
        });
    }
    base64
}

pub fn english_score(bytes: &[u8]) -> u32 {
    let mut score: u32 = 0;
    for byte in bytes {
        score = score.saturating_add(match *byte as char {
            ' ' => 150,
            'e' | 'E' => 127,
            't' | 'T' => 90,
            'a' | 'A' => 82,
            'o' | 'O' => 75,
            'i' | 'I' => 70,
            'n' | 'N' => 67,
            's' | 'S' => 63,
            'h' | 'H' => 60,
            'r' | 'R' => 60,
            'd' | 'D' => 42,
            'l' | 'L' => 40,
            '.' => 30,
            'u' | 'U' => 27,
            'c' | 'C' => 27,
            'm' | 'M' => 24,
            'w' | 'W' => 24,
            'f' | 'F' => 22,
            'g' | 'G' => 20,
            'y' | 'Y' => 20,
            'p' | 'P' => 19,
            'b' | 'B' => 15,
            'v' | 'V' => 10,
            'k' | 'K' => 8,
            'j' | 'J' => 2,
            'x' | 'X' => 2,
            'q' | 'Q' => 1,
            'z' | 'Z' => 1,
            _ => 0,
        });
    }
    return score;
}

fn h2b(h: u8) -> u8 {
    return match h as char {
        '0'...'9' => h - ('0' as u8),
        'a'...'f' => 10 + h - ('a' as u8),
        'A'...'F' => 10 + h - ('A' as u8),
        _ => panic!("{} is not a valid hexadecimal character.", h),
    };
}

fn b2h(b: u8) -> u8 {
    return match b {
        0...9 => ('0' as u8) + b,
        10...15 => ('a' as u8) + (b - 10),
        _ => panic!("{} is out of hexadecimal range.", b),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64() {
        let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let bytes = htob(hex);
        let base64 = btoa(&bytes);
        assert_eq!(
            base64.as_slice(),
            b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".as_ref()
        );
    }

    #[test]
    fn fixed_xor_hex_output() {
        let input = htob(b"1c0111001f010100061a024b53535009181c");
        let code = htob(b"686974207468652062756c6c277320657965");
        let result = xor(&input, &code);
        assert_eq!(
            btoh(&result).as_slice(),
            b"746865206b696420646f6e277420706c6179".as_ref()
        );
    }

    #[test]
    fn solve_single_byte_xor_cipher() {
        let input = htob(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let key = solve_single_byte_xor(&input);
        assert_eq!(
            String::from_utf8(input.iter().map(|b| b ^ key).collect()).unwrap(),
            "Cooking MC's like a pound of bacon"
        );
        assert_eq!(key, 88);
    }

    #[test]
    fn score_english_higher_than_noise() {
        let english = b"It is a truth universally acknowledged...";
        let noise = b"asdf0(JD)F(ajsdfasd0fjsd asdf ;lj23r ASDFoi";
        assert_eq!(english_score(english) > english_score(noise), true);
    }

    #[test]
    fn repeating_key_xor_cipher() {
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let cipher = b"ICE";
        let encrypted = repeating_xor(input, cipher);
        assert_eq!(
            btoh(&encrypted).as_slice(),
            b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".as_ref());
    }

    #[test]
    fn hamming_distance_between_strings() {
        let str1 = b"this is a test";
        let str2 = b"wokka wokka!!!";
        assert_eq!(hamming_distance(str1, str2), 37);
    }
}
