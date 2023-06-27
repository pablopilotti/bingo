use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use colored::Colorize;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_tickets(filename: String) -> Option<Vec<[u32;15]>> {
    if let Ok(lines) = read_lines(filename) {
        let mut tickets:Vec<[u32;15]> = Vec::new();
        for ticket_str in lines.flatten() {
            let ticket:[u32;15] = ticket_str.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
            tickets.push(ticket);
        }
        return Some(tickets);
    }
    None
}

pub fn show(ticket: [u32;15]) {
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
             ticket[0], ticket[1], ticket[2], ticket[3], ticket[4], 
             ticket[5], ticket[6], ticket[7], ticket[8], ticket[9], 
             ticket[10], ticket[11], ticket[12], ticket[13], ticket[14]); 
}
pub fn showm(matrix : &[[Option<u8>;9];6]) {
    let mut sum2:[u8;9] = [0;9];
    for fil in matrix {
        let mut sum = 0;
        for (col ,j) in fil.iter().enumerate() {
            match j {
                Some(n) => {sum+= *n+1; sum2[col]+=*n+1; print!("{:?} ",n)}
                None => {print!("_ ")}
            }
        }
        println!("{}{}{}", "[".red(),sum,"]".red());
    }
    for i in sum2 {
        print!("{} ",i);    
    }
    println!();
}
