#![allow(unused)]

mod cpu;
mod interrupt;
mod joypad;
mod memory;
mod ppu;
mod registers;
mod sound;
mod timer;

use cpu::CPUState;
use cpu::CPU;
use memory::Memory;
use ppu::PPU;
use registers::Flag;
use registers::Registers;

use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
enum GumBoiState {
    Active,
    Halt,
    Exit,
}

pub struct GumBoi {
    cpu: CPU,
    ppu: PPU,
    memory: Rc<RefCell<Memory>>,
    cycle: usize,
    state: GumBoiState,
}

impl GumBoi {
    pub fn new() -> GumBoi {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let memory_cpu = Rc::clone(&memory);
        let memory_ppu = Rc::clone(&memory);

        GumBoi {
            cpu: CPU::new(memory_cpu),
            ppu: PPU::new(memory_ppu),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        }
    }
    pub fn load(&mut self, catridge_rom: Vec<u8>) {
        //Load Catridge into GumBoi ROM
        let mut addr: u16 = 0x0000;
        for byte in catridge_rom {
            self.memory.borrow_mut().set_addr(addr, byte);
            addr += 1;
        }
    }
    pub fn start(&mut self) {
        //self.memory.set_addr(0xff44,0x90);
        while self.cpu.get_state() == CPUState::Active {
            self.cpu.execute();
        }
    }
    pub fn exit(&self) {
        //TODO
    }
}
