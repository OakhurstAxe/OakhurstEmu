
use crate::m68000::{M68000, AddressBus24};

pub struct AddressOpCodes {}

pub trait AddressMethod: Sync + Send {
    fn execute(&self, cpu: &mut M68000, addr: &mut AddressBus24, step: u8) -> bool;
}


