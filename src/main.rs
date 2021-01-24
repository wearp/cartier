use std::io::{self, Write};

fn main() {
    loop {
        let input = read_input();
        println!("Your input: {}", input)
    }
}

fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input
}