use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_tickets(filename: String) -> Option<Vec<[u32; 15]>> {
    match read_lines(&filename) {
        Ok(lines) => {
            let mut tickets: Vec<[u32; 15]> = Vec::new();
            let mut line_number = 0;

            for line_result in lines {
                line_number += 1;
                match line_result {
                    Ok(ticket_str) => {
                        // Parse each number, skipping invalid ones
                        let numbers: Vec<u32> = ticket_str
                            .split_whitespace()
                            .filter_map(|x| x.parse::<u32>().ok())
                            .collect();

                        // Only add valid tickets with exactly 15 numbers
                        if numbers.len() == 15 {
                            match numbers.try_into() {
                                Ok(ticket) => tickets.push(ticket),
                                Err(_) => eprintln!(
                                    "Warning: Failed to convert numbers to ticket array on line {}",
                                    line_number
                                ),
                            }
                        } else {
                            eprintln!("Warning: Invalid ticket on line {} - expected 15 numbers, found {}", 
                                     line_number, numbers.len());
                        }
                    }
                    Err(e) => eprintln!("Warning: Failed to read line {}: {}", line_number, e),
                }
            }

            if !tickets.is_empty() {
                Some(tickets)
            } else {
                eprintln!("Error: No valid tickets found in file '{}'", filename);
                None
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to open file '{}': {}", filename, e);
            None
        }
    }
}

pub fn show(ticket: [u32; 15]) {
    // Format ticket numbers with consistent spacing
    let formatted_numbers: Vec<String> = ticket.iter().map(|num| format!("{:2}", num)).collect();

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        formatted_numbers[0],
        formatted_numbers[1],
        formatted_numbers[2],
        formatted_numbers[3],
        formatted_numbers[4],
        formatted_numbers[5],
        formatted_numbers[6],
        formatted_numbers[7],
        formatted_numbers[8],
        formatted_numbers[9],
        formatted_numbers[10],
        formatted_numbers[11],
        formatted_numbers[12],
        formatted_numbers[13],
        formatted_numbers[14]
    );
}
pub fn showm(matrix: &[[Option<u8>; 9]; 6]) {
    let mut column_sums: [u8; 9] = [0; 9];

    // Process each row
    for row in matrix {
        let mut row_sum = 0;

        // Process each cell in the row
        for (col, cell) in row.iter().enumerate() {
            match cell {
                Some(n) => {
                    let value = *n + 1;
                    row_sum += value;
                    column_sums[col] += value;
                    print!("{:2} ", n);
                }
                None => {
                    print!("_  ");
                }
            }
        }

        // Print row sum
        println!("{}{}{}", "[".red(), row_sum, "]".red());
    }

    // Print column sums
    for sum in &column_sums {
        print!("{:2} ", sum);
    }
    println!();
}
