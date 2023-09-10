pub fn encode(bytes: &[u8], uppercase: bool) -> String {
    let characters = match uppercase {
        true => "0123456789ABCDEF",
        false => "0123456789abcdef",
    };

    let mut result = String::with_capacity(bytes.len() * 2);

    for i in 0..(bytes.len()) {
        // mask off either bits
        let first = (bytes[i] >> 4) & 0xF;
        let second = bytes[i] & 0xF;
        result.push(characters.chars().nth(first as usize).unwrap());
        result.push(characters.chars().nth(second as usize).unwrap());
    }

    result
}

pub fn decode(input: String) -> Vec<u8> {
    if input.len() == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut buffer = 0u8;
    let mut buffer_length = 0;

    for &byte in input.as_bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A' + 10,
            b'a'..=b'z' => byte - b'a' + 10,
            b'0'..=b'9' => byte - b'0',
            _ => panic!("invalid character in hex string"),
        };

        buffer = (buffer << 4) | (value as u8);
        buffer_length += 4;
        if buffer_length >= 8 {
            result.push((buffer >> (buffer_length -8)) as u8);
            buffer_length -= 8;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;


    #[test]
    fn will_convert_uppercase_hex_bytes_correctly() {
        let bytes = [255, 128, 0];
        let result = super::encode(&bytes, true);
        assert_eq!(result, "FF8000");
    }

    #[test]
    fn will_convert_lowercase_hex_bytes_correctly() {
        let bytes = [254];
        let result = super::encode(&bytes, false);
        assert_eq!(result, "fe");
    }

    #[rstest(input, expected_result,
        case("DEADBEEF", &[222, 173, 190, 239]),
        case("AA", &[170]),
        case("00", &[0]),
        case("ABC123", &[171, 193, 35]),
    )]
    fn will_decode_hex_bytes_correctly(input: String, expected_result: &[u8]) {
        let result = super::decode(input);
        assert_eq!(result, expected_result);
    }
}
