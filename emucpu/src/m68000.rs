


use crate::m68000_opcodes::{*};
use crate::m68000_address::{*};

pub const EXTEND_FLAG: u8   = 0x10;
pub const NEGATIVE_FLAG: u8 = 0x08;
pub const ZERO_FLAG: u8     = 0x04;
pub const OVERFLOW_FLAG: u8 = 0x02;
pub const CARRY_FLAG: u8    = 0x01;

pub struct M68000 {
    pub d_registers: [u32; 8],
    pub a_registers: [u32; 8],
    pub user_stack_pointer: u32,
    pub program_counter: u32,
    pub condition_code_register: u16,
    _debug: u8,
}

impl M68000 {
    fn new () -> M68000 {
        Self {
            d_registers: [0; 8],
            a_registers: [0; 8],
            user_stack_pointer: 0,
            program_counter: 0,
            condition_code_register: 0,
            _debug: 0,
        }
    }
}

#[derive(Default)]
pub struct AddressBus24 {
    pub address: u32,
    pub write: bool,
    pub byte: u16,
}

impl AddressBus24 {
    fn new(address: u32, write: bool, byte: u16) -> AddressBus24 {
        Self {
            address,
            write,
            byte,
        }
    }
}

#[derive(PartialEq)]
enum M68000RunnerStep {
    ReadPc,
    AddressStep,
    AddressStepLoadByte,
    OpCodeStep,
    OpCodeWrite,
}


pub struct M68000Runner {
    cpu: M68000,
    op_code: u32,
    op_step: u8,
    address_step: u8,
    is_reset_set: bool,
    op_code_lookup: Vec<Box<dyn CpuOperation>>,
    address_method_lookup: Vec<Box<dyn AddressMethod>>,
    tick_count: u8,
    runner_step: M68000RunnerStep,
    _debug: u8,
}

impl M68000Runner {

    pub fn new () -> M68000Runner {
        Self {
            cpu: M68000::new(),
            op_code: 0,
            op_step: 0,
            address_step: 0,
            is_reset_set: true,
            op_code_lookup: Vec::new(),
            address_method_lookup: Vec::new(),
            tick_count: 0,
            runner_step: M68000RunnerStep::ReadPc,
            _debug: 0,
        }
    }

    fn reset(&mut self) {
        self.op_code = 0x100;
        self.runner_step = M68000RunnerStep::OpCodeStep;
        self.is_reset_set = false;
    }

    fn read_program_counter(&mut self, addr: &mut AddressBus24) {
        addr.address = self.cpu.program_counter;
        addr.write = false;
        self.runner_step = M68000RunnerStep::AddressStep;
    }

    pub fn execute_tick(&mut self, addr: &mut AddressBus24) {

        if self.runner_step == M68000RunnerStep::ReadPc {
            self.read_program_counter(addr);
        }

    }


}

