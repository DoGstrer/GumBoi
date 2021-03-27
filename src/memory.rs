/*https://www.reddit.com/r/rust/comments/8k4vwc/rust_noob_using_a_value_from_an_array_as_an_index/
Arrays cannot be indexed with u16/u8 etc types which are dependednt on underlying architecture. usize is a good fit
 RUST DEF : fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
*/

const BOOT_ROM_SIZE: usize = 0x100;
const CATRIDGE_ROM_SIZE: usize = 0x8000;
const VRAM_SIZE: usize = 0x2000;
const EXTERNAL_RAM_SIZE: usize = 0x2000;
const RAM_SIZE: usize = 0x2000;
const HRAM: usize = 0x7F;
const OAM_RAM: usize = 0xA0;

use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub struct Memory {
    bank: [u8; 65536],
    pub boot_rom: [u8; BOOT_ROM_SIZE],
    catridge_rom: [u8; CATRIDGE_ROM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bank: [0u8; 65536],
            boot_rom: [0u8; BOOT_ROM_SIZE],
            catridge_rom: [0u8; CATRIDGE_ROM_SIZE],
        }
    }
    pub fn get_addr(&self, addr: u16) -> u8 {
        if addr > 0x00ff {
            self.bank[addr as usize]
        } else {
            match self.bank[0xff50] {
                0x0 => self.boot_rom[addr as usize],
                _ => self.bank[addr as usize],
            }
        }
    }
    pub fn set_addr(&mut self, addr: u16, val: u8) {
        if addr > 0x00ff {
            self.bank[addr as usize] = val;
        } else {
            match self.bank[0xff50] {
                0x0 => self.boot_rom[addr as usize] = val,
                _ => self.bank[addr as usize] = val,
            }
        }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
