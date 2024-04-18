mod cli;

use cli::display;

use std::io::{self, BufReader};
fn main() {
    let stdin = io::stdin();
    let input_source = BufReader::new(stdin.lock());
    let output_sink = io::stdout();

    display(input_source, output_sink);
}
