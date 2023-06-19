use std::env;
use std::process;
use bingo::Config;

fn main() {
   
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = bingo::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
