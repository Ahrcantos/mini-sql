use std::fmt::{self, Display, Formatter};

pub struct Table {
    rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn insert(&mut self, row: Row) {
        self.rows.push(row);
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

pub struct Row {
    id: u32,
    username: [u8; 128],
    email: [u8; 128],
}

impl Row {
    pub fn new(id: u32, username: &str, email: &str) -> Self {
        let username = username.as_bytes();
        let email = email.as_bytes();

        if username.len() > 128 {
            panic!("Username is too long");
        }

        if email.len() > 128 {
            panic!("Email is too long");
        }

        let mut username_buffer: [u8; 128] = [0; 128];
        let mut email_buffer: [u8; 128] = [0; 128];

        username_buffer[0..username.len()].copy_from_slice(username);
        email_buffer[0..email.len()].copy_from_slice(email);

        Self {
            id,
            username: username_buffer,
            email: email_buffer,
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| {} | {} | {} |",
            self.id,
            String::from_utf8_lossy(&self.username),
            String::from_utf8_lossy(&self.email)
        )
    }
}
