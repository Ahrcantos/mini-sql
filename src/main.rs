mod pager;
mod parser;
mod schema;
mod table;
mod tokenizer;

use std::{io::{self, Write}, path::PathBuf};

use clap::Parser;

use parser::Statement;
use schema::{DatabaseSchema, TableSchema};
use table::{Row, Table};
use tokenizer::Tokenizer;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    data_dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    let schema_file = args.data_dir.join("./schema.json");

    let mut table = Table::new();
    let mut db_schema =
        DatabaseSchema::load(&schema_file).expect("Failed to load database schema");

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
                },

                ".list" => {
                    let tables = db_schema.list_tables();
                    for table in tables {
                        println!("{}", table);
                    }
                },

                cmd if cmd.starts_with(".create") => {

                    let args = cmd.split(' ').collect::<Vec<&str>>();

                    let table_name = args.get(1).expect("Argument for table name was not provided");
                    let path = args.get(2).expect("Argument for path was not provided");

                    let schema = TableSchema::load(path).expect("Failed to load table schema");
                    db_schema.add_table(table_name, schema);
                    db_schema.save(&schema_file).expect("Failed to save schema");
                },

                cmd if cmd.starts_with(".table") => {
                    let args = cmd.split(' ').collect::<Vec<&str>>();
                    let table_name = args.get(1).expect("Argument for table name was not provided");

                    let schema = db_schema.get_table_schema(table_name);

                    match schema {
                        Some(schema) => { print!("{}", schema) },
                        None => { eprintln!("Table \"{}\" does not exist!", &table_name)}
                    }

                },
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
    let parser = parser::Parser::new(tokens);
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
