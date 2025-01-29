# Bingo Ticket Generator and Simulator

This repository contains a set of Rust crates for generating and simulating bingo tickets. It includes three main components: a generator for creating bingo tickets, a simulator for running bingo games, and a ticket library for handling ticket operations.

## Components

### Generator
The `generator` crate is responsible for creating bingo tickets. It uses random number generation to produce unique ticket configurations, ensuring a fair and varied set of tickets for each game.

### Simulator
The `simulator` crate simulates bingo games using the generated tickets. It shuffles numbers and determines winners based on the tickets, providing a realistic simulation of a bingo game.

### Ticket
The `ticket` crate provides utilities for reading and displaying bingo tickets. It handles file I/O and ticket formatting, making it easy to manage and present tickets in a user-friendly format.

## Installation

To build and run the project, you need to have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and navigate to the project directory:

```bash
git clone <repository-url>
cd <repository-directory>
```

## Usage

### Generating Tickets

To generate bingo tickets, navigate to the `generator` directory and run the following command. Replace `<number_of_tickets>` with the desired number of tickets and `<random_seed>` with a seed for random number generation:

```bash
cargo run --bin generator -- --size <number_of_tickets> --seed <random_seed>
```

### Simulating Bingo Games

To simulate bingo games, navigate to the `simulator` directory and run the following command. Replace `<path_to_ticket_file>` with the path to your ticket file, `<number_of_raffles>` with the number of raffles to simulate, and `<random_seed>` with a seed for random number generation:

```bash
cargo run --bin simulator -- --file <path_to_ticket_file> --size <number_of_raffles> --seed <random_seed>
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
