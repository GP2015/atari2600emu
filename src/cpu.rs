use crate::mapper::Mapper;
use anyhow::Result;

const PC_RESET_ADDR_HIGH_BYTE: usize = 0xFFFC;
const PC_RESET_ADDR_LOW_BYTE: usize = 0xFFFD;

const ACCUMULATOR_RESET_VALUE: u8 = 0x00;
const X_REGISTER_RESET_VALUE: u8 = 0x00;
const Y_REGISTER_RESET_VALUE: u8 = 0x00;
const STATUS_REGISTER_RESET_VALUE: u8 = 0x00;
const STACK_POINTER_RESET_VALUE: u8 = 0xFD;

pub struct CPU {
    program_counter: u16,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    stack_pointer: u8,
}

impl CPU {
    pub fn new<T: Mapper>(cart: &mut T) -> Result<Self> {
        let pc_high_byte = cart.read(PC_RESET_ADDR_HIGH_BYTE)?;
        let pc_low_byte = cart.read(PC_RESET_ADDR_LOW_BYTE)?;

        let pc_start_addr = ((pc_high_byte as u16) << 8) | (pc_low_byte as u16);

        Ok(Self {
            program_counter: pc_start_addr,
            accumulator: ACCUMULATOR_RESET_VALUE,
            x_register: X_REGISTER_RESET_VALUE,
            y_register: Y_REGISTER_RESET_VALUE,
            status_register: STATUS_REGISTER_RESET_VALUE,
            stack_pointer: STACK_POINTER_RESET_VALUE,
        })
    }
}
