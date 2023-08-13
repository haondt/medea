use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct BaseArgs {
    #[arg(short, long, global = true)]
    pub use_colors: Option<bool>,

    #[command(subcommand)]
    pub command: ArgsEnum,
}

impl BaseArgs {
    pub fn parse_args() -> BaseArgs {
        let args = Self::parse();
        return args;
    }
}

#[enum_dispatch]
trait Runnable {
    fn run(&self, base_args: &BaseArgs) -> String;
}

///////////////////////////////////////////////////

// uuid: generate n uuids

#[derive(Parser, Debug)]
pub struct UuidArgs {
    #[arg(short, long, global = true)]
    pub count: Option<i32>,
}


impl Runnable for UuidArgs {
    fn run(&self, base_args: &BaseArgs) -> String {
        return format!("use colors: {}, count: {}", &base_args.use_colors.map(|c| c.to_string()).unwrap_or_default(), &self.count.unwrap_or_default());
    }
}




// foo: TODO

#[derive(Parser, Debug)]
pub struct FooArgs { }

impl Runnable for FooArgs {
    fn run(&self, _: &BaseArgs) -> String {
        return "Hello from foo command!".to_string();
    }
}

///////////////////////////////////////////////////

#[derive(Parser, Debug)]
#[enum_dispatch(Runnable)]
pub enum ArgsEnum {
    Uuid(UuidArgs),
    Foo(FooArgs)
}



fn run() -> Result<(), Box<dyn std::error::Error>>  {
    let args = BaseArgs::parse();
    let result = &args.command.run(&args);
    println!("{}", result);
    Ok(())
}


///////////////////////////////////////////////////

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
        std::process::exit(libc::EXIT_FAILURE);
    }
}
