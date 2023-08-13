use clap::Parser;
use super::super::{Runnable, BaseArgs};

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