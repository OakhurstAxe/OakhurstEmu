
pub mod m68000_opcodes {

    use crate::m68000::emu_cpu::{M68000, AddressBus24};

    pub struct OpCodesUtils {}

    pub trait CpuOperation: Sync + Send {
        fn execute(&self, cpu: &mut M68000, addr: &mut AddressBus24, step: u8) -> bool;
    }

}

