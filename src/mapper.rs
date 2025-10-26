use anyhow::Result;
use serde::Deserialize;

pub mod m2k;
pub mod m4k;

#[derive(Deserialize, Debug)]
pub enum MapperType {
    M2K,
    M4K,
}

pub trait Mapper {
    fn read(&mut self, addr: usize) -> Result<u8>;
    fn write(&mut self, addr: usize) -> Result<()>;
}
