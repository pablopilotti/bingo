use argparse::{ArgumentParser, StoreTrue, Store};
use std::process;

fn main() {
    // Default values
    let mut verbose: bool = false;
    let mut size: u8 = 10;
    let mut seed: u8 = 0;
    let mut file: String = "".to_string();

    {
        let mut ap: ArgumentParser<'_> = ArgumentParser::new();
        ap.set_description("Load a file with tickets and simulate a raffles");
        ap.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue,"Be verbose");
        ap.refer(&mut file).add_option(&["--file"], Store,"File with tickets").required();
        ap.refer(&mut seed).add_option(&["--seed"], Store,"Seed for the random generation numbers");
        ap.refer(&mut size).add_option(&["--size"], Store,"how many raffles simulate");
        ap.parse_args_or_exit();
    }

    let tickets = ticket::read_tickets(file).unwrap();
    if let Err(e) = simulator::run(simulator::Config{tickets, size, seed, verbose}) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
