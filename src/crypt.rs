const ZERO: &str = "skibidi";
const ONE:  &str = "toilet";
const DELTA: u32 = 0x9E3779B9;

pub fn crypt(encrypting: bool, key: &str, input: &str) -> String {
    let key = key_from_str(&key);
    if encrypting {
        let input = pad(&input);
        let mut bytes = to_bytes(&input);
        encrypt(&mut bytes, &key);
        let binary = to_binary(&bytes).join("");
        skibidify(&binary)
    } else {
        let binary = unskibidify(&input)
            .chars()
            .collect::<Vec<char>>()
            .chunks(32)
            .map(|i| i.iter().collect())
            .collect();
        let mut bytes = from_binary(&binary);
        decrypt(&mut bytes, &key);
        from_bytes(&bytes).trim().to_string()
    }
}

fn skibidify(input: &str) -> String {
    input.replace("0", ZERO).replace("1", ONE)
}

fn unskibidify(input: &str) -> String {
    input.replace(ZERO, "0").replace(ONE, "1")
}

// TEA, a tiny encryption algorithm
// by David J. Wheeler & Roger M. Needham
fn encrypt(bytes: &mut Vec<u32>, key: &[u32;4]) {
    let mut sum: u32;
    for b in bytes.chunks_mut(2) {
        sum = 0;
        for _ in 0..32 {
            sum = sum.wrapping_add(DELTA);
            b[0] = b[0].wrapping_add((b[1] << 4).wrapping_add(key[0]) ^ b[1].wrapping_add(sum) ^ (b[1] >> 5).wrapping_add(key[1]));
            b[1] = b[1].wrapping_add((b[0] << 4).wrapping_add(key[2]) ^ b[0].wrapping_add(sum) ^ (b[0] >> 5).wrapping_add(key[3]));
        }
    }
}

fn decrypt(bytes: &mut Vec<u32>, key: &[u32;4]) {
    let mut sum: u32;
    for b in bytes.chunks_mut(2) {
        sum = 0xC6EF3720;
        for _ in 0..32 {
            b[1] = b[1].wrapping_sub((b[0] << 4).wrapping_add(key[2]) ^ b[0].wrapping_add(sum) ^ (b[0] >> 5).wrapping_add(key[3]));
            b[0] = b[0].wrapping_sub((b[1] << 4).wrapping_add(key[0]) ^ b[1].wrapping_add(sum) ^ (b[1] >> 5).wrapping_add(key[1]));
            sum = sum.wrapping_sub(DELTA);
        }
    }
}

fn to_binary(bytes: &Vec<u32>) -> Vec<String> {
    let mut binary: Vec<String> = Vec::new();
    for i in bytes {
        binary.push(format!("{:b}", i));
    }
    binary
}

fn from_binary(binary: &Vec<String>) -> Vec<u32> {
    let mut bytes: Vec<u32> = Vec::new();
    for i in binary {
        bytes.push(u32::from_str_radix(i, 2).expect("Couldn't convert from binary to bytes"));
    }
    bytes
}

fn to_bytes(bytesstr: &str) -> Vec<u32> {
    let bytes = bytesstr.to_string().into_bytes();
    let mut largebytes: Vec<u32> = Vec::new();
    for i in bytes.chunks(4) {
        largebytes.push(u32::from_le_bytes([
            i[0], i[1], i[2], i[3]
        ]));
    }
    largebytes
}

fn from_bytes(largebytes: &Vec<u32>) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    for i in largebytes {
        bytes.extend_from_slice(&i.to_le_bytes());
    }
    String::from_utf8(bytes).expect("Couldn't convert from bytes to a string")
}

fn pad(str: &str) -> String {
    let amount = (8 - str.len() % 8) % 8;
    format!("{}{}", &str, &" ".repeat(amount))
}

fn key_from_str(keystr: &str) -> [u32;4] {
    let mut key: [u32;4] = [0,0,0,0];
    let iter = keystr.split_whitespace();
    if iter.clone().count() != 4 {
        panic!("The key must consist of 4 unsigned 32-bit integers");
    }
    for (i, j) in iter.enumerate() {
        key[i] = j.parse().expect("Couldn't derive the key from the given string");
    }
    key
}
