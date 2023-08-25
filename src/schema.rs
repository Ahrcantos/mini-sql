use std::fmt::{self, Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{collections::HashMap, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSchema {
    tables: HashMap<String, TableSchema>,
}

impl DatabaseSchema {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(&path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let value = serde_json::from_str::<Self>(&content)?;
        Ok(value)
    }

    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = OpenOptions::new().write(true).open(&path)?;

        let content = serde_json::to_string_pretty(self)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn add_table(&mut self, name: &str, schema: TableSchema) {
        self.tables.insert(String::from(name), schema);
    }

    pub fn get_table_schema(&self, name: &str) -> Option<&TableSchema> {
        self.tables.get(name)
    }

    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    columns: Vec<Column>,
}

impl TableSchema {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(&path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let value = serde_json::from_str(&content)?;
        Ok(value)
    }

    pub fn offset(&self, column_name: &str) -> Option<usize> {

        let mut total = 0;

        for column in self.columns.iter() {
            if column.name == column_name {
                return Some(total)
            } else {
                total += column.size();
            }
        }

        None
    }

    pub fn size(&self) -> usize {
        self.columns.iter().map(|c| c.size()).sum()
    }
}

impl Display for TableSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{0: <16} | {1: <16} | {2: <6} | {3: <6}",
            "column", "type", "size", "offset"
        )?;
        for column in self.columns.iter() {
            let kind = match column.kind {
                ColumnKind::Int => String::from("int"),
                ColumnKind::String(StringColumn { length }) => format!("string({})", length),
            };

            let size = column.size();
            let offset = self.offset(&column.name).unwrap();

            writeln!(
                f,
                "{0: <16} | {1: <16} | {2: <6} | {3: <6}",
                column.name, kind, size, offset
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    name: String,
    kind: ColumnKind,
}

impl Column {
    pub fn size(&self) -> usize {
        match self.kind {
            ColumnKind::Int => 4,
            ColumnKind::String(StringColumn { length }) => length,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ColumnKind {
    #[serde(rename = "string")]
    String(StringColumn),
    #[serde(rename = "int")]
    Int,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringColumn {
    length: usize,
}
