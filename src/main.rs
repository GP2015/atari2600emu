mod bus;
mod config;
mod cpu;
mod mapper;
mod state;

use crate::bus::Bus;
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
    let mut state = State::new();

    let mut address_bus = Bus::new(13);
    let mut data_bus = Bus::new(8);
    let mut rw_line = false;
    let mut phi2_line = false;
    let mut rdy_line = false;

    let mut cart = MapperKind::to_mapper(args.mapper, &args.program_path)?;
    let mut cpu = CPU::new();

    loop {
        cart.tick(&mut address_bus, &mut data_bus);
    }

    Ok(())
}
