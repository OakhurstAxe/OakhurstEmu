
pub mod neogeo {

    use crate::neogeo_file::neogeo::NeoGeoFile;
    
    pub struct NeoGeoCartridge {
        pub cpu_prog_rom: Vec<u8>,
    }

    impl NeoGeoCartridge {

        pub fn new(ineogeo: &NeoGeoFile) -> NeoGeoCartridge {
            Self {
                cpu_prog_rom: ineogeo.get_prog_rom_data(),
            }
        }

        pub fn execute_tick(&mut self)
        {

        }
        
        pub fn cpu_read(&self, location: u16) -> u8 {
            self.cpu_prog_rom[location as usize]
        }

        pub fn cpu_write(&mut self, location: u16, byte: u8) {
            panic!("Write not allowed to cartridge");
        }
    }

}