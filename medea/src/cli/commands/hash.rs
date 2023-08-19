use std::error::Error;

use super::super::{BaseArgs, Runnable};
use base64ct::{Base64, Encoding};
use clap::{Parser, ValueEnum};
use digest::OutputSizeUser;
use hmac::{Hmac, Mac};
use sha1::Sha1;

use md5::{digest::DynDigest, Md5};
use sha2::{Sha256, Sha512};

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;
type HmacSha1 = Hmac<Sha1>;
type HmacMd5 = Hmac<Md5>;

#[derive(Parser, Debug)]
pub struct HashArgs {
    #[arg(short, long, value_enum, default_value = "hex")]
    format: Format,

    #[arg(short, long, value_enum, default_value = "md5")]
    algorithm: Algorithm,

    #[arg(long)]
    hmac: Option<String>,

    #[arg(short, long, default_value = "false")]
    upper: bool,
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    Hex,
    B64,
}

#[derive(ValueEnum, Debug, Clone)]
enum Algorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
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
    fn run(&self, _: &BaseArgs, get_input:impl Fn() -> String) -> Result<String,Box<dyn Error>> {
        let message = get_input();
        let data = message.as_bytes();
        let res: Vec<u8>;

        if self.hmac.is_some() {
            let key = self.hmac.clone().unwrap();
            let mut alg = match &self.algorithm {
                Algorithm::MD5 => Box::new(HmacMd5::new_from_slice(key.as_bytes())?) as Box<dyn DynHmacDigest>,
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

        let hash = match self.format {
            Format::B64 => Base64::encode_string(&res),
            Format::Hex => match self.upper {
                true => base16ct::upper::encode_string(&res),
                false => base16ct::lower::encode_string(&res),
            },
        };

        Ok(hash)
    }
}
