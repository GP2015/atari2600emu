mod bus;
mod config;
mod cpu;
mod mapper;
mod state;

use crate::cpu::CPU;
use crate::state::State;
use anyhow::Result;
use clap::Parser;
use mapper::MapperKind;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    program_path: String,
    mapper: MapperKind,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = config::generate_config()?;
    let state = State::new();

    let mut cart = MapperKind::to_mapper(args.mapper, &args.program_path)?;
    let cpu = CPU::new(&mut *cart)?;

    Ok(())
}
