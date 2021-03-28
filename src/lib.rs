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

use std::sync::{Mutex,Arc};
use std::convert::TryInto;

#[derive(PartialEq, Debug)]
enum GumBoiState {
    Active,
    Halt,
    Exit,
}

pub struct GumBoi {
    cpu: CPU,
    ppu: PPU,
    memory: Arc<Mutex<Memory>>,
    cycle: usize,
    state: GumBoiState,
}

impl GumBoi {
    pub fn new() -> GumBoi {
        let memory = Arc::new(Mutex::new(Memory::new()));
        let memory_cpu = Arc::clone(&memory);
        let memory_ppu = Arc::clone(&memory);

        GumBoi {
            cpu: CPU::new(memory_cpu),
            ppu: PPU::new(memory_ppu),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        }
    }
    pub fn insert_cartridge(&mut self, cartridge_rom: Vec<u8>) {
        //Load Catridge into GumBoi ROM
        self.memory.lock().unwrap().load_cartridge(cartridge_rom);
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
