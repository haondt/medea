use clap::{Parser, ValueEnum};
use colored::Colorize;
use serde_json::Value;

use crate::cli::{args::{Runnable, BaseArgs}, utils::{base64_utils, ascii_utils}};

use std::error::Error;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct JwtArgs {
    #[arg(help = "Token to be decoded, or payload to be encoded")]
    input: String,

    #[arg(long, group("mode"), default_value = "true")]
    decode: bool,

    #[arg(long, group("mode"), default_value = "false")]
    encode: bool,

    #[arg(short = 'k', long, help = "Signing key")]
    signing_key: Option<String>,

    #[arg(short, long, help = "Encoding format of signing key", default_value = "ascii")]
    from: KeyFormat,
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
    fn decode_part(&self, part: &str) -> Result<Value, Box<dyn Error>> {
        Ok(serde_json::from_str::<Value>(&ascii_utils::encode(&base64_utils::decode_url(&part)))?)
    }

    fn decode(&self, jwt: &str) -> Result<Jwt, Box<dyn Error>> {
        let parts: Vec<String> = jwt.split('.').map(|s| s.to_string()).collect();
        let jwt = Jwt {
            header: self.decode_part(&parts[0])?,
            payload: self.decode_part(&parts[1])?,
            signature: parts[2].to_string()
        };

        Ok(jwt)
    }

    fn get_alg(&self, jwt: &Jwt) -> Result<HmacAlgorithm, Box<dyn Error>> {
        match jwt.header["alg"].as_str() {
            Some(s) =>  match s {
                "HS1" => Ok(HmacAlgorithm::HS1) as Result<HmacAlgorithm, Box<dyn Error>>,
                "HS256" => Ok(HmacAlgorithm::HS256),
                "HS512" => Ok(HmacAlgorithm::HS512),
                _ => Err(format!("unexpected algorithm: {:?}", s).into())
            },
            _ => Err("missing algorithm".into())
        }
    }

    fn validate_structure(&self, jwt: &Jwt) -> Result<HmacAlgorithm, Box<dyn Error>> {
        if let Some(typ) = jwt.header["typ"].as_str() {
            if typ != "JWT" && typ != "jwt" {
                return Err(format!("unexpected type: {:?}", typ).into());
            }
        } else {
            return Err("missing type".into());
        }

        let alg = self.get_alg(&jwt)?;

        Ok(alg)
    }

    fn serialize_jwt(&self, jwt: &Jwt, signature_status: &str) -> String {
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
}
impl Runnable for JwtArgs {
    fn run(&self, _: &BaseArgs, _:impl Fn() -> String) -> Result<String,Box<dyn Error>> {
        let jwt = self.decode(&self.input)?;
        let alg = self.validate_structure(&jwt)?;
        let signature_status = match &self.signing_key {
            Some(s) => match self.from {
                KeyFormat::B64 => todo!(),
                KeyFormat::Ascii => todo!(),
            },
            _ => String::from("signature not validated (no signing key provided)")
        };
        let result = self.serialize_jwt(&jwt, &signature_status);

        Ok(result)
    }
}