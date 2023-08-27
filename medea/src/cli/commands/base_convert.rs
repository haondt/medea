
/*
Rules:
- if there is one input, we will convert a single number.
    - if the input format is ascii, we will consider multiple
        characters as multiple inputs
- if there is multiple inputs, we will consider it a list of bytes.
    therefore, each input must be 0 - 255
    - for binary, this means no more than 8 bits
    - for hex, this means no more than 2 characters
    - for b64, we will just allow for space-delimited 3 byte segments
    - for ascii, characters do not need to be space-delimited
    - for decimal, this means 0 - 255
- for practical purposes, the data fed into an ascii output (either as a single
    number or as multiple bytes) must fit into the reasonable range of characters,
    i.e. 32 - 126
- hex input does not care about casing
*/

use clap::{Parser, ValueEnum};
use indoc::indoc;

use crate::cli::args::{Runnable, BaseArgs};

#[derive(Parser, Debug, Clone)]
#[command(
    about = "Convert numbers between different bases",
    after_help = "See `medea help base-convert` for details",
    long_about = indoc!{"
        Convert the input number to another number base. Input may be supplied
        as a single number, or as a space-delimited list of bytes.

        For ascii input, the bytes should not be space-delimited.
        For ascii output, input values must be between 32 - 126.
    "},
    after_long_help = indoc!{r#"
        Examples:
            TODO!
    "#}
)]
pub struct BaseConvertArgs {
    #[arg(
        num_args(0..),
        help = "Number or bytes to be converted",
        long_help = indoc!{"
            Number or bytes to be converted. Can be supplied as
            a single value, or a space-delimited list of bytes.
        "}
    )]
    input: Vec<String>,

    #[arg(
        short,
        long,
        help = "Input format",
        value_enum,
        value_name = "FORMAT"
    )]
    from: Format,

    #[arg(
        short,
        long,
        help = "Output format",
        value_enum,
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
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    Bin,
    Hex,
    B64,
    Ascii,
    Dec
}

trait Converter {
    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8>;
    fn to_string(&self, bytes: &Vec<u8>) -> Vec<String>;
}

struct AsciiConverter;
impl Converter for AsciiConverter {
    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8> {
        bytes.concat().chars().map(|c| c as u8).collect()
    }

    fn to_string(&self, bytes: &Vec<u8>) -> Vec<String> {
        let s = bytes.iter().map(|&b| b as char).collect();
        vec![s]
    }
}

struct TodoConverter;
impl Converter for TodoConverter {
    fn to_bytes(&self, _bytes: &Vec<String>) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self, _bytes: &Vec<u8>) -> Vec<String> {
        todo!()
    }
}

impl BaseConvertArgs {
    fn select_converter(&self, format: &Format) -> Box<dyn Converter> {
        match format {
            Format::Ascii => Box::new(AsciiConverter),
            _ => Box::new(TodoConverter)
        }
    }
}

impl Runnable for BaseConvertArgs {
    fn run(&self, _: &BaseArgs, _:impl Fn() -> String) -> Result<String,Box<dyn std::error::Error>> {
        let from_converter = self.select_converter(&self.from);
        let to_converter = self.select_converter(&self.to);

        let bytes = from_converter.to_bytes(&self.input);
        let result = to_converter.to_string(&bytes);

        println!("bytes: {:?}", bytes);
        return Ok(format!("result: {:?}", result));
    }
}
