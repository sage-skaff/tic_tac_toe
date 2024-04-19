use std::io::{self, Write, BufRead};
use std::fmt;

const HELP_MESSAGE: &str = "\
-----------------------\n\
Welcome to Tic Tac Toe!\n\
-----------------------\n\
\n\
Available commands:\n\
- play: Start the game\n\
- exit: Exit the game\n\
- help: Display this help message\n";

// Represents possible commands a user can input
enum Command {
    Play,
    Help,
    Exit,
    Invalid,
}

// Converts string input into corresponding Command enum
impl From<&str> for Command {
    fn from(input: &str) -> Self {
        match input {
            "play" => Command::Play,
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}

// Allows Commands to be easily printed, linking enum variants to specific messages
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

// Manages game loop and handles user input, writing responses to the output
pub fn start_game_interface<I, O>(input_source: I, mut output_sink: O) -> io::Result<()>
where
    I: BufRead,
    O: Write,
{
    writeln!(output_sink, "{}", HELP_MESSAGE)?;

    for command_line in input_source.lines() {
        let command = Command::from(command_line?.as_str());

        if let Command::Exit = command {
            writeln!(output_sink, "{}", command)?;
            break;
        }

        writeln!(output_sink, "{}", command)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_command_from_str() {
        assert!(matches!(Command::from("play"), Command::Play));
        assert!(matches!(Command::from("help"), Command::Help));
        assert!(matches!(Command::from("exit"), Command::Exit));
        assert!(matches!(Command::from("unknown"), Command::Invalid));
    }

    #[test]
    fn test_command_display() {
        assert_eq!(Command::Play.to_string(), "Playing the game...");
        assert_eq!(Command::Help.to_string(), HELP_MESSAGE);
        assert_eq!(Command::Exit.to_string(), "Exiting game...");
        assert_eq!(Command::Invalid.to_string(), "Invalid command");
    }

    #[test]
    fn test_start_game_interface() {
        let input = "help\nplay\nexit\n";
        let input = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        start_game_interface(input, &mut output).unwrap();
        
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains(HELP_MESSAGE));
        assert!(output.contains("Playing the game..."));
        assert!(output.contains("Exiting game..."));
    }
}
