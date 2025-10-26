use anyhow::Result;

pub mod m2k;
pub mod m4k;

pub trait Mapper {
    fn read(&mut self, addr: usize) -> Result<u8>;
    fn write(&mut self, addr: usize) -> Result<()>;
}
