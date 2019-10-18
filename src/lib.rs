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
    let keys_by_score: Vec<(f32, u8)> = ((std::u8::MIN)..(std::u8::MAX))
        .map(|key| {
            for i in 0..bytes.len() {
                decrypted[i] = bytes[i] ^ key;
            }
            (english_score(&decrypted), key)
        })
        .collect();
    let &(_, key) = keys_by_score
        .iter()
        .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
        .unwrap();
    key
}

pub fn hamming_distance(vec1: &[u8], vec2: &[u8]) -> u32 {
    let mut distance = 0;
    let difference = xor(vec1, vec2);
    for byte in difference {
        distance += byte.count_ones();
    }
    distance
}

/// Decodes a string of data which has been encoded using base-64 encoding.
pub fn atob(base64: &[u8]) -> Vec<u8> {
    if base64.len() % 4 != 0 {
        panic!("Invalid base-64 string (length not a multiple of 4).");
    }

    let mut bytes: Vec<u8> = vec![];
    let mut buffer: u32 = 0;
    let mut index = 0;

    for &c in base64 {
        if c == ('=' as u8) {
            // Padding (should only appear at the end).
            break;
        } else if c == ('\n' as u8) {
            // Ignore newlines.
            continue;
        } else {
            buffer = (buffer << 6) | (a2b(c) as u32);
            index = index + 1;
            if index == 4 {
                bytes.push(((buffer >> 16) & 0xFF) as u8);
                bytes.push(((buffer >> 8) & 0xFF) as u8);
                bytes.push((buffer & 0xFF) as u8);
                buffer = 0;
                index = 0;
            }
        }
    }

    // Process padding, if any.
    if index == 3 {
        // 2 padding chars.
        bytes.push(((buffer >> 10) & 0xFF) as u8);
        bytes.push(((buffer >> 2) & 0xFF) as u8);
    } else if index == 2 {
        // 1 padding char.
        bytes.push(((buffer >> 4) & 0xFF) as u8);
    }

    bytes
}

/// Creates a base-64 encoded ASCII string from a binary string.
pub fn btoa(bytes: &[u8]) -> Vec<u8> {
    let mut base64: Vec<u8> = vec![];

    for i in 0..(bytes.len() * 8 / 6) {
        if i > 0 && i % 76 == 0 {
            // Wrap to 76 chars per line.
            base64.push('\n' as u8);
        }

        let mut value: u8 = 0;
        for j in 0..6 {
            let byte_index = (i * 6 + j) / 8;
            let bit_index = (i * 6 + j) % 8;
            let shift = 7 - bit_index;
            let bit = (bytes[byte_index] >> shift) & 1;
            value |= bit << (5 - j);
        }
        base64.push(b2a(value));
    }

    base64
}

fn a2b(a: u8) -> u8 {
    match a as char {
        'A'...'Z' => a - ('A' as u8),
        'a'...'z' => a - ('a' as u8) + 26,
        '0'...'9' => a - ('0' as u8) + 52,
        '+' => 62,
        '/' => 63,
        _ => panic!("{} is out of base64 range.", a),
    }
}

fn b2a(b: u8) -> u8 {
    match b {
        0...25 => ('A' as u8) + b,
        26...51 => ('a' as u8) + (b - 26),
        52...61 => ('0' as u8) + (b - 52),
        62 => ('+' as u8),
        63 => ('/' as u8),
        _ => panic!("{} is out of base64 range.", b),
    }
}

pub fn english_score(bytes: &[u8]) -> f32 {
    let mut score: f32 = 0.0;
    for byte in bytes {
        score += match *byte as char {
            ' ' => 15.0,
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
            '.' => 3.0,
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
            _ => -15.0,
        };
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
    fn base64_encode() {
        let input = b"Governments are instituted among Men, deriving their just powers from the consent of the governed,--That whenever any Form of Government becomes destructive of these ends, it is the Right of the People to alter or to abolish it, and to institute new Government, laying its foundation on such principles and organizing its powers in such form, as to them shall seem most likely to effect their Safety and Happiness.";
        let encoded = btoa(input);
        assert_eq!(
            encoded.as_slice(),
            b"R292ZXJubWVudHMgYXJlIGluc3RpdHV0ZWQgYW1vbmcgTWVuLCBkZXJpdmluZyB0aGVpciBqdXN0
IHBvd2VycyBmcm9tIHRoZSBjb25zZW50IG9mIHRoZSBnb3Zlcm5lZCwtLVRoYXQgd2hlbmV2ZXIg
YW55IEZvcm0gb2YgR292ZXJubWVudCBiZWNvbWVzIGRlc3RydWN0aXZlIG9mIHRoZXNlIGVuZHMs
IGl0IGlzIHRoZSBSaWdodCBvZiB0aGUgUGVvcGxlIHRvIGFsdGVyIG9yIHRvIGFib2xpc2ggaXQs
IGFuZCB0byBpbnN0aXR1dGUgbmV3IEdvdmVybm1lbnQsIGxheWluZyBpdHMgZm91bmRhdGlvbiBv
biBzdWNoIHByaW5jaXBsZXMgYW5kIG9yZ2FuaXppbmcgaXRzIHBvd2VycyBpbiBzdWNoIGZvcm0s
IGFzIHRvIHRoZW0gc2hhbGwgc2VlbSBtb3N0IGxpa2VseSB0byBlZmZlY3QgdGhlaXIgU2FmZXR5
IGFuZCBIYXBwaW5lc3Mu"
                .as_ref()
        );
    }

    #[test]
    fn base64_decode() {
        let base64 = b"TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz
IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg
dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu
dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo
ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";
        let decoded = atob(base64);
        assert_eq!(String::from_utf8(decoded).unwrap(), "Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.");
    }

    #[test]
    fn hamming_distance_between_strings() {
        let str1 = b"this is a test";
        let str2 = b"wokka wokka!!!";
        assert_eq!(hamming_distance(str1, str2), 37);
    }
}
