use clap::Parser;
use enum_dispatch::enum_dispatch;
use super::ArgsEnum;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct BaseArgs {
    #[arg(short, long, global = true)]
    pub use_colors: Option<bool>,

    #[command(subcommand)]
    pub command: ArgsEnum,
}


pub fn run() -> Result<(), Box<dyn std::error::Error>>  {
    let args = BaseArgs::parse();
    let result = &args.command.run(&args);
    println!("{}", result);
    Ok(())
}


#[enum_dispatch]
pub trait Runnable {
    fn run(&self, base_args: &BaseArgs) -> String;
}