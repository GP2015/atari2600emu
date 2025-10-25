use anyhow::{Result, anyhow};

const BASE_ROM_SIZE: usize = 4096;

pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn new(program_path: &str) -> Result<Self> {
        let Ok(program) = std::fs::read(&program_path) else {
            return Err(anyhow!("Could not find valid program at {program_path}."));
        };

        if program.len() > BASE_ROM_SIZE {
            return Err(anyhow!("Program {program_path} is too large."));
        }

        Ok(Self { data: program })
    }

    pub fn read_byte(&self, index: usize) -> Result<u8> {
        if index >= self.data.len() {
            return Err(anyhow!("Could not read out of bounds index {index}."));
        };

        Ok(self.data[index])
    }

    pub fn read_bytes(&self, index: usize, count: usize) -> Result<Vec<u8>> {
        if index + count > self.data.len() {
            return Err(anyhow!(
                "Could not read out of bounds index {}.",
                index + count - 1
            ));
        };

        Ok(self.data[index..index + count].to_vec())
    }

    pub fn read_u16(&self, index: usize) -> Result<u16> {
        if index + 1 >= self.data.len() {
            return Err(anyhow!("Could not read out of bounds index {}.", index + 1));
        };

        let bytes = self.data[index..=index + 1].to_vec();
        Ok((u16::from(bytes[0]) << 8) | u16::from(bytes[1]))
    }
}
