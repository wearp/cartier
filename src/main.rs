use std::io::{self, Write};
use std::process::exit;

enum MetaCommandResult {
    MetaCommandSuccess
}

fn main() {
    loop {
        let input = read_input();

        if input.as_str().starts_with(".") {
            let result = execute_meta_command(&input);
            match result {
                Ok(_command) => (),
                Err(_e) => print!("Unrecognised command '{}'.\n", input),
            }
        } else {
            continue
        }
    }
}

fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error reading input");

    // ignore leading and trailing whitespace
    input = input.trim().parse().unwrap();

    input
}

fn execute_meta_command(input: &str) -> Result<MetaCommandResult, &'static str> {
    if input == ".exit" {
        exit(0);
    } else {
        Err("Invalid command")
    }
}
