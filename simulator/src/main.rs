// Copyright 2025 Pablo E. Pilotti
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use argparse::{ArgumentParser, Store, StoreTrue};
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
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut file)
            .add_option(&["--file"], Store, "File with tickets")
            .required();
        ap.refer(&mut seed).add_option(
            &["--seed"],
            Store,
            "Seed for the random generation numbers",
        );
        ap.refer(&mut size)
            .add_option(&["--size"], Store, "how many raffles simulate");
        ap.parse_args_or_exit();
    }

    match ticket::read_tickets(file) {
        Some(tickets) => {
            if let Err(e) = simulator::run(simulator::Config {
                tickets,
                size,
                seed,
                verbose,
            }) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        None => {
            eprintln!("Error: Could not read tickets from file");
            process::exit(1);
        }
    }
}
