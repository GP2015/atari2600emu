use crate::rom::ROM;
use anyhow::{Result, anyhow};

const PC_START_LOCATION: usize = 0xFFFC;

pub struct CPU {
    program_counter: u16,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    stack_pointer: u8,
}

impl CPU {
    pub fn new(rom: &ROM) -> Result<Self> {
        Ok(Self {
            program_counter: rom.read_u16(PC_START_LOCATION)?,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            status_register: 0,
            stack_pointer: 0,
        })
    }
}
