
use super::super::{BaseArgs, Runnable};
use clap::{Parser, ValueEnum};

use indoc::indoc;
use rand::Rng;

use std::cmp::min;

#[derive(Parser, Debug, Clone)]
#[command(
    about = "Generate random bytes of data",
    after_help = "See `medea help timestamp` for details",
    after_long_help = indoc!{r#"
        Examples:
            # generate 32 random bytes and output as uppercase hex string
            medea rnd -f hex -u 32

            # generate 128 random bytes and output as base64 string
            medea rnd -f b64 128

    "#}
)]

pub struct RandomArgs {
    #[arg(short, long, help = "Output format", value_enum, default_value = "hex")]
    format: Format,

    #[arg(short, long, help = "Use upper case characters for hex output", default_value = "false")]
    upper: bool,

    #[arg(help = "Number of bytes to generate")]
    count_bytes: u32,
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    Hex,
    B64,
}

impl RandomArgs {
    fn convert_to_base_64(bytes: &[u8]) -> String {
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        // pad to a multiple of 3
        let mut string_capacity = bytes.len();
        if bytes.len() % 3 > 0 { string_capacity += 3 - bytes.len() % 3; }

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
            for j in 0..(4-num_padded_characters) {
                // shift the corresponding 6-bit slice into the rightmost position
                let shifted = value >> ((3 - j) * 6);
                // mask off those 6 bits
                let six_bits = shifted & mask;
                // grab the matching character
                result.push(characters.chars().nth(six_bits as usize).unwrap());
            }

            // fill any leftover spaces in the 4-character string with padding
            for _ in 0..num_padded_characters {
                result.push('=');
            }


            i += 3;
        }

        result
    }

    fn convert_to_hex(bytes: &[u8], uppercase: bool) -> String {
        let characters = match uppercase {
            true => "0123456789ABCDEF",
            false => "0123456789abcdef"
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
}



impl Runnable for RandomArgs {
    fn run(
        &self,
        _: &BaseArgs,
        _: impl Fn() -> String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..self.count_bytes).map(|_| rng.gen()).collect();
        let output = match self.format {
            Format::B64 => Self::convert_to_base_64(&random_bytes),
            Format::Hex => Self::convert_to_hex(&random_bytes, self.upper)
        };
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::RandomArgs;

    #[test]
    fn will_pad_b64_bytes_correctly() {
        let bytes = [255, 255];
        let result = RandomArgs::convert_to_base_64(&bytes);
        assert_eq!(result, "//8=");
    }

    #[test]
    fn will_convert_b64_bytes_correctly() {
        let bytes = [255, 255, 0];
        let result = RandomArgs::convert_to_base_64(&bytes);
        assert_eq!(result, "//8A");
    }

    #[test]
    fn will_convert_uppercase_hex_bytes_correctly() {
        let bytes = [255, 128, 0];
        let result = RandomArgs::convert_to_hex(&bytes, true);
        assert_eq!(result, "FF8000");
    }

    #[test]
    fn will_convert_lowercase_hex_bytes_correctly() {
        let bytes = [254];
        let result = RandomArgs::convert_to_hex(&bytes, false);
        assert_eq!(result, "fe");
    }
}
