mod tokenizer;
mod parser;

use std::io::{self, Write};

use tokenizer::Tokenizer;
use parser::Parser;

fn main() {
    loop {
        let mut buffer = String::new();
        print!("db> ");
        io::stdout().flush().expect("Failed to flush");
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Failed to read line");

        match buffer.trim() {
            special_cmd if special_cmd.starts_with('.') => match special_cmd {
                ".exit" => std::process::exit(0),
                cmd => {
                    eprintln!("Unrecognized command: {}", cmd)
                }
            },
            sql => {
                execute_sql(sql);
            }
        }
    }
}

fn execute_sql(sql: &str) {
    let tokens = Tokenizer::parse(sql);
    let parser = Parser::new(tokens);
    let statement = parser.parse();
    println!("{:?}", statement);
}
