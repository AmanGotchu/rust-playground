use std::fs;
use std::path::{Path, PathBuf};

struct Database {
    data_dir: PathBuf,
    // In memory balanced tree (B-Tree or AVL Tree or ...)
}

impl Database {
    fn new(data_dir: PathBuf) -> Result<Database, std::io::Error> {
        if !data_dir.exists() {
            // Create data directory
            fs::create_dir_all(&data_dir)?;
        }

        Ok(Database {
            data_dir: data_dir,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        // Insert key-value pair into the database
    }

    fn get(&self, key: String) -> Option<String> {
        // Get value from the database
        None
    }

    fn delete(&mut self, key: String) {
        // Delete key-value pair from the database
    }
}

fn main() {
    let absolute_data_dir= Path::new("./data").canonicalize().unwrap();
    let mut db = Database::new(absolute_data_dir);
}
