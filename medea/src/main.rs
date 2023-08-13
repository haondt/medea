mod cli;
use cli::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
        std::process::exit(libc::EXIT_FAILURE);
    }
}
