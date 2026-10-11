

pub mod emu_cpu{

    use crate::m6502_address::m6502_address::{*};
    use crate::m6502_opcodes::m6502_opcodes::{*};
    use crate::m6502_opcode_ticks::m6502_opcodeticks::{*};

    pub const CARRY_FLAG: u8     = 0x01;
    pub const ZERO_FLAG: u8      = 0x02;
    pub const INTERRUPT_FLAG: u8 = 0x04;
    pub const DECIMAL_MODE: u8   = 0x08;
    pub const BREAK_COMMAND: u8  = 0x10;
    pub const IGNORED: u8        = 0x20;
    pub const OVERFLOW_FLAG: u8  = 0x40;
    pub const NEGATIVE_FLAG: u8  = 0x80;

    pub struct AddressBus {
        pub address: u16,
        pub write: bool,
        pub byte: u8,
        pub is_accumulator: bool,
        pub bus_state: M6502AddressBus,
    }

    impl AddressBus {
        fn new(address: u16, write: bool, byte: u8) -> AddressBus {
            Self {
                address,
                write,
                byte,
                is_accumulator: false,
                bus_state: M6502AddressBus::None,
            }
        }

        pub fn cpu_read(&mut self, address: u16) {
            self.address = address;
            self.bus_state = M6502AddressBus::CpuRead;
        }

        pub fn addr_read(&mut self, address: u16) {
            self.address = address;
            self.bus_state = M6502AddressBus::AddressRead;
        }

        pub fn addr_write(&mut self, address: u16, byte: u8) {
            self.address = address;
            self.byte = byte;
            self.write = true;
            self.bus_state = M6502AddressBus::AddressRead;
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct M6502Status {
        pub program_counter: u16,
        pub op_code: u16,
        pub status_byte: u8,
        pub is_op_code_updated: bool,
    }
    impl M6502Status {
        pub fn new() -> M6502Status {
            Self {
                program_counter: 0,
                op_code: 0x100,
                status_byte: 0,
                is_op_code_updated: false,
            }
        }
    }
    
    pub struct M6502 {
        pub prefetch_state: M6502Status,
        pub current_state: M6502Status,
        pub stack_pointer_page: u16,
        pub stack_pointer: u16,
        pub accumulator: u8,
        pub register_x: u8,
        pub register_y: u8,
        pub status_register: u8,
        pub lookup_address: AddressBus,
        pub op_cli_clear: bool,
        pub op_sei_set: bool,
        pub is_halt_set: bool,
        pub is_nmi_set: bool,
        pub is_irq_set: bool,
        pub is_halted: bool,
        _debug: u8,
    }

    impl M6502 {
        fn new () -> M6502 {
            Self {
                prefetch_state: M6502Status::new(),
                current_state: M6502Status::new(),
                stack_pointer_page: 0x100,
                stack_pointer: 0,
                accumulator: 0,
                register_x: 0,
                register_y: 0,
                status_register: 0,
                lookup_address: AddressBus::new(0, false, 0),
                op_cli_clear: false,
                op_sei_set: false,
                is_halt_set: false,
                is_nmi_set: false,
                is_irq_set: false,
                is_halted: false,
                _debug: 0,
            }
        }
    }

    #[derive(PartialEq)]
    pub enum M6502Version {
        AtariVcs,
        Nes
    }

    #[derive(PartialEq)]
    enum M6502RunnerStep {
        ReadPc,
        AddressStep,
        AddressStepLoadByte,
        OpCodeStep,
        OpCodeWrite,
    }

    #[derive(PartialEq)]
    pub enum M6502AddressBus {
        ReadOpCode,
        AddressRead,
        CpuRead,
        None
    }

    pub struct M6502Runner {
        cpu: M6502,
        op_step: u8,
        address_step: u8,
        runner_step: M6502RunnerStep,
        is_reset_set: bool,
        op_code_lookup: Vec<Box<dyn CpuOperation>>,
        op_code_ticks: OpCodeTicks,
        address_method_lookup: Vec<Box<dyn AddressMethod>>,
        tick_count: u8,
        _debug: u8,
    }

    impl M6502Runner {
        pub fn new (version: M6502Version) -> M6502Runner {

            let mut cpu = M6502::new();
            let mut op_code_lookup = OpCodesUtils::get_opcodes();

            if version == M6502Version::AtariVcs {
                cpu.stack_pointer_page = 0x00;
            } else if version == M6502Version::Nes {
                op_code_lookup[0x61] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x65] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x69] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x6d] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x71] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x75] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x79] = Box::new(CpuOpAdcNoDecimal {});
                op_code_lookup[0x7d] = Box::new(CpuOpAdcNoDecimal {});

                op_code_lookup[0xe1] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xe5] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xe9] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xeb] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xed] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xf1] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xf5] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xf9] = Box::new(CpuOpSbcNoDecimal {});
                op_code_lookup[0xfd] = Box::new(CpuOpSbcNoDecimal {});
            }

            Self {
                cpu,
                op_step: 0,
                address_step: 0,
                runner_step: M6502RunnerStep::ReadPc,
                is_reset_set: true,
                op_code_lookup,
                op_code_ticks: OpCodeTicks::new(),
                address_method_lookup: AddressOpCodes::get_address_methods(),
                tick_count: 0,
                _debug: 0,
            }
        }

        pub fn set_nmi(&mut self, value: bool) {
            if value {
                self.cpu.is_nmi_set = value;
            }
        }

        pub fn set_irq(&mut self, value: bool) {
            self.cpu.is_irq_set = value;
        }

        pub fn halt_cpu(&mut self, halt: bool) {
            self.cpu.is_halt_set = halt;
        }

        pub fn is_cpu_halted(&mut self) -> bool {
            self.cpu.is_halted
        }

        fn reset(&mut self) {
            self.cpu.current_state.op_code = 0x100;
            self.runner_step = M6502RunnerStep::OpCodeStep;
            self.is_reset_set = false;
        }

        fn nmi(&mut self) {
            self.cpu.current_state.op_code = 0x101;
            self.runner_step = M6502RunnerStep::OpCodeStep;
            self.cpu.is_nmi_set = false;
        }

        fn irq(&mut self) {
            self.cpu.current_state.op_code = 0x102;
            self.runner_step = M6502RunnerStep::OpCodeStep;
        }

        fn read_program_counter(&mut self, addr: &mut AddressBus) {
            addr.is_accumulator = false;
            if self.is_reset_set {
                self.reset();
            } else if self.cpu.is_nmi_set {
                self.cpu.is_nmi_set = false;
                self.nmi();
            } else if self.cpu.is_irq_set && (self.cpu.current_state.status_byte & INTERRUPT_FLAG) == 0 {
                self.irq();
            } else {
                self.runner_step = M6502RunnerStep::AddressStep;
                //self.op_code_ticks.check_ticks(self.cpu.current_state.program_counter, self.cpu.current_state.op_code, self.tick_count);
                self.cpu.current_state.op_code = self.cpu.prefetch_state.op_code;
                //println!("PC: {:x}, opcode: {:x} ", self.cpu.current_state.program_counter, self.cpu.current_state.op_code);
                self.cpu.current_state.program_counter = self.cpu.current_state.program_counter.wrapping_add(1);
                self.tick_count = 0;
            }
        }

        fn is_halted(&mut self, addr: &mut AddressBus) -> bool {
            if self.cpu.is_halt_set && self.cpu.is_halted {
                return true;
            }

            if self.cpu.is_halt_set && !self.cpu.is_halted && !addr.write {
                self.cpu.is_halted = true;
                return true;
            }

            if !self.cpu.is_halt_set && self.cpu.is_halted {
                self.cpu.is_halted = false;
                return true;
            }

            false
        }

        pub fn execute_tick(&mut self, addr: &mut AddressBus) {
            
            self.tick_count = self.tick_count.wrapping_add(1);
            
            if self.is_halted(addr) {
                return;
            }

            if addr.bus_state == M6502AddressBus::CpuRead {
                self.cpu.lookup_address.byte = addr.byte;
                self.cpu.lookup_address.address = addr.address;
                self.cpu.lookup_address.is_accumulator = addr.is_accumulator;
            }

            if addr.bus_state == M6502AddressBus::ReadOpCode {
                self.cpu.prefetch_state.op_code = addr.byte as u16;
                self.cpu.prefetch_state.is_op_code_updated = true;
            }

            addr.bus_state = M6502AddressBus::None;

            if self.runner_step == M6502RunnerStep::AddressStepLoadByte {
                self.runner_step = M6502RunnerStep::OpCodeStep;
            }

            // Geting new op code
            if self.runner_step == M6502RunnerStep::ReadPc &&
                self.cpu.prefetch_state.is_op_code_updated &&
                self.cpu.prefetch_state.program_counter == self.cpu.current_state.program_counter {
                self.read_program_counter(addr);
            }

            // Prefetch uses status register before opcode tick
            self.cpu.prefetch_state.status_byte = self.cpu.status_register;
            if self.cpu.prefetch_state.program_counter != self.cpu.current_state.program_counter {
                self.cpu.prefetch_state.program_counter = self.cpu.current_state.program_counter;
                self.cpu.prefetch_state.is_op_code_updated = false;
            }

            // Calling address/op tick
            if self.runner_step == M6502RunnerStep::AddressStep {
                if self.address_method_lookup[self.cpu.current_state.op_code as usize].execute(&mut self.cpu, addr, self.address_step) {
                    self.address_step = 0;
                    self.runner_step = M6502RunnerStep::AddressStepLoadByte;
                } else {
                    self.address_step += 1;
                }
            } 

            if self.runner_step == M6502RunnerStep::AddressStepLoadByte && !self.op_code_lookup[self.cpu.current_state.op_code as usize].needs_addr_byte(addr) {
                self.cpu.lookup_address.byte = addr.byte;
                self.runner_step = M6502RunnerStep::OpCodeStep;
            }

            if self.runner_step == M6502RunnerStep::OpCodeStep {
                if self.op_code_lookup[self.cpu.current_state.op_code as usize].execute(&mut self.cpu, addr, self.op_step) {
                    self.op_step = 0;
                    self.runner_step = M6502RunnerStep::OpCodeWrite;
                } else {
                    self.op_step += 1;
                }
            }

            if self.runner_step == M6502RunnerStep::OpCodeWrite && !addr.write {
                self.runner_step = M6502RunnerStep::ReadPc;
                addr.address = self.cpu.current_state.program_counter;
            }

            if addr.bus_state == M6502AddressBus::None {
                addr.address = self.cpu.prefetch_state.program_counter;
                addr.bus_state = M6502AddressBus::ReadOpCode;
            }

        }

    }

}