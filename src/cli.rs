// Path: src/cli.rs

use crate::board::Board;
use std::fmt;
use std::io::{self, BufRead, Write};

const HELP_MESSAGE: &str = "\
-----------------------\n\
Welcome to Tic Tac Toe!\n\
-----------------------\n\
\n\
Available commands:\n\
- play: Start the game\n\
- exit: Exit the game\n\
- help: Display this help message\n";

enum Command {
    Play,
    Help,
    Exit,
    Invalid,
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        match input.trim() {
            "play" => Command::Play,
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Play => write!(f, "Playing the game..."),
            Command::Help => write!(f, "{}", HELP_MESSAGE),
            Command::Exit => write!(f, "Exiting game..."),
            Command::Invalid => write!(f, "Invalid command"),
        }
    }
}

pub fn start_game_interface<I, O>(input_source: I, mut output_sink: O) -> io::Result<()>
where
    I: BufRead,
    O: Write,
{
    writeln!(output_sink, "{}", HELP_MESSAGE)?;
    let board = Board::new(); // Initialize the game board

    for command_line in input_source.lines() {
        let command_line = command_line?;
        let command = Command::from(command_line.as_str());

        match command {
            Command::Play => {
                writeln!(output_sink, "{}", board.get_board_state())?;
            }
            Command::Exit => {
                writeln!(output_sink, "{}", command)?;
                break;
            }
            _ => {
                writeln!(output_sink, "{}", command)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, BufWriter, Cursor};

    #[test]
    fn test_initial_help_message() {
        let input = Cursor::new(b"");
        let output = Vec::new();
        let input = BufReader::new(input);
        let mut output = BufWriter::new(output);

        start_game_interface(input, &mut output).unwrap();

        let output = output.into_inner().unwrap(); // Take ownership of the inner buffer
        let output = String::from_utf8(output).unwrap();

        assert!(output.contains(HELP_MESSAGE));
    }

    #[test]
    fn test_display_board_on_play() {
        let input = Cursor::new(b"play\nexit\n");
        let output = Vec::new();
        let input = BufReader::new(input);
        let mut output = BufWriter::new(output);
    
        start_game_interface(input, &mut output).unwrap();
    
        let output = output.into_inner().unwrap();
        let output = String::from_utf8(output).unwrap();
    
        let expected_board_state = "  |   |  \n---------\n  |   |  \n---------\n  |   |  ";
    
        assert!(output.contains(expected_board_state))
    }
    

    #[test]
    fn test_exit_command() {
        let input = Cursor::new(b"exit\n");
        let output = Vec::new();
        let input = BufReader::new(input);
        let mut output = BufWriter::new(output);

        start_game_interface(input, &mut output).unwrap();

        let output = output.into_inner().unwrap(); // Take ownership of the inner buffer
        let output = String::from_utf8(output).unwrap();

        assert!(output.contains("Exiting game..."));
    }

    #[test]
    fn test_invalid_command() {
        let input = Cursor::new(b"unknown\nexit\n");
        let output = Vec::new();
        let input = BufReader::new(input);
        let mut output = BufWriter::new(output);

        start_game_interface(input, &mut output).unwrap();

        let output = output.into_inner().unwrap(); // Take ownership of the inner buffer
        let output = String::from_utf8(output).unwrap();

        assert!(output.contains("Invalid command"));
    }
}
