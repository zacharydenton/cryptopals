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
