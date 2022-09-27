#![cfg_attr(not(test), no_std)]

use tock_registers::interfaces::{Readable, Writeable};

pub mod hw;

pub struct Pl011<'a> {
    reg: &'a hw::Registers,
}

impl Pl011<'_> {
    pub unsafe fn new(addr: usize) -> Self {
        Self {
            reg: &*(addr as *const hw::Registers),
        }
    }

    pub fn write(&self, buf: &[u8]) {
        for x in buf.iter().copied() {
            while self.reg.fr.read(hw::FR::TXFF) != 0 {}
            self.reg.dr.set(x as u16);
        }
    }
}
