use clap::Parser;
use enum_dispatch::enum_dispatch;
use super::ArgsEnum;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct BaseArgs {
    #[arg(long, default_value="false")]
    pub colors: bool,

    #[command(subcommand)]
    pub command: ArgsEnum,
}


pub fn run() -> Result<(), Box<dyn std::error::Error>>  {
    let args = BaseArgs::parse();
    let result = &args.command.run(&args)?;
    print!("{}", result);
    Ok(())
}


#[enum_dispatch]
pub trait Runnable {
    fn run(&self, base_args: &BaseArgs) -> Result<String, Box<dyn std::error::Error>>;
}