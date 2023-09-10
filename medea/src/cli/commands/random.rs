use crate::cli::utils::{base64_utils, hex_utils};

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
            Format::Hex => hex_utils::encode(&random_bytes, self.upper),
        };
        Ok(output)
    }
}
