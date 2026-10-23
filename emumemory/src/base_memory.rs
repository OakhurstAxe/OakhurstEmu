

pub trait BaseMemory {
    fn load_data(&mut self, data: &[u8]);

    fn read(&mut self, location: u16) -> u8;

    fn write(&mut self, location: u16, byte: u8);

    fn get_name(&self) -> String;// {
    //   String::from("Unnamed Memory")
    //};

    fn check_location(&self, location: u16);
}

