mod args;
mod commands;


use clap::Parser;
use enum_dispatch::enum_dispatch;

use args::{Runnable, BaseArgs};
use commands::uuid::UuidArgs;
use commands::hash::HashArgs;

#[derive(Parser, Debug)]
#[enum_dispatch(Runnable)]
pub enum ArgsEnum {
    Uuid(UuidArgs),
    Hash(HashArgs),
}

pub use args::run;