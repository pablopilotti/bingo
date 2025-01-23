use argparse::{ArgumentParser, Store, StoreTrue};
use generator::Config;
use std::process;

fn main() {
    // Default values
    let mut verbose: bool = false;
    let mut size: usize = 10;
    let mut seed: u8 = 0;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap: ArgumentParser<'_> = ArgumentParser::new();
        ap.set_description("Generate and analize tombola ticket set");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut size)
            .add_option(&["--size"], Store, "Size of the set of tombola tickets");
        ap.refer(&mut seed).add_option(
            &["--seed"],
            Store,
            "seed for the random generation numbers",
        );
        ap.parse_args_or_exit();
    }

    if let Err(e) = generator::run(Config {
        size,
        seed,
        verbose,
    }) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
