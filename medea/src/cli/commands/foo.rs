use clap::Parser;
use super::super::{Runnable, BaseArgs};

#[derive(Parser, Debug)]
pub struct FooArgs { }

impl Runnable for FooArgs {
    fn run(&self, _: &BaseArgs) -> Result<String,Box<dyn std::error::Error>> {
        return Ok("Hello from foo command!".to_string());
    }
}