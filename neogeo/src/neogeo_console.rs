
 pub mod neogeo {

    use crate::neogeo_file::neogeo::NeoGeoFile;
    use crate::neogeo_cartridge::neogeo::NeoGeoCartridge;

    pub const TICKS_PER_FRAME: u16 =  59736;

    pub struct NeoGeoConsole {
        cartridge: NeoGeoCartridge,
    }

    impl NeoGeoConsole {

        pub fn new (rom_file: String) -> NeoGeoConsole {
            let mut neogeo_file: NeoGeoFile = NeoGeoFile::new();
            neogeo_file.load_file(rom_file);

            Self{
                cartridge: NeoGeoCartridge::new(&neogeo_file),
            }
        }

        pub fn run_frame(&mut self) {
        
            let mut ticks: i32 = 0;

            while ticks < TICKS_PER_FRAME as i32 {

                self.cartridge.execute_tick();

            }       
        }

    }

}