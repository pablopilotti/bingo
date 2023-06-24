use std::error::Error;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaChaRng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Config {
    pub size: u8,
    pub seed: u8,
    pub verbose: bool,
    pub tickets: Vec<[u32;15]>
}
impl Config {
    fn get_shuffle_numbers(&self, seed: u8) -> [u32;90]{
        let mut rng: rand_chacha::ChaCha20Rng = ChaChaRng::from_seed([seed; 32]);
        let mut numbers: Vec<u32> = (1..91).collect::<Vec<u32>>();
        numbers.shuffle(& mut rng);
        return numbers.try_into().unwrap();
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

    let mut control: HashMap<[u32;15], u32> = HashMap::new();
    let mut winners: Vec<[u32;15]>=Vec::new();
    for raffle in 0..config.size {
        let numbers = config.get_shuffle_numbers(raffle + config.seed);

        control.clear();
        for ticket in &config.tickets {
            control.insert(*ticket, 0);
        }

        winners.clear();
        for new_number in numbers {
            for (ticket, value) in control.iter_mut() {
                if ticket.contains(&new_number){
                    *value += 1;
                    if *value == 15 {
                        winners.push(*ticket);
                    }
                }
            }
            if !winners.is_empty() {
                break;
            }
        }
        println!("{} Winners  {:?} ",raffle, winners.len());
    }

    Ok(())
}
