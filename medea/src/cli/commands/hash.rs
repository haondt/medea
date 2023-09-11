use std::error::Error;

use crate::cli::utils::hash_utils::Algorithm;

use super::super::{BaseArgs, Runnable};
use base64ct::{Base64, Encoding};
use clap::{Parser, ValueEnum};
use digest::OutputSizeUser;
use hmac::{Hmac, Mac};
use sha1::Sha1;

use indoc::indoc;
use md5::{digest::DynDigest, Md5};
use sha2::{Sha256, Sha512};

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;
type HmacSha1 = Hmac<Sha1>;
type HmacMd5 = Hmac<Md5>;

#[derive(Parser, Debug, Clone)]
#[command(
    about = "Generate cryptographic hashes",
    after_help = "See `medea help hash` for details",
    long_about = indoc!{"
        Read data and generate a hash value, optionally using
        an hmac key.
    "},
    after_long_help = indoc!{r#"
        Examples:
            # generate an md5 hash
            $ medea hash "this is some data"
            1463f25d10e363181d686d2484a9eab6

            # generate a sha256 hash using file contents
            $ medea hash "$(cat data.txt)" --hmac "$(cat secret.txt)" -ua sha256
            147933218AAABC0B8B10A2B3A5C34684C8D94341BCF10A4736DC7270F7741851
    "#}
)]
pub struct HashArgs {
    #[arg(help = "Data to be hashed")]
    data: String,

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
        help = "Hashing algorithm",
        value_enum,
        default_value = "md5"
    )]
    algorithm: Algorithm,

    #[arg(long, help = "Key for generating hmac hashes")]
    hmac: Option<String>,

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
    Hex,
    B64,
}

trait DynHmacDigest {
    fn update(&mut self, data: &[u8]);
    fn finalize_into_bytes(&mut self) -> Vec<u8>;
}

impl<T: Mac + OutputSizeUser + Clone> DynHmacDigest for T {
    fn update(&mut self, data: &[u8]) {
        self.update(data)
    }

    fn finalize_into_bytes(&mut self) -> Vec<u8> {
        self.clone().finalize().into_bytes().to_vec().to_owned()
    }
}

impl Runnable for HashArgs {
    fn run(&self, _: &BaseArgs, _: impl Fn() -> String) -> Result<String, Box<dyn Error>> {
        let data = self.data.as_bytes();
        let res: Vec<u8>;

        if self.hmac.is_some() {
            let key = self.hmac.clone().unwrap();
            let mut alg = match &self.algorithm {
                Algorithm::MD5 => {
                    Box::new(HmacMd5::new_from_slice(key.as_bytes())?) as Box<dyn DynHmacDigest>
                }
                Algorithm::SHA1 => Box::new(HmacSha1::new_from_slice(key.as_bytes())?),
                Algorithm::SHA256 => Box::new(HmacSha256::new_from_slice(key.as_bytes())?),
                Algorithm::SHA512 => Box::new(HmacSha512::new_from_slice(key.as_bytes())?),
            };

            alg.update(&data);
            res = alg.finalize_into_bytes();
        } else {
            let mut alg = match &self.algorithm {
                Algorithm::MD5 => Box::new(Md5::default()) as Box<dyn DynDigest>,
                Algorithm::SHA1 => Box::new(Sha1::default()),
                Algorithm::SHA512 => Box::new(Sha512::default()),
                Algorithm::SHA256 => Box::new(Sha256::default()),
            };

            alg.update(data);
            res = alg.finalize().to_vec().to_owned();
        }

        let hash = match self.to {
            Format::B64 => Base64::encode_string(&res),
            Format::Hex => match self.upper {
                true => base16ct::upper::encode_string(&res),
                false => base16ct::lower::encode_string(&res),
            },
        };

        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{
        args::{BaseArgs, Runnable},
        ArgsEnum,
    };

    use super::{Algorithm, Format, HashArgs};

    fn base_args(a: HashArgs) -> BaseArgs {
        BaseArgs {
            trim: false,
            command: ArgsEnum::Hash(a),
        }
    }

    fn spoof_input(input: String) -> Box<dyn Fn() -> String> {
        return Box::new(move || -> String { return input.clone() });
    }

    #[test]
    fn will_create_base_64_hash() {
        let sut = HashArgs {
            algorithm: Algorithm::MD5,
            to: Format::B64,
            data: String::from("foo"),
            hmac: None,
            upper: false,
        };

        let hash = sut
            .run(&base_args(sut.clone()), spoof_input(String::new()))
            .unwrap();
        assert_eq!(hash, "rL0Y20zC+Fzt72VPzMSk2A==");
    }

    #[test]
    fn will_create_uppercase_hex_hmac_hash() {
        let sut = HashArgs {
            algorithm: Algorithm::SHA256,
            to: Format::Hex,
            data: String::from("foo"),
            hmac: Some(String::from("bar")),
            upper: true,
        };

        let hash = sut
            .run(&base_args(sut.clone()), spoof_input(String::new()))
            .unwrap();
        assert_eq!(
            hash,
            "147933218AAABC0B8B10A2B3A5C34684C8D94341BCF10A4736DC7270F7741851"
        );
    }
}
