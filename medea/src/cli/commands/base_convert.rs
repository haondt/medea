
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

use std::{error::Error, cmp::min};

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
    fn validate_string(&self, bytes: &Vec<String>) -> Result<(), Box<dyn Error>>;
    fn validate_bytes(&self, bytes: &Vec<u8>) -> Result<(), Box<dyn Error>>;
    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8>;
    fn to_string(&self, bytes: &Vec<u8>, concat: bool) -> Vec<String>;
}

struct AsciiConverter;
impl Converter for AsciiConverter {
    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8> {
        bytes.concat().chars().map(|c| c as u8).collect()
    }

    fn to_string(&self, bytes: &Vec<u8>, _: bool) -> Vec<String> {
       vec![bytes.iter().map(|&b| String::from(b as char)).collect()]
    }

    fn validate_string(&self, _bytes: &Vec<String>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn validate_bytes(&self, bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        for b in bytes {
            if b < &32 || b > &126 {
                return Err("Byte value(s) out of range for printable characters. Should be between 32 and 126.".into());
            }
        }
        return Ok(());
    }
}

struct BinConverter;
impl Converter for BinConverter {
    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8> {
        if bytes.len() == 1 {
            let chunk_size = min(bytes[0].len(), 8);
            let chunk = &bytes[0][0..chunk_size];
            let mut byte: u8 = 0;
            for c in chunk.chars() {
                byte <<= 1;
                byte += match c {
                    '1' => 1,
                    _ => 0
                };
            }
            return vec![byte];
        }

        todo!()

    }

    fn to_string(&self, bytes: &Vec<u8>, concat: bool) -> Vec<String> {
        todo!()
    }

    fn validate_string(&self, bytes: &Vec<String>) -> Result<(), Box<dyn Error>> {
        for s in bytes {
            if s.len() <= 1 {
                return Err("Must have at least 1 bit per byte".into());
            } else if s.len() > 8 && bytes.len() > 1 {
                return Err("Cannot have more than 8 bits per byte".into());
            }

            for c in s.chars() {
                if c != '1' && c != '0' {
                    return Err(format!("Unexpected bit: {:?}", c).into());
                }
            }
        }
        return Ok(());
    }

    fn validate_bytes(&self, _bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}

struct TodoConverter;
impl Converter for TodoConverter {
    fn to_bytes(&self, _bytes: &Vec<String>) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self, _bytes: &Vec<u8>, concat: bool) -> Vec<String> {
        todo!()
    }

    fn validate_string(&self, _bytes: &Vec<String>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn validate_bytes(&self, _bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl BaseConvertArgs {
    fn select_converter(&self, format: &Format) -> Box<dyn Converter> {
        match format {
            Format::Ascii => Box::new(AsciiConverter),
            Format::Bin => Box::new(BinConverter),
            _ => Box::new(TodoConverter)
        }
    }
}

impl Runnable for BaseConvertArgs {
    fn run(&self, _: &BaseArgs, _:impl Fn() -> String) -> Result<String,Box<dyn std::error::Error>> {
        let from_converter = self.select_converter(&self.from);
        let to_converter = self.select_converter(&self.to);

        from_converter.validate_string(&self.input)?;
        let bytes = from_converter.to_bytes(&self.input);
        to_converter.validate_bytes(&bytes)?;
        let mut result = to_converter.to_string(&bytes, self.input.len() == 1);

        if self.input.len() == 1 {
            result = vec![result.join("")]
        }

        println!("bytes: {:?}", bytes);
        return Ok(format!("result: {:?}", result));
    }
}


#[cfg(test)]
mod ascii_convert_tests {
    use rstest::rstest;

    use super::{AsciiConverter, Converter};

    #[test]
    fn will_read_from_single_string() {
        let bytes = vec![String::from("abc")];
        let expected_result = vec![97, 98, 99];
        let converter = AsciiConverter{};
        let result = converter.to_bytes(&bytes);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn will_read_from_multiple_string() {
        let bytes = vec![String::from("ab"), String::from("c")];
        let expected_result = vec![97, 98, 99];
        let converter = AsciiConverter{};
        let result = converter.to_bytes(&bytes);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn will_convert_from_single_byte() {
        let bytes = vec![99];
        let expected_result = vec![String::from("c")];
        let converter = AsciiConverter{};
        let result = converter.to_string(&bytes, true);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn will_convert_from_multiple_byte() {
        let bytes = vec![99, 100, 101];
        let expected_result = vec![String::from("cde")];
        let converter = AsciiConverter{};
        let result = converter.to_string(&bytes, true);
        assert_eq!(result, expected_result);
    }

    #[rstest(bytes,
        case(vec![99]),
        case(vec![99, 100]),
    )]
    fn will_accept_bytes(bytes: Vec<u8>) {
        let converter = AsciiConverter{};
        let result = converter.validate_bytes(&bytes);
        assert!(result.is_ok());
    }

    #[rstest(bytes,
        case(vec![99, 13, 100]),
        case(vec![150]),
    )]
    fn will_reject_bytes(bytes: Vec<u8>) {
        let converter = AsciiConverter{};
        let result = converter.validate_bytes(&bytes);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod bin_convert_test {
    use rstest::rstest;

    use super::{BinConverter, Converter};

    #[rstest(bytes, expected_result,
        case(vec![String::from("0")], vec![0]),
        case(vec![String::from("001100")], vec![12]),
        case(vec![String::from("10110011")], vec![179]),
        case(vec![String::from("101101010101000011110110111110110111")], vec![181, 80, 246, 251, 7]),
    )]
    fn will_read_from_single_string(bytes: Vec<String>, expected_result: Vec<u8>) {
        let converter = BinConverter{};
        let result = converter.to_bytes(&bytes);
        assert_eq!(result, expected_result);
    }
}