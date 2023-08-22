use std::io::{self, Read};

use clap::Parser;
use enum_dispatch::enum_dispatch;
use super::ArgsEnum;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct BaseArgs {
    #[arg(long, help = "Trim newline from end of output", default_value="false")]
    pub trim: bool,

    #[command(subcommand)]
    pub command: ArgsEnum,
}

fn get_input_from_stdin() -> String {
    let mut message = String::new();
    let _ = io::stdin().read_to_string(&mut message);
    return message;
}

pub fn run() -> Result<(), Box<dyn std::error::Error>>  {
    let args = BaseArgs::parse();
    let result = &args.command.run(&args, get_input_from_stdin)?;
    if args.trim {
        print!("{}", result);
    } else {
        println!("{}", result);
    }
    Ok(())
}


#[enum_dispatch]
pub trait Runnable {
    fn run(&self, base_args: &BaseArgs, get_input: impl Fn() -> String) -> Result<String, Box<dyn std::error::Error>>;
}