mod args;
mod commands;
mod utils;


use clap::Parser;
use enum_dispatch::enum_dispatch;

use args::{Runnable, BaseArgs};
use commands::uuid::UuidArgs;
use commands::hash::HashArgs;
use commands::timestamp::TimeStampArgs;
use commands::random::RandomArgs;
use commands::base_convert::BaseConvertArgs;

#[derive(Parser, Debug)]
#[enum_dispatch(Runnable,)]
pub enum ArgsEnum {
    Uuid(UuidArgs),
    Hash(HashArgs),
    #[command(visible_alias="ts")]
    Timestamp(TimeStampArgs),
    #[command(visible_alias="rnd")]
    Random(RandomArgs),
    #[command(visible_alias="base")]
    BaseConvert(BaseConvertArgs),
}

pub use args::run;

