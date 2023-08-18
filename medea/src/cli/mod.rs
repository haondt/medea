mod args;
mod commands;


use clap::Parser;
use enum_dispatch::enum_dispatch;

use args::{Runnable, BaseArgs};
use commands::uuid::UuidArgs;
use commands::hash::HashArgs;
use commands::timestamp::TimeStampArgs;

#[derive(Parser, Debug)]
#[enum_dispatch(Runnable)]
pub enum ArgsEnum {
    Uuid(UuidArgs),
    Hash(HashArgs),
    #[command(visible_alias="ts")]
    Timestamp(TimeStampArgs),
}

pub use args::run;