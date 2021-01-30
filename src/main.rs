use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        let input = read_input();

        if input.as_str() == ".exit"{
            exit(0);
        } else if input.is_empty() == true {
            continue;
        } else {
            print!("Unrecognised command '{}'.\n", input);
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
