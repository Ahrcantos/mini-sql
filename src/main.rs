mod tokenizer;

use std::io::{self, Write};

fn main() {
    loop {
        let mut buffer = String::new();
        print!("db> ");
        io::stdout().flush().expect("Failed to flush");
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Failed to read line");

        match buffer.trim() {
            special_cmd if special_cmd.starts_with(".") => match special_cmd {
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

enum Statement {
    Select,
    Insert,
}

struct SelectStatement {
    columns: Vec<String>,
    table: String,
    r#where: WhereClause,
    pagination: Pagination,
}

struct Pagination {
    limit: usize,
    offset: usize,
}

struct WhereClause {}

fn execute_sql(sql: &str) -> Statement {
    if sql.to_lowercase().starts_with("select") {
        return Statement::Select;
    }

    if sql.to_lowercase().starts_with("insert") {
        return Statement::Insert;
    }

    panic!("Invalid sql statement");
}
