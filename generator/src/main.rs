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
