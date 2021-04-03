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

const BOOT_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];

use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub struct Memory {
    bank: [u8; 65536],
    boot_rom: [u8; BOOT_ROM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bank: [0u8; 65536],
            boot_rom: BOOT_ROM,
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
    //TODO : Better API
    pub fn load_cartridge(&mut self, cartridge_rom: Vec<u8>) {
        match cartridge_rom {
            cartridge_rom if cartridge_rom.len() < 0x8000 => {
                for (index, val) in cartridge_rom.iter().enumerate() {
                    self.bank[index] = *val;
                }
            }
            _ => panic!("Inserted Cartridge Exceeds 32Kb!"),
        };
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
