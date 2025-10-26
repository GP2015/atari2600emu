use super::*;
use anyhow::{Result, anyhow};

const ROM_SIZE: usize = 4096;

pub struct Mapper4K {
    rom: Vec<u8>,
}

impl Mapper4K {
    pub fn new(program_path: &str) -> Result<Self> {
        let Ok(program) = std::fs::read(&program_path) else {
            return Err(anyhow!("Could not find valid program at {program_path}."));
        };

        if program.len() > ROM_SIZE {
            return Err(anyhow!("Program {program_path} is too large."));
        }

        Ok(Self { rom: program })
    }
}

impl Mapper for Mapper4K {
    fn read(&mut self, addr: usize) -> Result<u8> {
        if addr >= self.rom.len() {
            return Err(anyhow!("Could not read out of bounds address {addr}."));
        };

        Ok(self.rom[addr])
    }

    fn write(&mut self, _: usize) -> Result<()> {
        Err(anyhow!("Writing to 4K mapper is not allowed."))
    }
}
