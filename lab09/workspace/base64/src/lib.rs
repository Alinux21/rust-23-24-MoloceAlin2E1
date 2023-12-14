pub fn encode(input: &[u8]) -> String {
    let mut alphabet: [char; 65] = [' '; 65];

    alphabet[0] = 'A';
    alphabet[26] = 'a';
    alphabet[52] = '0';
    alphabet[62] = '+';
    alphabet[63] = '/';
    alphabet[64] = '=';

    for i in 1..26 {
        alphabet[i] = ((alphabet[i - 1] as u8) + 1) as char;
    }

    for i in 27..52 {
        alphabet[i] = ((alphabet[i - 1] as u8) + 1) as char;
    }

    for i in 53..62 {
        alphabet[i] = ((alphabet[i - 1] as u8) + 1) as char;
    }

    let mut result: String = String::new();

    let mut a: Vec<&u8> = Vec::new();
    for index in input {
        a.push(index);
    }

    while a.len() / 3 > 0 || a.len() == 3 {
        let combined: u32 = ((*a[0] as u32) << 2 * 8) | ((*a[1] as u32) << 8) | *a[2] as u32;

        let v1 = (combined >> (3 * 6)) & 0b111_111;
        let v2 = (combined >> (2 * 6)) & 0b111_111;
        let v3 = (combined >> (1 * 6)) & 0b111_111;
        let v4 = combined & 0b111_111;

        let c1 = alphabet[v1 as usize];
        let c2 = alphabet[v2 as usize];
        let c3 = alphabet[v3 as usize];
        let c4 = alphabet[v4 as usize];

        result.push(c1);
        result.push(c2);
        result.push(c3);
        result.push(c4);

        a.remove(0);
        a.remove(0);
        a.remove(0);
    }

    if a.len() == 2 {
        let combined: u32 = ((*a[0] as u32) << 2 * 8) | ((*a[1] as u32) << 8);

        let v1 = (combined >> (3 * 6)) & 0b111_111;
        let v2 = (combined >> (2 * 6)) & 0b111_111;
        let v3 = (combined >> (1 * 6)) & 0b111_111;

        let c1 = alphabet[v1 as usize];
        let c2 = alphabet[v2 as usize];
        let c3 = alphabet[v3 as usize];
        let c4 = alphabet[64];

        result.push(c1);
        result.push(c2);
        result.push(c3);
        result.push(c4);
    } else if a.len() == 1 {
        let combined: u32 = (*a[0] as u32) << 2 * 8;

        let v1 = (combined >> (3 * 6)) & 0b111_111;
        let v2 = (combined >> (2 * 6)) & 0b111_111;

        let c1 = alphabet[v1 as usize];
        let c2 = alphabet[v2 as usize];
        let c3 = alphabet[64];
        let c4 = alphabet[64];

        result.push(c1);
        result.push(c2);
        result.push(c3);
        result.push(c4);
    }

    result
}

/// Encodes text to Base64. If `input` length is
/// divisible by 3 the returned String will contain only characters
/// otherwise it will contain one or two `=` paddings.
///
/// How Base64 encoding works -> https://en.wikipedia.org/wiki/Base64
///
/// # Example
///
/// ```
/// println!("{:?}",encode("Hey")); //returns  the string "SGV5"
/// println!("{:?}",encode("Apple")); // "QXBwbGU="
/// println!("{:?}",encode("Woah")); // "V29haA=="
///


#[test]
fn check_encoding_no_padding() {
    let result = String::from(encode(b"Rust is cool"));
    assert_eq!(result, "UnVzdCBpcyBjb29s");
}
#[test]
fn check_encoding_half_padding() {
    let result = String::from(encode(b"I'm having fun"));
    assert_eq!(result, "SSdtIGhhdmluZyBmdW4=");
}
#[test]
fn check_encoding_full_padding() {
    let result = String::from(encode(b"I like ice cream"));
    assert_eq!(result, "SSBsaWtlIGljZSBjcmVhbQ==");
}
