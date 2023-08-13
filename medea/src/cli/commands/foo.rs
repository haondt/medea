use clap::Parser;
use super::super::{Runnable, BaseArgs};

#[derive(Parser, Debug)]
pub struct FooArgs { }

impl Runnable for FooArgs {
    fn run(&self, _: &BaseArgs) -> String {
        return "Hello from foo command!".to_string();
    }
}