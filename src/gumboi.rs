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
    boot_rom: Vec<u8>,
    registers: Registers,
    cycle: usize
}

impl GumBoi{
    pub fn new(boot_rom: Vec<u8>) -> GumBoi{
        GumBoi{
            memory: Memory::new(),
            boot_rom: boot_rom,
            registers: Registers::new(),
            cycle: 0
        }
    }
    pub fn load(&mut self,catridge_rom: Vec<u8>){
        //Load Catridge into GumBoi ROM
        let mut addr: u16 = 0x0000;
        for byte in catridge_rom{
            self.memory.set_addr(addr,byte);
            addr+=1;
        }
    }
    pub fn start(&mut self){
        //TODO
    }
    pub fn exit(&self){
        //TODO
    }
}