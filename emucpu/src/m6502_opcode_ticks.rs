
use std::range::RangeInclusive;


#[derive(Default)]
pub struct OpCodeTicks {
    pub op_code_ticks: Vec<RangeInclusive<u8>>,
}

impl OpCodeTicks {
    pub fn new () -> OpCodeTicks {
        
        Self {
            op_code_ticks: OpCodeTicks::set_ticks(),
        }
    }

    pub fn check_ticks(&self, program_counter: u16, op_code: u16, actual_ticks: u8) {

        if !self.op_code_ticks[op_code as usize].contains(&actual_ticks) {
            println!("PC: {:x}, opcode: {:x} tickcount: {} expected: {}", program_counter, op_code, actual_ticks, self.op_code_ticks[op_code as usize].start);
        }
    }

    fn set_ticks() -> Vec<RangeInclusive<u8>> {
        let mut op_code_ticks: Vec<RangeInclusive<u8>> = Vec::with_capacity(0x200);
        for _i in 0..0x200 {
            op_code_ticks.push(RangeInclusive{ start: 0, last: 0 });
        }

        op_code_ticks[0x01] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0x02] = RangeInclusive{ start: 7, last: 7 };
        op_code_ticks[0x05] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x06] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0x08] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x09] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x0a] = RangeInclusive{ start: 2, last: 2 };

        op_code_ticks[0x10] = RangeInclusive{ start: 2, last: 3 };
        op_code_ticks[0x11] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0x15] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x18] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x19] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0x20] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0x24] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x25] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x26] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0x29] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x2a] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x2c] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0x30] = RangeInclusive{ start: 2, last: 3 };
        op_code_ticks[0x35] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x38] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x3d] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0x45] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x46] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0x48] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x49] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x4a] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x4c] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x4e] = RangeInclusive{ start: 6, last: 6 };

        op_code_ticks[0x50] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x56] = RangeInclusive{ start: 6, last: 6 };

        op_code_ticks[0x60] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0x65] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x66] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0x68] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x69] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x6a] = RangeInclusive{ start: 2, last: 2 };

        op_code_ticks[0x70] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x75] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x78] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x79] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x7d] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0x84] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x85] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x86] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0x88] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x8a] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x8c] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x8d] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0x90] = RangeInclusive{ start: 2, last: 3 };
        op_code_ticks[0x91] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0x94] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x95] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x96] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0x98] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0x99] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0x9a] = RangeInclusive{ start: 2, last: 2 };

        op_code_ticks[0xa0] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xa1] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0xa2] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xa4] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xa5] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xa6] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xa8] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xa9] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xaa] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xad] = RangeInclusive{ start: 4, last: 4 }; 
        op_code_ticks[0xae] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0xb0] = RangeInclusive{ start: 2, last: 3 };
        op_code_ticks[0xb1] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0xb4] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xb5] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xb6] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xb9] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xba] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xbc] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xbd] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xbe] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0xc0] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xc4] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xc5] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xc6] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0xc8] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xc9] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xca] = RangeInclusive{ start: 2, last: 2 };

        op_code_ticks[0xd0] = RangeInclusive{ start: 2, last: 3 };
        op_code_ticks[0xd1] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0xd5] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xd6] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0xd8] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xd9] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xdd] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0xe0] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xe4] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xe5] = RangeInclusive{ start: 3, last: 3 };
        op_code_ticks[0xe6] = RangeInclusive{ start: 5, last: 5 };
        op_code_ticks[0xe8] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xe9] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xea] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xec] = RangeInclusive{ start: 4, last: 4 };

        op_code_ticks[0xf0] = RangeInclusive{ start: 2, last: 3 };
        op_code_ticks[0xf6] = RangeInclusive{ start: 6, last: 6 };
        op_code_ticks[0xf8] = RangeInclusive{ start: 2, last: 2 };
        op_code_ticks[0xf9] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xfd] = RangeInclusive{ start: 4, last: 4 };
        op_code_ticks[0xff] = RangeInclusive{ start: 7, last: 7 };

        op_code_ticks
    }
}
