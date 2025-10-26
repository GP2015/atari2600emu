mod bus;
mod cpu;
mod mapper;
mod state;

use crate::bus::Bus;
use crate::cpu::CPU;
use crate::state::State;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    program_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let state = State::new();

    let mut cart = mapper::m2k::Mapper2K::new(&args.program_path)?;
    let cpu = CPU::new(&mut cart)?;

    Ok(())
}
