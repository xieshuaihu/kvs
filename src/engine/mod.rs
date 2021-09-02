use crate::Result;

pub mod sled;
pub mod kv;

pub use kv::KvStore;
pub use sled::SledKvsEngine;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;

    fn get(&mut self, key: String) -> Result<Option<String>>;

    fn remove(&mut self, key: String) -> Result<()>;
}