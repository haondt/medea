use std::error::Error;

use clap::Parser;
use uuid::Uuid;
use mac_address::get_mac_address;
use super::super::{Runnable, BaseArgs};

#[derive(Parser, Debug)]
pub struct UuidArgs {
    #[arg(short, long, default_value="1")]
    count: u32,

    #[arg(short, long, default_value="false")]
    upper: bool,

    #[arg(short, long, default_value="4")]
    version: String,

    #[arg(short, long, default_value="false")]
    no_hyphens: bool,
}

#[derive(Debug)]
enum UuidError {
    InvalidVersion,
    MacAddressError(mac_address::MacAddressError),
    MacAddressNotFound
}

impl From<mac_address::MacAddressError> for UuidError {
    fn from(err: mac_address::MacAddressError) -> Self {
        UuidError::MacAddressError(err)
    }
}

impl std::fmt::Display for UuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UuidError::InvalidVersion => write!(f, "Invalid UUID version. Supported versions are '1' and '4'"),
            UuidError::MacAddressError(_) => write!(f, "Error retrieving system MAC address"),
            UuidError::MacAddressNotFound => write!(f, "Error retrieving system MAC address"),
        }
    }
}

impl std::error::Error for UuidError {}


impl UuidArgs {
    fn get_uuid(&self) -> Result<Uuid, UuidError> {
        match self.version.as_str() {
            "1" => {
                let ma = get_mac_address()?.ok_or(UuidError::MacAddressNotFound)?;
                return Ok(Uuid::now_v1(&ma.bytes()));
            }
            "4" => {
                return Ok(Uuid::new_v4());
            }
            _ => Err(UuidError::InvalidVersion)
        }
    }

    fn get_uuid_string(&self) -> Result<String, Box<dyn Error>>{
        let uuid = self.get_uuid()?;
        let mut t = if self.no_hyphens { uuid.simple().to_string() } else { uuid.to_string() };
        if self.upper { t = t.to_uppercase(); }
        return Ok(t);
    }

}

impl Runnable for UuidArgs {
    fn run(&self, _: &BaseArgs, _:impl Fn() -> String) -> Result<String,Box<dyn Error>> {
        if self.count == 0 { return Ok(String::new()); }

        let mut s = self.get_uuid_string()?;
        for _ in 0..(self.count - 1) {
            s += &format!("\n{}", &self.get_uuid_string()?);
        }
        return Ok(s);
    }
}