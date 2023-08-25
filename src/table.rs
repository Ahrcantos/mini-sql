use std::fmt::{self, Display, Formatter};

use crate::pager::{Page, Pager};

pub struct Table {
    rows: Vec<Row>,
    pager: Pager,
    count: usize,
}

impl Table {
    const ROWS_PER_PAGE: usize = Page::PAGE_SIZE / Row::OPTIONAL_ROW_SIZE;

    pub fn new() -> Self {
        let pager = Pager::new("./data.hex");

        Self {
            rows: Vec::new(),
            pager,
            count: 0,
        }
    }

    pub fn insert(&mut self, row: Row) {
        let page_index = self.count / Self::ROWS_PER_PAGE;
        let page = self.pager.get_page(page_index);
        let mut page = page.lock().unwrap();

        let index = self.count % Self::ROWS_PER_PAGE;
        let offset_start = Row::OPTIONAL_ROW_SIZE * index;
        let offset_end = offset_start + Row::OPTIONAL_ROW_SIZE;
        let data = row.serialize();
        page[offset_start..offset_end].copy_from_slice(data.as_slice());
        self.count += 1;
    }

    pub fn flush(&mut self) {
        self.pager.flush();
    }

    pub fn get_row(&mut self, index: usize) -> Option<Row> {
        let page_index = index / Self::ROWS_PER_PAGE;
        let page = self.pager.get_page(page_index);
        let page = page.lock().unwrap();
        Row::from_page(&page, index & Self::ROWS_PER_PAGE)
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
    pub const OPTIONAL_ROW_SIZE: usize = std::mem::size_of::<Option<Row>>();

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

    pub fn from_page(page: &Page, index: usize) -> Option<Self> {
        let offset_start = index * Self::OPTIONAL_ROW_SIZE;

        if page[offset_start..offset_start + 4] != [0xff, 0xff, 0xff, 0xff] {
            return None;
        }

        let id: u32 = {
            let b1: u32 = page[offset_start + 7].into();
            let b2: u32 = page[offset_start + 6].into();
            let b3: u32 = page[offset_start + 5].into();
            let b4: u32 = page[offset_start + 4].into();

            b1 + (b2 * 256) + (b3 * (256 * 256)) + (b4 * (256 * 256 * 256))
        };

        let username = {
            let mut data = [0u8; 128];
            data.copy_from_slice(&page[offset_start + 8..offset_start + 136]);
            data
        };

        let email = {
            let mut data = [0u8; 128];
            data.copy_from_slice(&page[offset_start + 136..offset_start + 264]);
            data
        };

        Some(Self {
            id,
            username,
            email,
        })
    }

    pub fn serialize(&self) -> Box<[u8; Self::OPTIONAL_ROW_SIZE]> {
        let mut data = Box::new([0u8; Self::OPTIONAL_ROW_SIZE]);

        data[0..4].copy_from_slice(&[0xff, 0xff, 0xff, 0xff]); // Any non-zero value
        data[4..8].copy_from_slice(&self.id.to_be_bytes());
        data[8..136].copy_from_slice(&self.username);
        data[136..264].copy_from_slice(&self.email);

        data
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_row_serialize() {
        let row = Row::new(1, "Username", "email");

        let data = row.serialize();

        assert_eq!(&data[0..4], &[0xff, 0xff, 0xff, 0xff]);
    }
}
