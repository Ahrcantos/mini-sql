mod pager;
mod parser;
mod table;
mod tokenizer;

use std::io::{self, Write};

use parser::{Parser, Statement};
use table::Table;
use tokenizer::Tokenizer;

use crate::table::Row;

fn main() {
    let mut table = Table::new();

    loop {
        let mut buffer = String::new();
        print!("db> ");
        io::stdout().flush().expect("Failed to flush");
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Failed to read line");

        match buffer.trim() {
            special_cmd if special_cmd.starts_with('.') => match special_cmd {
                ".exit" => {
                    table.flush();
                    std::mem::drop(table);
                    std::process::exit(0);
                }
                cmd => {
                    eprintln!("Unrecognized command: {}", cmd)
                }
            },
            sql => {
                execute_sql(sql, &mut table);
            }
        }
    }
}

fn execute_sql(sql: &str, table: &mut Table) {
    let tokens = Tokenizer::parse(sql);
    let parser = Parser::new(tokens);
    let statement = parser.parse();

    match statement {
        Ok(Statement::Select(_)) => {
            let row = table.get_row(0);

            if let Some(row) = row {
                println!("{}", row);
            } else {
                println!("No data");
            }
        }
        Ok(Statement::Insert(stmt)) => {
            let id = stmt.values.get(0);
            let username = stmt.values.get(1);
            let email = stmt.values.get(2);

            use parser::Value;

            match (id, username, email) {
                (
                    Some(Value::Int(id)),
                    Some(Value::String(username)),
                    Some(Value::String(email)),
                ) => {
                    let row = Row::new(*id, username, email);
                    table.insert(row);
                }
                _ => {
                    eprintln!("Invalid values");
                }
            };
        }
        Err(_) => eprintln!("Invalid SQL"),
    }
}
