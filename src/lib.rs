pub fn htob(hex: Vec<u8>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for pair in hex.chunks(2) {
        let a = h2b(pair[0]);
        let b = h2b(pair[1]);
        bytes.push((a << 4) + b);
    }
    return bytes;
}

pub fn btoh(bytes: Vec<u8>) -> Vec<u8> {
    let mut hex: Vec<u8> = Vec::new();
    for byte in bytes {
        hex.push(b2h(byte >> 4));
        hex.push(b2h(byte & 0xf));
    }
    return hex;
}

pub fn xor(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for (a, b) in vec1.iter().zip(vec2.iter()) {
        result.push(a ^ b);
    }
    return result;
}

pub fn repeating_xor(bytes: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let repeating_key: Vec<u8> = key.iter().cloned().cycle().take(bytes.len()).collect::<Vec<_>>();
    return xor(bytes, repeating_key);
}

pub fn btoa(bytes: Vec<u8>) -> Vec<u8> {
    let mut base64: Vec<u8> = Vec::new();
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
            _ => panic!("{} is out of base64 range.", value)
        });
    }
    return base64;
}

pub fn english_score(bytes: Vec<u8>) -> f32 {
    let mut score = 0.0;
    for byte in bytes {
        score += match byte as char {
            ' '       => 15.0,
            'e' | 'E' => 12.7,
            't' | 'T' => 9.0,
            'a' | 'A' => 8.2,
            'o' | 'O' => 7.5,
            'i' | 'I' => 7.0,
            'n' | 'N' => 6.7,
            's' | 'S' => 6.3,
            'h' | 'H' => 6.0,
            'r' | 'R' => 6.0,
            'd' | 'D' => 4.2,
            'l' | 'L' => 4.0,
            '.'       => 3.0,
            'u' | 'U' => 2.7,
            'c' | 'C' => 2.7,
            'm' | 'M' => 2.4,
            'w' | 'W' => 2.4,
            'f' | 'F' => 2.2,
            'g' | 'G' => 2.0,
            'y' | 'Y' => 2.0,
            'p' | 'P' => 1.9,
            'b' | 'B' => 1.5,
            'v' | 'V' => 1.0,
            'k' | 'K' => 0.8,
            'j' | 'J' => 0.2,
            'x' | 'X' => 0.2,
            'q' | 'Q' => 0.1,
            'z' | 'Z' => 0.1,
            _ => -15.0
        }
    }
    return score;
}

fn h2b(h: u8) -> u8 {
    return match h as char {
        '0'...'9' => h - ('0' as u8),
        'a'...'f' => 10 + h - ('a' as u8),
        'A'...'F' => 10 + h - ('A' as u8),
        _ => panic!("{} is not a valid hexadecimal character.", h)
    };
}

fn b2h(b: u8) -> u8 {
    return match b {
        0...9 => ('0' as u8) + b,
        10...15 => ('a' as u8) + (b - 10),
        _ => panic!("{} is out of hexadecimal range.", b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".as_bytes().to_vec();
        let bytes = htob(hex);
        let base64 = btoa(bytes);
        assert_eq!(base64, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".as_bytes().to_vec());
    }

    #[test]
    fn fixed_xor_hex_output() {
        let input = htob("1c0111001f010100061a024b53535009181c".as_bytes().to_vec());
        let code = htob("686974207468652062756c6c277320657965".as_bytes().to_vec());
        let result = xor(input, code);
        assert_eq!(btoh(result), "746865206b696420646f6e277420706c6179".as_bytes().to_vec());
    }

    #[test]
    fn score_english_higher_than_noise() {
        let english = "It is a truth universally acknowledged...".as_bytes().to_vec();
        let noise = "asdf0(JD)F(ajsdfasd0fjsd asdf ;lj23r ASDFoi".as_bytes().to_vec();
        assert_eq!(english_score(english) > english_score(noise), true);
    }

    #[test]
    fn repeating_key_xor_cipher() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();
        let cipher = "ICE".as_bytes().to_vec();
        let encrypted = repeating_xor(input, cipher);
        assert_eq!(btoh(encrypted), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".as_bytes().to_vec());
    }
}
