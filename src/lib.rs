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
use interrupt::{InterruptController,Interrupt,InterruptType};
use memory::Memory;
use ppu::PPU;
use registers::Flag;
use registers::Registers;

use std::sync::{Mutex,Arc};
use std::sync::mpsc;
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
    interrupt_controller: InterruptController,
    memory: Arc<Mutex<Memory>>,
    cycle: usize,
    state: GumBoiState,
}

impl GumBoi {
    pub fn new() -> GumBoi {

        let memory = Arc::new(Mutex::new(Memory::new()));
        let ime = Arc::new(Mutex::new(false));
        let (interrupt_tx,interrupt_rx): (mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();

        GumBoi {
            cpu: CPU::new(Arc::clone(&memory),Arc::clone(&ime)),
            ppu: PPU::new(Arc::clone(&memory)),
            interrupt_controller: InterruptController::new(Arc::clone(&memory),Arc::clone(&ime),interrupt_rx),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        }
    }
    pub fn insert_cartridge(&mut self, cartridge_rom: Vec<u8>) {
        self.memory.lock().unwrap().load_cartridge(cartridge_rom); //Load Catridge into GumBoi ROM
    }
    pub fn start(&mut self) {
        //self.memory.set_addr(0xff44,0x90);
        let rst_addr:u16;
        while self.cpu.get_state() == CPUState::Active {
            self.cpu.execute();
            match self.interrupt_controller.execute() {
                Some(rst_addr) => self.cpu.rst(rst_addr),
                None => {},
            }          
        }
    }
    pub fn exit(&self) {
        //TODO
    }
}

#[cfg(test)]
mod interrupt_tests{
    use super::{GumBoi,InterruptController,CPU,PPU,InterruptType,Memory,GumBoiState,Interrupt};
    use std::sync::{Arc,Mutex,mpsc};

    macro_rules! memory {
        ($($addr:expr=>$value:expr),*) => {
            {
                let mut mem = Memory::new();
                $(
                    mem.set_addr($addr, $value);
                )*
                mem
            }
        }
    }


    #[test]
    fn test_interrupt_rst_joypad(){
        let memory = Arc::new(Mutex::new(memory!(0x0=>0x31,0x1=>0xFF,0x2=>0xFE,0x60=>0x76,0xFFFF=>0b00010000)));
        let ime = Arc::new(Mutex::new(true));
        let (interrupt_tx,interrupt_rx): (mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();

        let mut gumboi = GumBoi {
            cpu: CPU::new(Arc::clone(&memory),Arc::clone(&ime)),
            ppu: PPU::new(Arc::clone(&memory)),
            interrupt_controller: InterruptController::new(Arc::clone(&memory),Arc::clone(&ime),interrupt_rx),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        };

        interrupt_tx.send(Interrupt::JOYPAD).unwrap();
        interrupt_tx.send(InterruptType::EXIT).unwrap();

        gumboi.start();

        assert_eq!(gumboi.cpu.get_registers().pc,0x61);
    }
    #[test]
    fn test_interrupt_rst_serial(){
        let memory = Arc::new(Mutex::new(memory!(0x0=>0x31,0x1=>0xFF,0x2=>0xFE,0x58=>0x76,0xFFFF=>0b00001000)));
        let ime = Arc::new(Mutex::new(true));
        let (interrupt_tx,interrupt_rx): (mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();

        let mut gumboi = GumBoi {
            cpu: CPU::new(Arc::clone(&memory),Arc::clone(&ime)),
            ppu: PPU::new(Arc::clone(&memory)),
            interrupt_controller: InterruptController::new(Arc::clone(&memory),Arc::clone(&ime),interrupt_rx),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        };

        interrupt_tx.send(Interrupt::JOYPAD).unwrap();
        interrupt_tx.send(InterruptType::EXIT).unwrap();

        gumboi.start();

        assert_eq!(gumboi.cpu.get_registers().pc,0x59);
    }
    #[test]
    fn test_interrupt_rst_timer(){
        let memory = Arc::new(Mutex::new(memory!(0x0=>0x31,0x1=>0xFF,0x2=>0xFE,0x50=>0x76,0xFFFF=>0b00000100)));
        let ime = Arc::new(Mutex::new(true));
        let (interrupt_tx,interrupt_rx): (mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();

        let mut gumboi = GumBoi {
            cpu: CPU::new(Arc::clone(&memory),Arc::clone(&ime)),
            ppu: PPU::new(Arc::clone(&memory)),
            interrupt_controller: InterruptController::new(Arc::clone(&memory),Arc::clone(&ime),interrupt_rx),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        };

        interrupt_tx.send(Interrupt::TIMER).unwrap();
        interrupt_tx.send(InterruptType::EXIT).unwrap();

        gumboi.start();

        assert_eq!(gumboi.cpu.get_registers().pc,0x51);
    }
    #[test]
    fn test_interrupt_rst_lcd_stat(){
        let memory = Arc::new(Mutex::new(memory!(0x0=>0x31,0x1=>0xFF,0x2=>0xFE,0x48=>0x76,0xFFFF=>0b00000010)));
        let ime = Arc::new(Mutex::new(true));
        let (interrupt_tx,interrupt_rx): (mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();

        let mut gumboi = GumBoi {
            cpu: CPU::new(Arc::clone(&memory),Arc::clone(&ime)),
            ppu: PPU::new(Arc::clone(&memory)),
            interrupt_controller: InterruptController::new(Arc::clone(&memory),Arc::clone(&ime),interrupt_rx),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        };

        interrupt_tx.send(Interrupt::LCD_STAT).unwrap();
        interrupt_tx.send(InterruptType::EXIT).unwrap();

        gumboi.start();

        assert_eq!(gumboi.cpu.get_registers().pc,0x49);
    }
    #[test]
    fn test_interrupt_rst_vblank(){
        let memory = Arc::new(Mutex::new(memory!(0x0=>0x31,0x1=>0xFF,0x2=>0xFE,0x40=>0x76,0xFFFF=>0b00000001)));
        let ime = Arc::new(Mutex::new(true));
        let (interrupt_tx,interrupt_rx): (mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();

        let mut gumboi = GumBoi {
            cpu: CPU::new(Arc::clone(&memory),Arc::clone(&ime)),
            ppu: PPU::new(Arc::clone(&memory)),
            interrupt_controller: InterruptController::new(Arc::clone(&memory),Arc::clone(&ime),interrupt_rx),
            memory: memory,
            state: GumBoiState::Active,
            cycle: 0,
        };

        interrupt_tx.send(Interrupt::JOYPAD).unwrap();
        interrupt_tx.send(InterruptType::EXIT).unwrap();

        gumboi.start();

        assert_eq!(gumboi.cpu.get_registers().pc,0x41);
    }
}