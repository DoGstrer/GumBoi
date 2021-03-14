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
use std::convert::TryInto;

#[derive(PartialEq,Debug)]
pub struct GumBoi{
    memory: Memory,
    registers: Registers,
    cycle: usize,
    state: u8,
}

impl GumBoi{
    pub fn new() -> GumBoi{
        GumBoi{
            memory: Memory::new(),
            registers: Registers::new(),
            cycle: 0,
            state: 1,
        }
    }
    pub fn load(&mut self,boot_rom: Vec<u8>,catridge_rom: Vec<u8>){
        //Load Boot ROM into GumBoi
        self.memory.boot_rom = boot_rom[..].try_into().expect("Boot ROM exceeds 256 bytes!");
        //Load Catridge into GumBoi ROM
        let mut addr: u16 = 0x0000;
        for byte in catridge_rom{
            self.memory.set_addr(addr,byte);
            addr+=1;
        }
    }
    pub fn start(&mut self){
        //self.memory.set_addr(0xff44,0x90);
        while self.state == 1{
            println!("{:?}",self);
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
    use std::convert::TryInto;
    
    macro_rules! arr_u8{
        ( $size:expr,[$($x:expr),*] ) => {
            {
                let mut temp_a = Vec::new();
                $(
                    temp_a.push($x);
                )*
                temp_a.push(0xff);
                let mut temp_b:[u8;$size] = [0;$size];
                for (temp_index,temp_a_elem) in temp_a.iter().enumerate(){
                    temp_b[temp_index] = *temp_a_elem;
                }
                temp_b
            }
        };
    }

    macro_rules! registers{
        ($($register_name:ident:$register_value:expr),*) => {
            {
                Registers{
                    $(
                        $register_name: $register_value,
                    )*
                    ..empty_registers
                }
            }
        }
    }

    macro_rules! memory{
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

    macro_rules! test_case{
        ($test_name:ident|$initial_state:expr,$expected_state:expr) => {
            #[test]
            fn $test_name(){
                assert_eq!(get_next_state($initial_state),$expected_state)
            }
        }
    }
    
    const SET_Z:u8=0b10000000;
    const SET_N:u8=0b01000000;
    const SET_H:u8=0b00100000;
    const SET_C:u8=0b00010000;

    const RESET_Z:u8=0b01111111;
    const RESET_N:u8=0b10111111;
    const RESET_H:u8=0b11011111;
    const RESET_C:u8=0b11101111;

    const empty_registers: Registers = Registers{a:0x0,b:0x0,c:0x0,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x0};

    fn get_next_state(current_state: (Registers,Memory,usize)) -> (Registers,Memory,usize){
        let mut gb: GumBoi = GumBoi{
            registers: current_state.0,
            memory: current_state.1,
            cycle: current_state.2,
            state: 1,
        };
        gb.start();    
        gb.registers.pc-=1;
        (gb.registers,gb.memory,gb.cycle)
    }

    #[test]
    fn Oxff(){
        let instruction_set: [u8;256] = arr_u8!(256,[0xff]);
        let initial_state = (registers!(),memory!(0x0=>0xff),0);
        let expected_state = (registers!(),memory!(0x0=>0xff),0);

        assert_eq!(get_next_state(initial_state),expected_state);
    }

    #[test]
    //RLC [Z 0 0 C]
    fn OxCB_Ox11(){
        let instruction_set = arr_u8!(256,[0xCB,0x11]);
        let initial_state = (Registers{c: 0x85,..empty_registers},memory!(0x0=>0xcb,0x1=>0x11,0x2=>0xff),0);
        let expected_state = (Registers{c: 0x0B,f:SET_C,pc:0x2, ..empty_registers},memory!(0x0=>0xcb,0x1=>0x11,0x2=>0xff),8);

        assert_eq!(get_next_state(initial_state),expected_state);       
    }

    #[test]
    //RLA [0 0 0 C]
    fn Ox17(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x17]);
        let initial_state = (Registers{a:0x95,..empty_registers},memory!(0x0=>0x17,0x1=>0xff),0);
        let expected_state = (Registers{a:0x2A,f:SET_C,pc:1,..empty_registers},memory!(0x0=>0x17,0x1=>0xff),4);

        assert_eq!(get_next_state(initial_state),expected_state);        
    }

    #[test]
    fn Ox21(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x21,0xFF,0x9F]);
        let initial_state = (Registers{..empty_registers},memory!(0x0=>0x21,0x1=>0xff,0x2=>0x9f,0x3=>0xff),0);
        let expected_state = (Registers{h:0x9F,l:0xFF,pc:3,..empty_registers},memory!(0x0=>0x21,0x1=>0xff,0x2=>0x9f,0x3=>0xff),12);

        assert_eq!(get_next_state(initial_state),expected_state);        
    }

    #[test]
    fn Ox32(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x32]);
        let initial_state = (Registers{a:0x0,h:0x1,l:0x1,..empty_registers},memory!(0x0=>0x32,0x1=>0xff),0);
        let expected_state = (Registers{a:0x0,h:0x1,l:0x0,pc:0x1,..empty_registers},memory!(0x0=>0x32,0x1=>0xff),8);

        assert_eq!(get_next_state(initial_state),expected_state);   
    }

    //[test]
    fn Ox20_with_z_set(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x20,0x03,0xFF,0xFF,0xFF]);
        let initial_state = (Registers{f:SET_Z,..empty_registers},memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF),0);
        let expected_state = (Registers{f:SET_Z,pc:2,..empty_registers},memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF),8);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    fn Ox20_with_z_not_set(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x20,0x03,0xFF,0xFF,0xFF]);
        let initial_state = (Registers{..empty_registers},memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF),0);
        let expected_state = (Registers{pc:4,..empty_registers},memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF),12);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    fn OxCB_Ox7C(){
        let instruction_set: [u8;256] = arr_u8!(256,[0xCB,0x7C]);
        let initial_state = (Registers{h:0x80,..empty_registers},memory!(0x0=>0xcb,0x1=>0x7c,0x2=>0xff),0);
        let expected_state = (Registers{h:0x80,pc:2,f:SET_H,..empty_registers},memory!(0x0=>0xcb,0x1=>0x7c,0x2=>0xff),8);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    //JR Z,r8
    fn Ox28_with_z_set(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x28,0x02,0xFF,0xFF]);
        let initial_state = (Registers{f:SET_Z,..empty_registers},memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff),0);
        let expected_state = (Registers{pc:4,f:SET_Z,..empty_registers},memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff),12);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    //JR Z,r8
    fn Ox28_with_z_not_set(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x28,0x02,0xff,0xff]);
        let initial_state = (Registers{..empty_registers},memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff),0);
        let expected_state = (Registers{pc:2,..empty_registers},memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff),8);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    //JR i8 [- - - -]
    fn Ox18_negative_jump(){
        let instruction_set: [u8;256] = arr_u8!(256,[0xFF,0x3C,0xFF,0x18,0xFC,0xff,0xff]);
        let initial_state = (Registers{a:0x00,pc:3,..empty_registers},memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0xFC,0x5=>0xff,0x6=>0xff),0);
        let expected_state = (Registers{a:0x01,pc:2,..empty_registers},memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0x01,0x5=>0xff,0x6=>0xff),4);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    //JR i8 [- - - -]
    fn Ox18_positive_jump(){
        let initial_state = (Registers{a:0x00,pc:3,..empty_registers},memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0x01,0x5=>0xff,0x6=>0x3c),0);
        let expected_state = (Registers{a:0x01,pc:7,..empty_registers},memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0x01,0x5=>0xff,0x6=>0x3c),4);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    //INC A [- * 0 *]
    fn Ox3C_with_overflow(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x3C]);
        let initial_state = (Registers{a:0xFF,f: SET_N | SET_C | SET_N,..empty_registers},memory!(0x0=>0x3c,0x1=>0xff),0);
        let expected_state = (Registers{a:0x00,pc:1,f:SET_H | SET_Z | SET_C,..empty_registers},memory!(0x0=>0x3c,0x1=>0xff),4);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    #[test]
    //INC A [- * 0 *]
    fn Ox3C_without_overflow(){
        let instruction_set: [u8;256] = arr_u8!(256,[0x3C]);
        let initial_state = (Registers{a:0x0E,f:SET_C | SET_H | SET_N | SET_Z,..empty_registers},memory!(0x0=>0x3c,0x1=>0xff),0);
        let expected_state = (Registers{a:0x0F,f:SET_C,pc:1,..empty_registers},memory!(0x0=>0x3c,0x1=>0xff),4);

        assert_eq!(get_next_state(initial_state),expected_state);
    }
    // LD INTRUCTIONS //
    //LD A d8
    test_case![Ox3E|(registers!(),memory!(0x0=>0x3E,0x1=>0xFE),0),(registers!(a:0xFE,pc:2),memory!(0x0=>0x3E,0x1=>0xFE),8)];
}