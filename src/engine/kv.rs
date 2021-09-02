use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use rand::Rng;

use crate::engine::KvsEngine;

pub struct KvStore {
    path: PathBuf,
    file: File, 
    memory_date: HashMap<String, String>,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut memory_data = HashMap::with_capacity(100);
        let file = Self::create_new_file(path)?;
        let mut kv_store = KvStore {
            path: path.to_owned(),
            file: file,
            memory_date: memory_data,
        };

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name = entry.file_name().into_string()?;
            if file_type.is_file() && file_name.starts_with("kv") {
                let file = fs::File::open(entry.path())?;
                kv_store.recovery_from_file(file);
            }
        }

        Ok(kv_store)
    }

    fn create_new_file(path:&Path) -> Result<File, Box<dyn Error>> {
        let mut gen = rand::thread_rng();
        let file = loop {
          let number = gen.gen::<i32>();
          let path = PathBuf::from(path);
          path.push(format!("kv-{}", number));
          if !path.exists() {
            break fs::File::create(path)?;
          }
        };
        Ok(file)
    }

    fn recovery_from_file(&mut self, file: File) -> Result<(), Box<dyn Error>> {
        let file = BufReader::new(file);
        for line in file.lines() {
            let line = line?;
            let elements: Vec<&str> = line.split(" ").collect();
            match elements.first() {
                Some(&"SET") => {
                    if let Some(&key) = elements.get(1) {
                        if let Some(&value) = elements.get(2) {
                            self.memory_date.insert(key.to_owned(), value.to_owned());
                        }
                    }
                }
                Some(&"RM") => {
                    if let Some(key) = elements.get(1) {
                        self.memory_date.remove(key);
                    }
                }
                None => {}
            }
        }
        Ok(())
    }
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>> {
        self.file.write(format!("SET {} {}", &key, &value).as_bytes())?;
        self.memory_date.insert(key, value);
        
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>, Box<dyn Error>> {
        todo!()
    }

    fn remove(&mut self, key: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
