mod board;
mod cli;

use cli::*;

use std::io::{self};

fn main() {
    let input = io::BufReader::new(io::stdin().lock());
    let output = io::stdout();

    if let Err(e) = start_game_interface(input, output) {
        eprintln!("Error: {}", e);
    }
}
