pub mod reads;
pub mod regs;

use crate::cpu::{
    reads::{CpuAllReads, CpuLineReads},
    regs::CpuRegs,
};
use common::{
    cond::{base::BaseCondition, check::CheckIs},
    line::{multi::BusDriveState, single::DriveState},
    mux_matches,
    read::single::SingleRead,
    signal::LineSignal,
};
use emucore_macros::mnem_pat;

macro_rules! db {
    ($r:ident, $($v:ident),+) => {
        $r.line.db.is_any(mnem_pat!($($v),+).iter())
    };
}

macro_rules! ic {
    ($r:ident, $($v:literal),+) => {
        ($(BaseCondition::from($r.reg.instr_cycle[$v]))|+)
    };
}

macro_rules! db_ic {
    ($r:ident, $($db:ident),+, $($ic:literal),+) => {
        (db!($r, $($db),+) & ic!($r, $($ic),+))
    };
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cpu {
    pub phi2_out: DriveState,
    pub a_out: BusDriveState<13>,
    pub db_out: BusDriveState<8>,
    pub rw_out: DriveState,
    reg: CpuRegs,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            phi2_out: DriveState::none_enabled(),
            a_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            db_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            rw_out: LineSignal::HighZ.into(),
            reg: CpuRegs::new(),
        }
    }

    pub fn handle_rising_edge(&mut self, line_reads: CpuLineReads) {
        let r = CpuAllReads::new(line_reads, self.reg.clone());
        todo!()
    }

    fn update_s(&mut self, r: &CpuAllReads) {
        self.reg.s = mux_matches!(
            (db_ic!(r, Txs, 1), &|| r.reg.x.clone()),
            (
                db_ic!(r, Pha, Php, Brk, 2) | db_ic!(r, Brk, Jsr, 3, 4),
                &|| r.reg.s.decremented()
            ),
            (
                db_ic!(r, Pla, Plp, Rti, Rts, 2) | db_ic!(r, Rti, Rts, 3) | db_ic!(r, Rti, 4),
                &|| r.reg.s.incremented()
            ),
            &|| r.reg.s.clone()
        );
    }

    // Girl you need to keep track of what instruction you're on!!!
    // You can't just use the data bus.

    pub fn handle_falling_edge(&mut self, line_reads: CpuLineReads) {
        let r = CpuAllReads::new(line_reads, self.reg.clone());
        self.update_s(&r);
        todo!()
    }
}
