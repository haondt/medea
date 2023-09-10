use std::cmp::min;

fn encode_internal(bytes: &[u8], alphabet: &str, pad: bool) -> String {
    // pad to a multiple of 3
    let mut string_capacity = bytes.len();
    if bytes.len() % 3 > 0 {
        string_capacity += 3 - bytes.len() % 3;
    }

    // multiply by 4/3, since each set of 3 bytes can fit into 4 b64 characters
    string_capacity /= 3;
    string_capacity *= 4;

    let mut result = String::with_capacity(string_capacity);

    let mut i = 0;
    while i < bytes.len() {
        // slice off up to 3 bytes
        let input_chunk = &bytes[i..min(i + 3, bytes.len())];
        let mut value = 0u32;

        // push up to 3 bytes into value
        for j in 0..3 {
            value <<= 8;
            if j < input_chunk.len() {
                value |= u32::from(input_chunk[j]);
            }
        }

        // convert those bytes into characters
        let num_padded_characters = 3 - input_chunk.len();
        let mask = 0x3F;
        for j in 0..(4 - num_padded_characters) {
            // shift the corresponding 6-bit slice into the rightmost position
            let shifted = value >> ((3 - j) * 6);
            // mask off those 6 bits
            let six_bits = shifted & mask;
            // grab the matching character
            result.push(alphabet.chars().nth(six_bits as usize).unwrap());
        }

        // fill any leftover spaces in the 4-character string with padding
        if pad {
            for _ in 0..num_padded_characters {
                result.push('=');
            }
        }

        i += 3;
    }

    result

}

pub fn encode(bytes: &[u8]) -> String {
    encode_internal(bytes, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", true)
}

pub fn encode_url(bytes: &[u8]) -> String {
    encode_internal(bytes, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", false)
}

fn decode_internal<F>(input: &str, byte_map: F) -> Vec<u8>
where
    F: Fn(u8) -> u8
{
    if input.len() == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut buffer = 0u16; // 2 byte buffer
    let mut buffer_length = 0;

    for &byte in input.as_bytes() {
        let value = byte_map(byte);
        if byte == b'=' {
            continue;
        }

        buffer = (buffer << 6) | (value as u16);
        buffer_length += 6;
        if buffer_length >= 8 {
            result.push((buffer >> (buffer_length -8)) as u8);
            buffer_length -= 8;
        }

    }

    result

}

pub fn decode(input: String) -> Vec<u8> {
    decode_internal(input.as_str(),
        |byte| match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => 255, // Ignore padding characters
            _ => panic!("Invalid character in Base64 string"),
        }
    )
}

pub fn decode_url(input: String) -> Vec<u8> {
    decode_internal(input.as_str(),
        |byte| match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => panic!("Invalid character in Base64 string"),
        }
    )
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::cli::utils::base64_utils;

    #[test]
    fn will_pad_b64_bytes_correctly() {
        let bytes = [255, 255];
        let result = base64_utils::encode(&bytes);
        assert_eq!(result, "//8=");
    }

    #[test]
    fn will_pad_b64_url_bytes_correctly() {
        let bytes = [255, 255];
        let result = base64_utils::encode_url(&bytes);
        assert_eq!(result, "__8");
    }

    #[test]
    fn will_encode_b64_bytes_correctly() {
        let bytes = [255, 255, 0];
        let result = base64_utils::encode(&bytes);
        assert_eq!(result, "//8A");
    }

    #[rstest(input, expected_result,
        case("ZGU", &[100, 101]),
        case("ZGU=", &[100, 101]),
        case("Z=GU", &[100, 101]),
        case("ZGUA", &[100, 101, 0]),
        case("VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIDEzIGxhenkgZG9ncy4=", &[84, 104, 101, 32, 113, 117, 105, 99, 107, 32, 98, 114, 111, 119, 110, 32, 102, 111, 120, 32, 106, 117, 109, 112, 115, 32, 111, 118, 101, 114, 32, 49, 51, 32, 108, 97, 122, 121, 32, 100, 111, 103, 115, 46]),
    )]
    fn will_decode_b64_bytes_correctly(input: String, expected_result: &[u8]) {
        let result = base64_utils::decode(input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn will_decode_b64_url_bytes_correctly() {
        let input = String::from("__8");
        let expected_result = [255, 255];
        let result = base64_utils::decode_url(input);
        assert_eq!(result, expected_result);
    }
}