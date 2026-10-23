
use std::io::Read;

pub struct NeoGeoFile {
    prog_rom_data: Vec<u8>,
}

impl NeoGeoFile {

    pub fn new() -> NeoGeoFile {
        Self {
            prog_rom_data: vec![0; 0],
        }
    }

    pub fn get_prog_rom_data(&self) -> Vec<u8> {
        self.prog_rom_data.clone()
    }

    pub fn load_file(&mut self, rom_file: String) {

        let zipfile = match std::fs::File::open(&rom_file) {
            Ok(f) => f,
            Err(e) => {
                panic!("Error: failed to open {:?}: {e}", rom_file);
            }
        };

        let mut archive = zip::ZipArchive::new(zipfile).unwrap();

        for file in archive.file_names() {
            println!("File in zip: {}", "file");
        }

        archive.by_name("005-pg1.p1").unwrap().read_to_end(&mut self.prog_rom_data).unwrap();

        println!("File loaded");
    }

}
