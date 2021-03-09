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

pub struct GumBoi{
    memory: Memory,
    registers: Registers,
    cycle: usize
}

impl GumBoi{
    pub fn new(boot_rom: Vec<u8>) -> GumBoi{
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

#[cfg(test)]
mod alu_intruction_tests{
    use super::Registers;
    use super::GumBoi;
    use super::cpu::CPU;
    
    const SET_Z:u8=0b10000000;
    const SET_N:u8=0b01000000;
    const SET_H:u8=0b00100000;
    const SET_C:u8=0b00010000;

    const RESET_Z:u8=0b01111111;
    const RESET_N:u8=0b10111111;
    const RESET_H:u8=0b11011111;
    const RESET_C:u8=0b11101111;

    struct RegisterStates{
        initial_state: Registers,
        expected_state: Registers,
    }

    #[test]
    fn cb_11(){
        let instruction_set: Vec<u8> = vec![0xCB,0x11];
        let test_cases: Vec<RegisterStates> = vec![
            RegisterStates{ initial_state: Registers{a:0x0,b:0x0,c:0x80,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x0}, expected_state: Registers{a:0x0,b:0x0,c:0x0,d:0x0,e:0x0,f:SET_C | SET_Z,h:0x0,l:0x0,sp:0x0,pc:0x2}},
            RegisterStates{ initial_state: Registers{a:0x0,b:0x0,c:0x1,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x0}, expected_state: Registers{a:0x0,b:0x0,c:0x2,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x2}},
        ];
        
        let mut gb: GumBoi = GumBoi::new(instruction_set);
        for test_case in test_cases{
            gb.registers=test_case.initial_state;    
            gb.execute();
            assert_eq!(gb.registers,test_case.expected_state);
        }       
    }
}