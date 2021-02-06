use std::io::{self, Write};
use std::process::exit;

enum MetaCommandResult {
    MetaCommandSuccess,
}

enum StatementType {
    SelectStatement,
    InsertStatement,
}

struct Statement {
    statement_type: StatementType,
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
            continue;
        }
        let statement_result = prepare_statement(&input);
        match statement_result {
            Ok(statement) => execute_statement(statement),
            Err(_e) => print!("Unrecognized keyword at start of '{}'.\n", input),
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

fn prepare_statement(input: &str) -> Result<Statement, &'static str> {
    match input {
        "insert" => Ok(Statement {
            statement_type: StatementType::InsertStatement,
        }),
        "select" => Ok(Statement {
            statement_type: StatementType::SelectStatement,
        }),
        _ => Err("Invalid statement"),
    }
}

fn execute_statement(statement: Statement) {
    match statement.statement_type {
        StatementType::SelectStatement => print!("This is where we would do a select.\n"),
        StatementType::InsertStatement => print!("This is where we would do a insert.\n"),
    }
}
