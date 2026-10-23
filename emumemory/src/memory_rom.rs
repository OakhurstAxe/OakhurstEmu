
use crate::base_memory::BaseMemory;


pub trait MemoryRom: BaseMemory {
    fn write(&mut self, _location: u16, _byte: u8) {
        panic!("Cannot write to ROM");
    }
}
    
