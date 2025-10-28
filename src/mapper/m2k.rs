use super::*;
use anyhow::{Result, anyhow};

const ROM_SIZE: usize = 2048;
const CHIP_ENABLE_LINE: usize = 12;

pub struct Mapper2K {
    rom: Vec<u8>,
}

impl UseAsMapper for Mapper2K {
    fn new(program_path: &str) -> Result<Self> {
        let Ok(program) = std::fs::read(&program_path) else {
            return Err(anyhow!("Could not find valid program at {program_path}."));
        };

        if program.len() > ROM_SIZE {
            return Err(anyhow!("Program {program_path} is too large."));
        }

        Ok(Self { rom: program })
    }

    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        if address_bus.get_line(CHIP_ENABLE_LINE).unwrap() {
            let addr = address_bus.get_combined();
            let data = self.rom[addr % ROM_SIZE];
            data_bus.set_combined(data as usize);
        }
    }
}
