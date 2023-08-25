use std::fmt::{self, Debug, Formatter};
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub struct Page(Box<[u8; Self::PAGE_SIZE]>);

impl Page {
    pub const PAGE_SIZE: usize = 4096;
}

impl Debug for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.0[0..128])
    }
}

impl Deref for Page {
    type Target = [u8; Self::PAGE_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Page {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const MAX_PAGES: usize = 100;
pub struct Pager {
    file: File,
    pages: [Option<Arc<Mutex<Page>>>; MAX_PAGES],
}

impl Pager {
    pub fn new<P>(filepath: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new().write(true).read(true).open(&filepath);

        let file = match file {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let mut file = File::create(&filepath).unwrap();

                for _ in 0..MAX_PAGES {
                    let buffer = [0; Page::PAGE_SIZE];
                    file.write_all(&buffer).unwrap();
                }

                file
            }
            Err(_) => {
                panic!("idk")
            }
        };

        let pages = std::array::from_fn(|_| None);

        Self { file, pages }
    }

    pub fn get_page(&mut self, page_index: usize) -> Arc<Mutex<Page>> {
        if page_index > MAX_PAGES {
            panic!("Index too large");
        }

        let page = self.pages.get(page_index).and_then(|p| p.as_ref());

        if let Some(page) = page {
            return page.clone();
        }

        let offset: u64 = (Page::PAGE_SIZE * page_index)
            .try_into()
            .expect("Cant convert offset");

        self.file.seek(SeekFrom::Start(offset)).unwrap();

        let mut buffer = [0; Page::PAGE_SIZE];

        self.file.read_exact(&mut buffer).unwrap();

        let page = Page(Box::new(buffer));
        let page = Mutex::new(page);
        let page = Arc::new(page);

        self.pages[page_index] = Some(page.clone());

        page
    }

    pub fn flush(&mut self) {
        for page_index in 0..MAX_PAGES {
            let page = self.pages.get(page_index).and_then(|p| p.as_ref());

            match page {
                Some(page) => {
                    let offset: u64 = (Page::PAGE_SIZE * page_index)
                        .try_into()
                        .expect("Cant convert offset");
                    self.file.seek(SeekFrom::Start(offset)).unwrap();

                    let page = page.lock().unwrap();
                    self.file.write_all(page.deref().deref()).unwrap();
                }
                None => {
                    continue;
                }
            }
        }

        self.file.flush().unwrap();
    }
}
