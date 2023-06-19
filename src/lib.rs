//! # bingo
//!
//! `bingo` is a collection of utilities to create bingo tickets.
use std::error::Error;
use itertools::Itertools;
use itertools::iproduct;
use thousands::Separable;
use std::iter::Cycle;
use std::ops;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaChaRng;

pub struct Config {
    size: usize,
    seed: u8,
}

#[derive(Clone, Debug)]
struct Generator {
    columns: [[ Vec<Vec<u32>>; 3]; 9],
    indexes: [[Cycle<std::vec::IntoIter<Vec<u32>>>; 3]; 9],
    configurations : Vec<[usize; 9]>
}

impl Generator {
    fn new(seed:u8) -> Self {
        let mut rng: rand_chacha::ChaCha20Rng = ChaChaRng::from_seed([seed; 32]);

        let mut columns: [[Vec<Vec<u32>>; 3]; 9] = [(1..10), (10..20), (20..30), (30..40), (40..50), (50..60), (60..70), (70..80), (80..91)]
                    .map(|x: ops::Range<u32>| [x.clone().combinations(1), x.clone().combinations(2), x.combinations(3)]
                    .map(|y: itertools::Combinations<ops::Range<u32>>| y.collect::<Vec<Vec<u32>>>()));
        
        // shuffle 
        columns.iter_mut().for_each(|x: &mut [Vec<Vec<u32>>; 3] | x.iter_mut().for_each(|y: &mut Vec<Vec<u32>>| y.shuffle(& mut rng)));
        
        let indexes: [[Cycle<std::vec::IntoIter<Vec<u32>>>; 3]; 9] = columns.clone().map(|x: [Vec<Vec<u32>>; 3]| x.map(|y: Vec<Vec<u32>>| y.into_iter().cycle()));
        let mut configurations: Vec<[usize; 9]> = Vec::new();
        let conf = iproduct!(0..3, 0..3, 0..3, 0..3, 0..3, 0..3, 0..3, 0..3, 0..3 );
        for x in conf {
            if x.0 + x.1 + x.2 + x.3 + x.4 + x.5 + x.6 + x.7 + x.8 == 6 {
                configurations.push([x.0, x.1, x.2, x.3, x.4, x.5, x.6, x.7, x.8]);
            }
        }
        configurations.shuffle(& mut rng);
        Generator {columns, indexes, configurations}
    }
    fn show_stats(& mut self) {
        let mut sum: usize = 0;
        for configuration in &self.configurations {
            let mut mult: usize = 1;
            for (m,n) in configuration.iter().enumerate().take(9) {
                mult *= &self.columns[m][*n].len();
            }
            sum += mult;
        }
        println!("Max set of ticket: {}", sum.separate_with_dots());
    }

    fn generate(& mut self, size: usize) {
        let mut tickets: Vec<[u32;15]> = Vec::new();
        let mut config_iter: Cycle<std::vec::IntoIter<[usize; 9]>> = self.configurations.clone().into_iter().cycle();
        for _ in 0..size {
            let config: [usize; 9] = config_iter.next().unwrap();
            let mut ticket:[u32;15] = [0;15];
            let mut index: usize= 0;
            for (m, n) in config.iter().enumerate() {
                for number in  self.indexes[m][*n].next().unwrap().into_iter() {
                    ticket[index] = number;
                    index += 1;
                }
                
            }
            tickets.push(ticket);
        }
        println!("{:?}", tickets);    
        println!("{:?}", tickets.len());
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let size: usize = args.next().expect("Didn't get size").parse::<usize>().unwrap();
        let seed: u8 = args.next().expect("Didn't get a seed").parse::<u8>().unwrap();
        // let file_path: String = args.next().expect("Didn't get a file path");
        Ok(Config { size, seed })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut g: Generator = Generator::new(config.seed);
    g.show_stats();
    g.generate(config.size);

    Ok(())
}
