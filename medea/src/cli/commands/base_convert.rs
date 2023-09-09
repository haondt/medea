
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

use std::error::Error;

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
        value_name = "FORMAT",
        default_value = "dec"
    )]
    from: Format,

    #[arg(
        short,
        long,
        help = "Output format",
        value_enum,
        value_name = "FORMAT",
        default_value = "dec"
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
impl BinConverter {
    fn to_byte(&self, input: &str) -> u8 {
        let mut byte: u8 = 0;
        for c in input.chars() {
            byte <<= 1;
            byte += match c {
                '1' => 1,
                _ => 0
            };
        }
        byte
    }
}
impl Converter for BinConverter {
    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8> {
        if bytes.len() == 1 {
            let padding = match bytes[0].len() % 8 != 0 {
                true => "0".repeat(8 - bytes[0].len() % 8),
                false => String::new()
            };
            let input = padding + &bytes[0];
            let mut output = Vec::new();
            let mut chunk_start = 0;
            let mut chunk_end = 8;

            while chunk_end < (input.len() + 1) {
                let chunk = &input[chunk_start..chunk_end];
                output.push(self.to_byte(&chunk));
                chunk_start = chunk_end;
                chunk_end += 8;
            }
            return output;
        }

        bytes.iter().map(|s| self.to_byte(s)).collect()
    }

    fn to_string(&self, bytes: &Vec<u8>, concat: bool) -> Vec<String> {
        let mut outputs = Vec::new();
        for byte in bytes {
            let mut byte_string = String::new();
            for i in 0..8 {
                let mask = 0x80 >> i;
                let masked_byte = byte & mask;
                let bit = masked_byte >> (7-i);
                byte_string.push(match bit {
                    1 => '1',
                    _ => '0'
                });
            }
            outputs.push(byte_string);
        }

        match concat {
            true => vec![outputs.join("")],
            false => outputs
        }
    }

    fn validate_string(&self, bytes: &Vec<String>) -> Result<(), Box<dyn Error>> {
        for s in bytes {
            if s.len() < 1 {
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

struct DecConverter;
impl DecConverter {
    fn divide_string_by_two(&self, input: &String) -> String {
        let mut result =  String::new();
        let mut remainder = 0u8;
        for c in input.chars() {
            let operand = c.to_digit(10).unwrap() as u8 + remainder * 10;
            let result_digit = operand / 2;
            result.push(char::from_digit(result_digit as u32, 10).unwrap());
            remainder = operand - result_digit * 2;
        }

        if result.len() > 1 && result.chars().nth(0).unwrap() == '0' {
            result = result.trim_start_matches('0').to_string();
        }

        return result;
    }

    fn decimal_string_to_binary_string(&self, input: &String) -> String {
        if input.chars().all(|c| c == '0') {
            return String::from("00000000");
        }

        let mut output = String::new();
        let mut working_value = input.trim_start_matches('0').to_string();
        while working_value != "0" {
            let last_digit = working_value.chars().last().unwrap();
            let last_digit = last_digit.to_digit(10).unwrap() as u8;
            output.insert(0, char::from_digit((last_digit % 2) as u32, 10).unwrap());
            working_value = self.divide_string_by_two(&working_value);
        }
        return output;
    }

    fn multiply_string_by_two(&self, input: &String) -> String {
        let mut result = String::new();
        let mut carry = 0u8;
        for c in input.chars().rev() {
            let operand = c.to_digit(10).unwrap() as u8 * 2 + carry;
            let result_digit = operand % 10;
            result.insert(0, char::from_digit(result_digit as u32, 10).unwrap());
            carry = operand / 10;
        }

        if carry != 0 {
            result.insert(0, '1');
        }

        return result;
    }

    fn add_one_to_string(&self, input: &String) -> String {
        let mut carry = 1;
        let mut result = String::new();
        for c in input.chars().rev() {
            if carry == 0 {
                result.insert(0, c);
                continue;
            }

            let digit = c.to_digit(10).unwrap() + 1;
            if digit > 9 {
                result.insert(0, '0');
            } else {
                carry = 0;
                result.insert(0, char::from_digit(digit, 10).unwrap());
            }
        }

        if carry == 1 {
            result.insert(0, '1');
        }

        return result;
    }

    fn binary_string_to_decimal_string(&self, input: &String) -> String {
        if input.chars().all(|c| c == '0') {
            return String::from("0");
        }

        let mut output = String::new();
        for c in input.trim_start_matches('0').chars() {
            output = self.multiply_string_by_two(&output);
            if c == '1' {
                output = self.add_one_to_string(&output);
            }
        }

        return output;
    }

}
impl Converter for DecConverter {
    fn validate_string(&self, bytes: &Vec<String>) -> Result<(), Box<dyn Error>> {
        if bytes.len() == 1 {
            for c in bytes[0].chars() {
                if !c.is_digit(10) {
                    return Err(format!("Unexpected character: {:?}. The number must be a valid unsigned integer.", c).into());
                }
            }
        } else {
            for s in bytes {
                s.parse::<u8>().map_err(|e| format!(
                    "{}: {}: each byte must be an unsigned 8-bit number",
                    s, e
                ))?;
            }
        }

        return Ok(());
    }

    fn validate_bytes(&self, _: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn to_bytes(&self, bytes: &Vec<String>) -> Vec<u8> {
        let bin_converter = BinConverter{};
        if bytes.len() == 1 {
            let bin_str = self.decimal_string_to_binary_string(bytes.first().unwrap());
            return bin_converter.to_bytes(&vec![bin_str]);
        }

        let bin_bytes: Vec<String> = bytes.iter().map(|s| self.decimal_string_to_binary_string(s)).collect();
        return bin_converter.to_bytes(&bin_bytes);
    }

    fn to_string(&self, bytes: &Vec<u8>, concat: bool) -> Vec<String> {
        let bin_converter = BinConverter{};
        let bin_str = bin_converter.to_string(bytes, concat);
        return bin_str.iter().map(|s| self.binary_string_to_decimal_string(s)).collect();
    }
}

struct TodoConverter;
impl Converter for TodoConverter {
    fn to_bytes(&self, _bytes: &Vec<String>) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self, _bytes: &Vec<u8>, _concat: bool) -> Vec<String> {
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
            Format::Dec => Box::new(DecConverter),
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
        case(vec![String::from("101101010101000011110110111110110111")], vec![11, 85, 15, 111, 183]),
    )]
    fn will_read_from_single_string(bytes: Vec<String>, expected_result: Vec<u8>) {
        let converter = BinConverter{};
        let result = converter.to_bytes(&bytes);
        assert_eq!(result, expected_result);
    }

    #[rstest(bytes, expected_result,
        case(vec![String::from("0"), String::from("1")], vec![0, 1]),
        case(vec![String::from("001100"), String::from("10110011")], vec![12, 179]),
        case(vec![String::from("10110101"), String::from("01010000"), String::from("11110110"), String::from("11111011"), String::from("0111")], vec![181, 80, 246, 251, 7]),
    )]
    fn will_read_from_multiple_string(bytes: Vec<String>, expected_result: Vec<u8>) {
        let converter = BinConverter{};
        let result = converter.to_bytes(&bytes);
        assert_eq!(result, expected_result);
    }

    #[rstest(bytes, expected_result,
        case(vec![0], vec![String::from("00000000")]),
        case(vec![12], vec![String::from("00001100")]),
        case(vec![255], vec![String::from("11111111")]),
    )]
    fn will_convert_from_single_byte(bytes: Vec<u8>, expected_result: Vec<String>) {
        let converter = BinConverter{};
        let result = converter.to_string(&bytes, true);
        assert_eq!(result, expected_result);
    }

    #[rstest(bytes, expected_result, concat,
        case(vec![255, 0], vec![String::from("1111111100000000")], true),
        case(vec![12, 30], vec![String::from("0000110000011110")], true),
        case(vec![11, 85, 15, 111, 183], vec![String::from("0000101101010101000011110110111110110111")], true),

        case(vec![0, 255], vec![String::from("00000000"), String::from("11111111")], false),
        case(vec![12, 30], vec![String::from("00001100"), String::from("00011110")], false),
        case(vec![11, 85, 15, 111, 183], vec![String::from("00001011"), String::from("01010101"), String::from("00001111"), String::from("01101111"), String::from("10110111")], false),
    )]
    fn will_convert_from_multiple_byte(bytes: Vec<u8>, expected_result: Vec<String>, concat: bool) {
        let converter = BinConverter{};
        let result = converter.to_string(&bytes, concat);
        assert_eq!(result, expected_result);
    }

    #[rstest(bytes,
        case(vec![String::from("1111111111111111111111111111111")]),
        case(vec![String::from("0"), String::from("11111101")]),
    )]
    fn will_accept_string(bytes: Vec<String>) {
        let converter = BinConverter{};
        let result = converter.validate_string(&bytes);
        assert!(result.is_ok());
    }

    #[rstest(bytes,
        case(vec![String::from("1111111111111111111111111111111"), String::from("0")]),
        case(vec![String::from("")]),
        case(vec![String::from("0b0")]),
    )]
    fn will_reject_string(bytes: Vec<String>) {
        let converter = BinConverter{};
        let result = converter.validate_string(&bytes);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod dec_convert_test {
    use rstest::rstest;

    use super::{DecConverter, Converter};

    #[rstest(bytes,
        case(vec![String::from("12312389123912")]),
        case(vec![String::from("123"), String::from("0")]),
    )]
    fn will_accept_string(bytes: Vec<String>) {
        let converter = DecConverter{};
        let result = converter.validate_string(&bytes);
        assert!(result.is_ok());
    }

    #[rstest(bytes,
        case(vec![String::from("1111111111111111111111111111111"), String::from("0")]),
        case(vec![String::from("-10")]),
        case(vec![String::from("0b0")]),
    )]
    fn will_reject_string(bytes: Vec<String>) {
        let converter = DecConverter{};
        let result = converter.validate_string(&bytes);
        assert!(result.is_err());
    }
}
