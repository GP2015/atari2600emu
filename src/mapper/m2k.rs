use super::*;
use anyhow::{Result, anyhow};

const ROM_SIZE: usize = 2048;

pub struct Mapper2K {
    rom: Vec<u8>,
}

impl Mapper2K {
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

impl Mapper for Mapper2K {
    fn read(&mut self, addr: usize) -> Result<u8> {
        let mirrored_addr = addr % ROM_SIZE;

        if mirrored_addr >= self.rom.len() {
            return Err(anyhow!(
                "Could not read out of bounds address {addr} (mirrored: {mirrored_addr})."
            ));
        };

        Ok(self.rom[mirrored_addr])
    }

    fn write(&mut self, _: usize) -> Result<()> {
        Err(anyhow!("Writing to 2K mapper is not allowed."))
    }
}
