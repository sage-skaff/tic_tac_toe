use std::io::{BufRead, Write};

pub fn display<I, O>(input_source: I, mut output_sink: O)
where
    I: BufRead,
    O: Write,
{
    writeln!(output_sink, "-----------------------").unwrap();
    writeln!(output_sink, "Welcome to Tic Tac Toe!").unwrap();
    writeln!(output_sink, "-----------------------\n").unwrap();
    writeln!(output_sink, "{}", help()).unwrap();

    for command in input_source.lines() {
        let command = command.unwrap();

        if command == "exit" {
            writeln!(output_sink, "Exiting game...").unwrap();
            break;
        }

        let output = process_input(&command);
        writeln!(output_sink, "{}", output).unwrap();
    }
}

pub fn process_input(input: &str) -> String {
    match input {
        "play" => "Playing the game...".to_string(),
        "exit" => "Exiting game...".to_string(),
        "help" => help(),
        _ => "Invalid command".to_string(),
    }
}

pub fn help() -> String {
    let mut help_output = String::new();
    help_output.push_str("Available commands:\n");
    help_output.push_str("play - Start the game\n");
    help_output.push_str("exit - Exit the game\n");
    help_output.push_str("help - Display this list\n");
    help_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_play() {
        let output = process_input("play");
        assert_eq!(output, "Playing the game...");
    }

    #[test]
    fn test_process_input_exit() {
        let output = process_input("exit");
        assert_eq!(output, "Exiting game...");
    }

    #[test]
    fn test_process_input_help() {
        let output = process_input("help");
        assert!(output.contains("Available commands:"));
        assert!(output.contains("play - Start the game"));
        assert!(output.contains("exit - Exit the game"));
        assert!(output.contains("help - Display this list"));
    }

    #[test]
    fn test_process_input_invalid() {
        let output = process_input("invalid");
        assert_eq!(output, "Invalid command");
    }
}
