

pub mod emu_cpu{

    use crate::m6502::emu_cpu::M6502AddressBus::AddressRead;
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
        pub is_abs_y: bool,
        pub bus_state: M6502AddressBus,
    }

    impl AddressBus {
        fn new(address: u16, write: bool, byte: u8) -> AddressBus {
            Self {
                address,
                write,
                byte,
                is_accumulator: false,
                is_abs_y: false,
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

    pub struct M6502 {
        pub program_counter: u16,
        pub stack_pointer_page: u16,
        pub stack_pointer: u16,
        pub accumulator: u8,
        pub register_x: u8,
        pub register_y: u8,
        pub status_register: u8,
        pub lookup_address: AddressBus,
        pub op_cli_clear: bool,
        pub op_sei_set: bool,
        pub is_halted: bool,
        _debug: u8,
    }

    impl M6502 {
        fn new () -> M6502 {
            Self {
                program_counter: 0,
                stack_pointer_page: 0x100,
                stack_pointer: 0,
                accumulator: 0,
                register_x: 0,
                register_y: 0,
                status_register: 0,
                lookup_address: AddressBus::new(0, false, 0),
                op_cli_clear: false,
                op_sei_set: false,
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
        op_code: u16,
        op_step: u8,
        address_step: u8,
        runner_step: M6502RunnerStep,
        is_reset_set: bool,
        op_code_lookup: Vec<Box<dyn CpuOperation>>,
        op_code_ticks: OpCodeTicks,
        address_method_lookup: Vec<Box<dyn AddressMethod>>,
        tick_count: u8,
        is_nmi_set: bool,
        is_irq_set: bool,
        is_halt_set: bool,
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
                op_code: 0x100,
                op_step: 0,
                address_step: 0,
                runner_step: M6502RunnerStep::ReadPc,
                is_reset_set: true,
                op_code_lookup,
                op_code_ticks: OpCodeTicks::new(),
                address_method_lookup: AddressOpCodes::get_address_methods(),
                tick_count: 0,
                is_nmi_set: false,
                is_irq_set: false,
                is_halt_set: false,
                _debug: 0,
            }
        }

        pub fn set_nmi(&mut self, value: bool) {
            self.is_nmi_set = value;
        }

        pub fn set_irq(&mut self, value: bool) {
            self.is_irq_set = value;
        }

        pub fn halt_cpu(&mut self, halt: bool) {
            self.is_halt_set = halt;
        }

        pub fn is_cpu_halted(&mut self) -> bool {
            self.cpu.is_halted
        }

        fn reset(&mut self) {
            self.op_code = 0x100;
            self.runner_step = M6502RunnerStep::OpCodeStep;
            self.is_reset_set = false;
        }

        fn nmi(&mut self) {
            self.op_code = 0x101;
            self.runner_step = M6502RunnerStep::OpCodeStep;
            self.is_nmi_set = false;
        }

        fn irq(&mut self) {
            self.op_code = 0x102;
            self.runner_step = M6502RunnerStep::OpCodeStep;
        }

        fn read_program_counter(&mut self, addr: &mut AddressBus) {
            addr.is_accumulator = false;
            self.cpu.lookup_address.is_abs_y = false;
            if self.is_reset_set {
                self.reset();
            } else if self.is_nmi_set {
                self.nmi();
            } else if self.is_irq_set && OpCodesUtils::get_status_flag(&mut self.cpu, INTERRUPT_FLAG) == 0 {
                self.irq();
            } else {
                self.runner_step = M6502RunnerStep::AddressStep;
                //self.op_code_ticks.check_ticks(self.cpu.program_counter, self.op_code, self.tick_count);
                self.op_code = addr.byte as u16;
                //println!("PC: {:x}, opcode: {:x} ", self.cpu.program_counter, self.op_code);
                self.cpu.program_counter = self.cpu.program_counter.wrapping_add(1);
                self.tick_count = 0;
            }
        }

        pub fn execute_tick(&mut self, addr: &mut AddressBus) {
            
            self.tick_count += 1;
            
            if self.is_halt_set && self.cpu.is_halted {
                return;
            }

            if self.is_halt_set && !self.cpu.is_halted && !addr.write {
                self.cpu.is_halted = true;
                return;
            }

            if !self.is_halt_set && self.cpu.is_halted {
                self.cpu.is_halted = false;
                return;
            }

            if addr.bus_state == M6502AddressBus::CpuRead {
                self.cpu.lookup_address.byte = addr.byte;
                self.cpu.lookup_address.address = addr.address;
                self.cpu.lookup_address.is_accumulator = addr.is_accumulator;
                self.cpu.lookup_address.is_abs_y = addr.is_abs_y;
            }

            addr.bus_state = M6502AddressBus::None;

            if self.runner_step == M6502RunnerStep::AddressStepLoadByte {
                self.runner_step = M6502RunnerStep::OpCodeStep;
            }

            // Geting new op code
            if self.runner_step == M6502RunnerStep::ReadPc {
                self.read_program_counter(addr);

                // Calling CLI works AFTER next op is loaded
                if self.cpu.op_cli_clear {
                    self.cpu.op_cli_clear = false;
                    OpCodesUtils::set_status_flag(&mut self.cpu, INTERRUPT_FLAG, false);
                }
                if self.cpu.op_sei_set {
                    self.cpu.op_sei_set = false;
                    OpCodesUtils::set_status_flag(&mut self.cpu, INTERRUPT_FLAG, true);
                }
            }

            // Calling address/op tick
            if self.runner_step == M6502RunnerStep::AddressStep {
                if self.address_method_lookup[self.op_code as usize].execute(&mut self.cpu, addr, self.address_step) {
                    self.address_step = 0;
                    self.runner_step = M6502RunnerStep::AddressStepLoadByte;
                } else {
                    self.address_step += 1;
                }
            } 

            if self.runner_step == M6502RunnerStep::AddressStepLoadByte && !self.op_code_lookup[self.op_code as usize].needs_addr_byte(addr) {
                self.cpu.lookup_address.byte = addr.byte;
                self.runner_step = M6502RunnerStep::OpCodeStep;
            }

            if self.runner_step == M6502RunnerStep::OpCodeStep {
                if self.op_code_lookup[self.op_code as usize].execute(&mut self.cpu, addr, self.op_step) {
                    self.op_step = 0;
                    self.runner_step = M6502RunnerStep::OpCodeWrite;
                } else {
                    self.op_step += 1;
                }
            }

            if self.runner_step == M6502RunnerStep::OpCodeWrite && !addr.write {
                self.runner_step = M6502RunnerStep::ReadPc;
                addr.address = self.cpu.program_counter;
            }

        }

    }

}