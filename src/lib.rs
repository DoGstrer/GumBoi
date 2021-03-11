#![allow(unused)]

mod registers;
mod memory;
mod ppu;
mod sound;
mod cpu;
mod joypad;
mod timer;
mod interrupt;

use cpu::CPU;
use memory::Memory;
use registers::Registers;
use registers::Flag;

#[derive(PartialEq,Debug)]
pub struct GumBoi{
    memory: Memory,
    registers: Registers,
    cycle: usize,
}

impl GumBoi{
    pub fn new(boot_rom: [u8;256]) -> GumBoi{
        GumBoi{
            memory: Memory::new(boot_rom),
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
        loop{
            self.execute();
        }
    }
    pub fn exit(&self){
        //TODO
    }
}

//TODO revamp unit tests
#[cfg(test)]
mod alu_intruction_tests{

    use super::Registers;
    use super::GumBoi;
    use super::cpu::CPU;
    use super::memory::Memory;
    
    const SET_Z:u8=0b10000000;
    const SET_N:u8=0b01000000;
    const SET_H:u8=0b00100000;
    const SET_C:u8=0b00010000;

    const RESET_Z:u8=0b01111111;
    const RESET_N:u8=0b10111111;
    const RESET_H:u8=0b11011111;
    const RESET_C:u8=0b11101111;

    const empty_registers: Registers = Registers{a:0x0,b:0x0,c:0x80,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x0};

    fn get_next_state(current_state: (Registers,Memory,usize)) -> (Registers,Memory,usize){
        let mut gb: GumBoi = GumBoi{
            registers: current_state.0,
            memory: current_state.1,
            cycle: current_state.2,
        };
        gb.execute();    
        (gb.registers,gb.memory,gb.cycle)
    }

    #[test]
    //RLC [Z 0 0 C]
    fn OxCB_Ox11(){
        let initial_state = (Registers{c: 0x85,..empty_registers},Memory::new(vec![0xCB,0x11]),0);
        let expected_state = (Registers{c: 0x0B,f:SET_C | SET_Z,pc:0x2, ..empty_registers},Memory::new(vec![0xCB,0x11]),8);

        assert_eq!(get_next_state(initial_state),expected_state);       
    }

    #[test]
    //RLA [0 0 0 C]
    fn Ox17(){
        let instruction_set: Vec<u8> = vec![0x17];
        let initial_state = Registers{a: 0x95,f: SET_C, ..empty_registers};
        let expected_state = Registers{a: 0x2B,f: SET_C, pc: 1, ..empty_registers};

        //assert_eq!(get_final_state(instruction_set,initial_state),expected_state);
    }
}