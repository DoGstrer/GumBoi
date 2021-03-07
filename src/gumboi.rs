mod registers;
mod memory;
mod ppu;
mod sound;
mod cpu;
mod joypad;

use memory::Memory;
use registers::Registers;

pub struct GumBoi{
    memory: Memory,
    registers: Registers,
    cycle: usize
}

impl GumBoi{
    pub fn new() -> GumBoi{
        GumBoi{
            memory: Memory::new(),
            registers: Registers::new(),
            cycle: 0
        }
    }
    pub fn load(&self,dmg_rom: Vec<u8>,catridge_rom: Vec<u8>){
        //TODO
    }
    pub fn start(&self){
        //TODO
    }
    pub fn exit(&self){
        //TODO
    }
}