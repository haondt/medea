mod cli;
use cli::run;
use colored::Colorize;

fn main() {
    if let Err(err) = run() {
        let error_str = "error:".red().bold();
        eprintln!("{}: {}", &error_str, err.to_string());
        std::process::exit(libc::EXIT_FAILURE);
    }
}
