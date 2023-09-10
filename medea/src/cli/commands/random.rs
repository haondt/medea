use crate::cli::utils::base64_utils;

use super::super::{BaseArgs, Runnable};
use clap::{Parser, ValueEnum};

use indoc::indoc;
use rand::Rng;

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
    #[arg(
        short,
        long,
        help = "Output format",
        value_enum,
        default_value = "hex",
        value_name = "FORMAT"
    )]
    to: Format,

    #[arg(
        short,
        long,
        help = "Use upper case characters for hex output",
        default_value = "false"
    )]
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
    fn convert_to_hex(bytes: &[u8], uppercase: bool) -> String {
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
}

impl Runnable for RandomArgs {
    fn run(
        &self,
        _: &BaseArgs,
        _: impl Fn() -> String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..self.count_bytes).map(|_| rng.gen()).collect();
        let output = match self.to {
            Format::B64 => base64_utils::encode(&random_bytes),
            Format::Hex => Self::convert_to_hex(&random_bytes, self.upper),
        };
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::RandomArgs;

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
