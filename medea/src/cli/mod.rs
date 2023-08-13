mod args;
mod commands;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use args::{Runnable, BaseArgs};
use commands::uuid::UuidArgs;
use commands::foo::FooArgs;

#[derive(Parser, Debug)]
#[enum_dispatch(Runnable)]
pub enum ArgsEnum {
    Uuid(UuidArgs),
    Foo(FooArgs),
}

pub use args::run;