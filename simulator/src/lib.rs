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
        numbers.try_into().unwrap()
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
    // Initialize a control map to track numbers matched on each ticket.
    let mut ticket_match_count: HashMap<[u32; 15], u32> = HashMap::new();
    // List to store winning tickets.
    let mut winning_tickets: Vec<[u32; 15]> = Vec::new();
    for raffle in 0..config.size {
        let numbers = config.get_shuffle_numbers(raffle + config.seed);

        ticket_match_count.clear();
        for ticket in &config.tickets {
            ticket_match_count.insert(*ticket, 0);
        }

        winning_tickets.clear();
        for drawn_number in numbers {
            for (ticket, match_count) in ticket_match_count.iter_mut() {
                if ticket.contains(&drawn_number) {
                    *match_count += 1;
                    if *match_count == 15 {
                        winning_tickets.push(*ticket);
                    }
                }
            }
            if !winning_tickets.is_empty() {
                break;
            }
        }
        println!("{} Winners  {:?} ", raffle, winning_tickets.len());
    }

    Ok(())
}
