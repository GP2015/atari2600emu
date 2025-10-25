mod bus;
mod cpu;
mod rom;
mod state;

use crate::bus::Bus;
use crate::cpu::CPU;
use crate::rom::ROM;
use crate::state::State;
use anyhow::{Result, anyhow};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    program_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let state = State::new();

    let rw_line = Bus::new(false);
    let data_bus = Bus::<u8>::new(0);
    let addr_bus = Bus::<u16>::new(0);

    let rom = ROM::new(&args.program_path)?;
    let cpu = CPU::new(&rom)?;

    Ok(())
}
