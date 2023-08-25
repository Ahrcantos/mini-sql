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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    name: String,
    kind: ColumnKind,
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
