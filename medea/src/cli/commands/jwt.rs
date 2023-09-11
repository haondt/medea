use clap::{Parser, ValueEnum};
use colored::Colorize;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

use crate::cli::{args::{Runnable, BaseArgs}, utils::{base64_utils, ascii_utils, hash_utils::DynHmacDigest}};

use std::error::Error;

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;
type HmacSha1 = Hmac<Sha1>;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct JwtArgs {
    #[arg(help = "Token to be decoded, or payload to be encoded")]
    input: String,

    #[arg(long, group("mode"), default_value = "false")]
    decode: bool,

    #[arg(long, group("mode"), default_value = "false")]
    encode: bool,

    #[arg(short = 'k', long, help = "Signing key")]
    signing_key: Option<String>,

    #[arg(short, long, help = "Encoding format of signing key", default_value = "ascii")]
    from: KeyFormat,

    #[arg(short, long, default_value = "hs256")]
    algorithm: HmacAlgorithm,
}

pub struct Jwt {
    header: Value,
    payload: Value,
    signature: String
}

#[derive(ValueEnum, Debug, Clone)]
pub enum HmacAlgorithm {
    HS1,
    HS256,
    HS512,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum KeyFormat {
    B64,
    Ascii,
}

impl JwtArgs {
    fn decode_part(part: &str) -> Result<Value, Box<dyn Error>> {
        Ok(Self::try_parse_json(&(&ascii_utils::encode(&base64_utils::decode_url(&part))))?)
    }

    fn try_parse_json(s: &str) -> Result<Value, Box<dyn Error>> {
        Ok(serde_json::from_str::<Value>(&s).map_err(|_| format!("unable to parse json value: `{}`", s))?)
    }

    fn decode_jwt(jwt: &str) -> Result<Jwt, Box<dyn Error>> {
        let parts: Vec<String> = jwt.split('.').map(|s| s.to_string()).collect();
        if parts.len() != 3 {
            return Err("token does not contain the correct number of parts".into());
        }

        let jwt = Jwt {
            header: Self::decode_part(&parts[0])?,
            payload: Self::decode_part(&parts[1])?,
            signature: parts[2].to_string()
        };

        Ok(jwt)
    }

    fn get_alg(jwt_header: &Value) -> Result<HmacAlgorithm, Box<dyn Error>> {
        match jwt_header["alg"].as_str() {
            Some(s) =>  match s {
                "HS1" => Ok(HmacAlgorithm::HS1) as Result<HmacAlgorithm, Box<dyn Error>>,
                "HS256" => Ok(HmacAlgorithm::HS256),
                "HS512" => Ok(HmacAlgorithm::HS512),
                _ => Err(format!("unexpected algorithm: {:?}", s).into())
            },
            _ => Err("missing algorithm".into())
        }
    }

    fn generate_signature(header: &str, payload: &str, signing_key: &[u8], alg: &HmacAlgorithm) -> Result<String, Box<dyn Error>> {
        let mut digest: Box<dyn DynHmacDigest> = match alg {
            HmacAlgorithm::HS1 => Box::new(HmacSha1::new_from_slice(&signing_key)?),
            HmacAlgorithm::HS256 => Box::new(HmacSha256::new_from_slice(&signing_key)?),
            HmacAlgorithm::HS512 => Box::new(HmacSha512::new_from_slice(&signing_key)?),
        };

        let data = String::new() + &header + &"." + &payload;
        digest.update(&ascii_utils::decode(&data));
        let signature = base64_utils::encode_url(&digest.finalize_into_bytes());
        Ok(signature)
    }

    fn validate_structure(jwt: &Jwt) -> Result<HmacAlgorithm, Box<dyn Error>> {
        if let Some(typ) = jwt.header["typ"].as_str() {
            if typ != "JWT" && typ != "jwt" {
                return Err(format!("unexpected type: {:?}", typ).into());
            }
        } else {
            return Err("missing type".into());
        }

        let alg = Self::get_alg(&jwt.header)?;

        Ok(alg)
    }

    fn serialize_jwt(jwt: &Jwt, signature_status: &str) -> String {
        let mut result = String::new();
        result += &"header:".bold().to_string();
        result += &"\n";
        result += &serde_json::to_string_pretty(&jwt.header).unwrap();
        result += &"\n\n";

        result += &"payload:".bold().to_string();
        result += &"\n";
        result += &serde_json::to_string_pretty(&jwt.payload).unwrap();
        result += &"\n\n";

        result += &"signature:".bold().to_string();
        result += &"\n";
        result += signature_status;

        result
    }

    fn decode_input(&self) -> Result<String, Box<dyn Error>> {
        let jwt = Self::decode_jwt(&self.input)?;
        let alg = Self::validate_structure(&jwt)?;
        let signature_status = match &self.signing_key {
            Some(k) => {
                let signing_bytes = match self.from {
                    KeyFormat::B64 => base64_utils::decode(k),
                    KeyFormat::Ascii => ascii_utils::decode(k)
                };

                let parts: Vec<String> = self.input.split('.').map(|s| s.to_string()).collect();
                let signature = Self::generate_signature(&parts[0], &parts[1], &signing_bytes, &alg)?;

                match signature == jwt.signature {
                    true => String::from("signature is valid"),
                    false => String::from("signature is not valid")
                }
            },
            _ => String::from("signature not validated (no signing key provided)")
        };

        Ok(Self::serialize_jwt(&jwt, &signature_status))
    }

    fn encode_input(&self) -> Result<String, Box<dyn Error>> {
        if self.signing_key.is_none() {
            return Err("signing key required to create jwt".into());
        }

        let alg_str =  match self.algorithm {
            HmacAlgorithm::HS1 => "HS1",
            HmacAlgorithm::HS256 => "HS256",
            HmacAlgorithm::HS512 => "HS512",
        };
        let header_json = Self::try_parse_json(&format!("{{\"alg\":\"{}\",\"typ\":\"JWT\"}}", &alg_str))?;
        let minified_header = serde_json::to_string(&header_json)?;
        let minified_payload = serde_json::to_string(&Self::try_parse_json(&self.input.clone())?)?;
        let header = base64_utils::encode_url(&ascii_utils::decode(&minified_header));
        let payload = base64_utils::encode_url(&ascii_utils::decode(&minified_payload));

        let alg = Self::get_alg(&header_json)?;
        let signing_bytes = match self.from {
            KeyFormat::B64 => base64_utils::decode(&self.signing_key.clone().unwrap()),
            KeyFormat::Ascii => ascii_utils::decode(&self.signing_key.clone().unwrap())
        };
        let signature = Self::generate_signature(&header, &payload, &signing_bytes, &alg)?;

        Ok(format!("{}.{}.{}", header, payload, signature))
    }
}
impl Runnable for JwtArgs {
    fn run(&self, _: &BaseArgs, _:impl Fn() -> String) -> Result<String,Box<dyn Error>> {
        match self.encode {
            true => self.encode_input(),
            false => self.decode_input()
        }
    }
}