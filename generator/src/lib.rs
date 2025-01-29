//! # bingo
//!
//! `bingo` is a collection of utilities to create bingo tickets.
use itertools::{iproduct, Itertools};
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaChaRng;
use std::error::Error;
use std::iter::from_fn;
use std::iter::Cycle;
use std::ops;
use thousands::Separable;
use ticket;

/// Configuration for the bingo ticket generator.
///
/// # Fields
/// - `size`: The number of tickets to generate.
/// - `seed`: The seed for random number generation.
/// - `verbose`: Whether to print detailed statistics.
pub struct Config {
    pub size: usize,
    pub seed: u8,
    pub verbose: bool,
}

#[derive(Clone, Debug)]
struct Generator {
    columns: [[Vec<Vec<u32>>; 3]; 9],
    indexes: [[Cycle<std::vec::IntoIter<Vec<u32>>>; 3]; 9],
    configurations: Vec<[usize; 9]>,
    conf_iter: Cycle<std::vec::IntoIter<[usize; 9]>>,
}

impl Generator {
    /// Creates a new `Generator` with the specified seed.
    ///
    /// # Arguments
    /// - `seed`: A seed for the random number generator.
    fn new(seed: u8) -> Self {
        let mut rng: rand_chacha::ChaCha20Rng = ChaChaRng::from_seed([seed; 32]);

        let mut columns: [[Vec<Vec<u32>>; 3]; 9] = [
            (1..10),
            (10..20),
            (20..30),
            (30..40),
            (40..50),
            (50..60),
            (60..70),
            (70..80),
            (80..91),
        ]
        .map(|range: ops::Range<u32>| {
            [
                range.clone().combinations(1),
                range.clone().combinations(2),
                range.combinations(3),
            ]
            .map(|combination: itertools::Combinations<ops::Range<u32>>| combination.collect::<Vec<Vec<u32>>>())
        });

        // shuffle
        columns.iter_mut().for_each(|column: &mut [Vec<Vec<u32>>; 3]| {
            column.iter_mut()
                .for_each(|combination_set: &mut Vec<Vec<u32>>| combination_set.shuffle(&mut rng))
        });

        let indexes: [[Cycle<std::vec::IntoIter<Vec<u32>>>; 3]; 9] = columns
            .clone()
            .map(|column: [Vec<Vec<u32>>; 3]| column.map(|combination_set: Vec<Vec<u32>>| combination_set.into_iter().cycle()));
        let mut configurations: Vec<[usize; 9]> = Vec::new();
        let conf = iproduct!(0..3, 0..3, 0..3, 0..3, 0..3, 0..3, 0..3, 0..3, 0..3);
        for configuration in conf {
            if configuration.0 + configuration.1 + configuration.2 + configuration.3 + configuration.4 + configuration.5 + configuration.6 + configuration.7 + configuration.8 == 6 {
                configurations.push([configuration.0, configuration.1, configuration.2, configuration.3, configuration.4, configuration.5, configuration.6, configuration.7, configuration.8]);
            }
        }
        configurations.shuffle(&mut rng);
        let conf_iter: Cycle<std::vec::IntoIter<[usize; 9]>> =
            configurations.clone().into_iter().cycle();
        Generator {
            columns,
            indexes,
            configurations,
            conf_iter,
        }
    }
    /// Displays statistics about the maximum set of tickets.
    fn show_stats(&mut self) {
        let mut sum: usize = 0;
        for configuration in &self.configurations {
            let mut mult: usize = 1;
            for (column_index, combination_index) in configuration.iter().enumerate().take(9) {
                mult *= &self.columns[column_index][*combination_index].len();
            }
            sum += mult;
        }
        println!("Max set of ticket: {}", sum.separate_with_dots());
    }

    /// Generates a new bingo ticket.
    ///
    /// # Returns
    /// An iterator over arrays of 15 numbers representing a bingo ticket.
    fn generate(&mut self) -> impl Iterator<Item = [u32; 15]> + '_ {
        from_fn(move || {
            let config: [usize; 9] = self.conf_iter.next().unwrap();
            let mut ticket: [u32; 15] = [0; 15];
            let mut ticket_index: usize = 0;
            for (column_index, combination_index) in config.iter().enumerate() {
                for number in self.indexes[column_index][*combination_index].next().unwrap().into_iter() {
                    ticket[ticket_index] = number;
                    ticket_index += 1;
                }
            }
            Some(ticket)
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut generator_instance: Generator = Generator::new(config.seed);
    if config.verbose {
        generator_instance.show_stats();
    }

    for _ in 0..config.size {
        let ticket: [u32; 15] = generator_instance.generate().next().unwrap();
        ticket::show(ticket);
    }
    Ok(())
}
