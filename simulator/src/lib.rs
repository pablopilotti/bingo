use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaChaRng;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug)]
/// Configuration for the bingo simulator.
///
/// # Fields
/// - `size`: The number of raffles to simulate.
/// - `seed`: The seed for random number generation.
/// - `verbose`: Whether to print detailed statistics.
/// - `tickets`: A vector of bingo tickets to use in the simulation.
pub struct Config {
    pub size: u8,
    pub seed: u8,
    pub verbose: bool,
    pub tickets: Vec<[u32; 15]>,
}
impl Config {
    /// Shuffles numbers from 1 to 90 using the given seed.
    ///
    /// # Arguments
    /// - `seed`: A seed for the random number generator.
    ///
    /// # Returns
    /// An array of shuffled numbers.
    fn get_shuffle_numbers(&self, seed: u8) -> [u32; 90] {
        let mut rng: rand_chacha::ChaCha20Rng = ChaChaRng::from_seed([seed; 32]);
        let mut numbers: Vec<u32> = (1..91).collect::<Vec<u32>>();
        numbers.shuffle(&mut rng);

        // This conversion is safe because we know numbers has exactly 90 elements
        // But we'll handle it safely anyway
        match numbers.try_into() {
            Ok(array) => array,
            Err(_) => {
                // This should never happen, but if it does, return a sequential array
                let mut fallback = [0; 90];
                for i in 0..90 {
                    fallback[i] = i as u32 + 1;
                }
                fallback
            }
        }
    }
}

// #[derive(Clone, Debug)]
// struct Simulator {
//     columns: [[ Vec<Vec<u32>>; 3]; 9],
//     indexes: [[Cycle<std::vec::IntoIter<Vec<u32>>>; 3]; 9],
//     configurations : Vec<[usize; 9]>,
//     conf_iter: Cycle<std::vec::IntoIter<[usize; 9]>>
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Initialize a control map to track numbers matched on each ticket
    let mut ticket_match_count: HashMap<[u32; 15], u32> = HashMap::new();

    // List to store winning tickets
    let mut winning_tickets: Vec<[u32; 15]> = Vec::new();

    // Run the specified number of raffles
    for raffle in 0..config.size {
        // Get shuffled numbers for this raffle
        let numbers = config.get_shuffle_numbers(raffle + config.seed);

        // Reset match counts for all tickets
        ticket_match_count.clear();
        for ticket in &config.tickets {
            ticket_match_count.insert(*ticket, 0);
        }

        // Clear winning tickets from previous raffle
        winning_tickets.clear();

        // Draw numbers one by one
        for drawn_number in numbers {
            // Update match counts for each ticket
            for (ticket, match_count) in ticket_match_count.iter_mut() {
                if ticket.contains(&drawn_number) {
                    *match_count += 1;

                    // Check if ticket has won (all 15 numbers matched)
                    if *match_count == 15 {
                        winning_tickets.push(*ticket);
                    }
                }
            }

            // Stop drawing if we have winners
            if !winning_tickets.is_empty() {
                break;
            }
        }

        // Print results for this raffle
        println!("Raffle {}: {} Winners", raffle, winning_tickets.len());

        // Print detailed ticket info if verbose mode is enabled
        if config.verbose && !winning_tickets.is_empty() {
            println!("Winning tickets:");
            for ticket in &winning_tickets {
                ticket::show(*ticket);
            }
        }
    }

    Ok(())
}
