use clap::ValueEnum;


#[derive(ValueEnum, Debug, Clone)]
pub enum Algorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}