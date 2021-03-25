/*TO TEST - 
add8 : 26 Intructions 
add16 : 12 instructions
sub8 : 37 instructions
sub16 : 7 instructions
*/
use std::cell::RefCell;
use std::rc::Rc;

use super::registers::Flag;
use super::registers::Registers;
use super::memory::Memory;

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum CPUState{
    Halt,
    Stop,
    Active,
    Exit,
}

pub struct CPU{
    pub registers: Registers,
    pub memory: Rc<RefCell<Memory>>,
    pub cycle: usize,
    pub state: CPUState,
}

impl CPU{
    pub fn new(memory: Rc<RefCell<Memory>>) -> CPU{
        CPU{
            registers: Registers::new(),
            cycle: 0,
            state: CPUState::Active,
            memory: memory,
        }
    }
    fn get_next_byte8(&mut self) -> u8{
        self.registers.pc+=1;
        self.memory.borrow().get_addr(self.registers.pc)
    }
    fn get_next_byte16(&mut self) -> u16{
        let mut byte: u16;
        self.registers.pc+=1;
        byte = self.memory.borrow().get_addr(self.registers.pc) as u16;
        self.registers.pc+=1;
        byte |= (self.memory.borrow().get_addr(self.registers.pc) as u16) << 8;
        byte
    }
    pub fn execute(&mut self){
        let opcode:u8 = self.memory.borrow().get_addr(self.registers.pc);
        println!("{:#x?}",opcode);
        let mut opcode_cb:u8=0x0;
        let mut byte:u16=0x0;
        let mut byte8:u8=0x0;
        let mut flag:bool=false;

        match opcode{
            //8 bit LD
            // LD r d8 [- - - -]
            0x3E => { self.registers.a = self.get_next_byte8(); self.cycle=8; },
            0x06 => { self.registers.b = self.get_next_byte8(); self.cycle=8; },
            0x0E => { self.registers.c = self.get_next_byte8(); self.cycle=8; },
            0x16 => { self.registers.d = self.get_next_byte8(); self.cycle=8; },
            0x1E => { self.registers.e = self.get_next_byte8(); self.cycle=8; },
            0x26 => { self.registers.h = self.get_next_byte8(); self.cycle=8; },
            0x2E => { self.registers.l = self.get_next_byte8(); self.cycle=8; },
            0x36 => { byte8 = self.get_next_byte8(); self.memory.borrow_mut().set_addr(self.registers.get_hl(),byte8); self.cycle=12; },
            
            0x7F => { self.registers.a=self.registers.a; self.cycle=4; },
            0x78 => { self.registers.a=self.registers.b; self.cycle=4; },
            0x79 => { self.registers.a=self.registers.c; self.cycle=4; },
            0x7A => { self.registers.a=self.registers.d; self.cycle=4; },
            0x7B => { self.registers.a=self.registers.e; self.cycle=4; },
            0x7C => { self.registers.a=self.registers.h; self.cycle=4; },
            0x7D => { self.registers.a=self.registers.l; self.cycle=4; },
            0x7E => { self.registers.a=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x40 => { self.registers.b=self.registers.b; self.cycle=4; },
            0x41 => { self.registers.b=self.registers.c; self.cycle=4; },
            0x42 => { self.registers.b=self.registers.d; self.cycle=4; },
            0x43 => { self.registers.b=self.registers.e; self.cycle=4; },
            0x44 => { self.registers.b=self.registers.h; self.cycle=4; },
            0x45 => { self.registers.b=self.registers.l; self.cycle=4; },
            0x46 => { self.registers.b=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x48 => { self.registers.c=self.registers.b; self.cycle=4; },
            0x49 => { self.registers.c=self.registers.c; self.cycle=4; },
            0x4A => { self.registers.c=self.registers.d; self.cycle=4; },
            0x4B => { self.registers.c=self.registers.e; self.cycle=4; },
            0x4C => { self.registers.c=self.registers.h; self.cycle=4; },
            0x4D => { self.registers.c=self.registers.l; self.cycle=4; },
            0x4E => { self.registers.c=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x50 => { self.registers.d=self.registers.b; self.cycle=4; },
            0x51 => { self.registers.d=self.registers.c; self.cycle=4; },
            0x52 => { self.registers.d=self.registers.d; self.cycle=4; },
            0x53 => { self.registers.d=self.registers.e; self.cycle=4; },
            0x54 => { self.registers.d=self.registers.h; self.cycle=4; },
            0x55 => { self.registers.d=self.registers.l; self.cycle=4; },
            0x56 => { self.registers.d=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x58 => { self.registers.e=self.registers.b; self.cycle=4; },
            0x59=>  { self.registers.e=self.registers.c; self.cycle=4; },
            0x5A => { self.registers.e=self.registers.d; self.cycle=4; },
            0x5B => { self.registers.e=self.registers.e; self.cycle=4; },
            0x5C => { self.registers.e=self.registers.h; self.cycle=4; },
            0x5D => { self.registers.e=self.registers.l; self.cycle=4; },
            0x5E => { self.registers.e=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x60 => { self.registers.h=self.registers.b; self.cycle=4; },
            0x61 => { self.registers.h=self.registers.c; self.cycle=4; },
            0x62 => { self.registers.h=self.registers.d; self.cycle=4; },
            0x63 => { self.registers.h=self.registers.e; self.cycle=4; },
            0x64 => { self.registers.h=self.registers.h; self.cycle=4; },
            0x65 => { self.registers.h=self.registers.l; self.cycle=4; },
            0x66 => { self.registers.h=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x68 => { self.registers.l=self.registers.b; self.cycle=4; },
            0x69 => { self.registers.l=self.registers.c; self.cycle=4; },
            0x6A => { self.registers.l=self.registers.d; self.cycle=4; },
            0x6B => { self.registers.l=self.registers.e; self.cycle=4; },
            0x6C => { self.registers.l=self.registers.h; self.cycle=4; },
            0x6D => { self.registers.l=self.registers.l; self.cycle=4; },
            0x6E => { self.registers.l=self.memory.borrow().get_addr(self.registers.get_hl()); self.cycle=8; },
            0x70 => { self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.b); self.cycle=8; },
            0x71 => { self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.c); self.cycle=8; },
            0x72 => { self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.d); self.cycle=8; },
            0x73 => { self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.e); self.cycle=8; },
            0x74 => { self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.h); self.cycle=8; },
            0x75 => { self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.l); self.cycle=8; },
            
            0x0A => { self.registers.a=self.memory.borrow().get_addr(self.registers.get_bc()); self.cycle=8; },
            0x1A => { self.registers.a=self.memory.borrow().get_addr(self.registers.get_de()); self.cycle=8; },
            0xFA => { byte=self.get_next_byte16(); self.registers.a=self.memory.borrow().get_addr(byte); self.cycle=16; },
            
            0x47 => { self.registers.b=self.registers.a; self.cycle=4; },
            0x4F => { self.registers.c=self.registers.a; self.cycle=4; },
            0x57 => {
                self.registers.d=self.registers.a;
                self.cycle=4;
            },
            0x5F => {
                self.registers.e=self.registers.a;
                self.cycle=4;
            },
            0x67 => {
                self.registers.h=self.registers.a;
                self.cycle=4;
            },
            0x6F => {
                self.registers.l=self.registers.a;
                self.cycle=4;
            },
            0x02 => {
                self.memory.borrow_mut().set_addr(self.registers.get_bc(),self.registers.a);
                self.cycle=8;
            },
            0x12 => {
                self.memory.borrow_mut().set_addr(self.registers.get_de(),self.registers.a);
                self.cycle=8;
            },
            0x77 => {
                self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.a);
                self.cycle=8;
            },
            0xEA => {
                byte = self.get_next_byte16();
                self.memory.borrow_mut().set_addr(byte,self.registers.a);
                self.cycle=16;
            },
            0xF2 => {
                self.registers.a=self.memory.borrow().get_addr(0xFF00|(self.registers.c as u16));
                self.cycle=8;
            },
            0xE2 => {
                self.memory.borrow_mut().set_addr(0xFF00+(self.registers.c as u16),self.registers.a); self.cycle=8;
            },
            0x3A => {
                self.registers.a=self.memory.borrow().get_addr(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl()-0x1);
                self.cycle=8;
            },
            0x32 => {
                self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.a);
                self.registers.set_hl(self.registers.get_hl()-1);
                self.cycle=8;
            },
            0x2A => {
                self.registers.a=self.memory.borrow().get_addr(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl()+0x1);
                self.cycle=8;
            },
            0x22 => {
                self.memory.borrow_mut().set_addr(self.registers.get_hl(),self.registers.a);
                self.registers.set_hl(self.registers.get_hl()+0x0001);
                self.cycle=8;
            },
            0xE0 => {
                byte8=self.get_next_byte8();
                self.memory.borrow_mut().set_addr(0xFF00+(byte8 as u16),self.registers.a);
                self.cycle=12;
            },
            0xF0 => {
                byte8 = self.get_next_byte8();
                self.registers.a=self.memory.borrow().get_addr(0xFF00+(byte8 as u16));
                self.cycle=12;
            },
            
            //16 bit LD
            0x01 => {
                byte = self.get_next_byte16();
                self.registers.set_bc(byte);
                self.cycle=12;
            },
            0x11 => {
                byte = self.get_next_byte16();
                self.registers.set_de(byte);
                self.cycle=12;
            },
            0x21 => {
                byte = self.get_next_byte16();
                self.registers.set_hl(byte);
                self.cycle=12;
            },
            0x31 => {
                byte = self.get_next_byte16();
                self.registers.sp=byte;
                self.cycle=12;
            },
            0xF9 => {
                self.registers.sp=self.registers.get_hl();
                self.cycle=8
            },
            0xF8 => {
                byte = self.get_next_byte16();
                byte = self.add16(self.registers.sp,byte,false); self.registers.set_hl(byte);
                self.registers.reset_z();
                self.registers.reset_n();
                self.cycle=12;
            },
            
            //To be reviewed
            0x08 => {
                byte = self.get_next_byte16();
                self.memory.borrow_mut().set_addr(byte,(self.registers.sp&0x00ff) as u8); 
                self.memory.borrow_mut().set_addr(byte+1,(self.registers.sp>>8) as u8);
                self.cycle=20;
            },

            //Stack Operations
            //PUSH
            0xF5 => { self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.a); self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.f); self.cycle=16; },
            0xC5 => { self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.b); self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.c); self.cycle=16; },
            0xD5 => { self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.d); self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.e); self.cycle=16; },
            0xE5 => { self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.h); self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,self.registers.l); self.cycle=16; },
            
            //POP
            0xF1 => { self.registers.f=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.registers.a=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.cycle=12; },
            0xC1 => { self.registers.c=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.registers.b=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.cycle=12; },
            0xD1 => { self.registers.e=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.registers.d=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.cycle=12; },
            0xE1 => { self.registers.l=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.registers.h=self.memory.borrow().get_addr(self.registers.sp); self.registers.sp+=1; self.cycle=12; },
            
            // 8 BIT ALU 
            //ADD
            0x87 => { self.registers.a=self.add8(self.registers.a,self.registers.a,false); self.cycle=4; },
            0x80 => { self.registers.a=self.add8(self.registers.a,self.registers.b,false); self.cycle=4; },
            0x81 => { self.registers.a=self.add8(self.registers.a,self.registers.c,false); self.cycle=4; },
            0x82 => { self.registers.a=self.add8(self.registers.a,self.registers.d,false); self.cycle=4; },
            0x83 => { self.registers.a=self.add8(self.registers.a,self.registers.e,false); self.cycle=4; },
            0x84 => { self.registers.a=self.add8(self.registers.a,self.registers.h,false); self.cycle=4; },
            0x85 => { self.registers.a=self.add8(self.registers.a,self.registers.l,false); self.cycle=4; },
            0x86 => { byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.a=self.add8(self.registers.a,byte8,false); self.cycle=8; },
            0xC6 => { byte8 = self.get_next_byte8(); self.registers.a=self.add8(self.registers.a,byte8,false); self.cycle=8; },
            
            //ADD WITH CARRY
            0x8F => { self.registers.a=self.add8(self.registers.a,self.registers.a,true); self.cycle=4; },                
            0x88 => { self.registers.a=self.add8(self.registers.a,self.registers.b,true); self.cycle=4; },
            0x89 => { self.registers.a=self.add8(self.registers.a,self.registers.c,true); self.cycle=4; },
            0x8A => { self.registers.a=self.add8(self.registers.a,self.registers.d,true); self.cycle=4; },
            0x8B => { self.registers.a=self.add8(self.registers.a,self.registers.e,true); self.cycle=4; },
            0x8C => { self.registers.a=self.add8(self.registers.a,self.registers.h,true); self.cycle=4; },
            0x8D => { self.registers.a=self.add8(self.registers.a,self.registers.l,true); self.cycle=4; },
            0x8E => { byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.a=self.add8(self.registers.a,byte8,true); self.cycle=8; },
            0xCE => { byte8 = self.get_next_byte8(); self.registers.a=self.add8(self.registers.a,byte8,true); self.cycle=8; },
            
            //SUB
            0x97 => { self.registers.a=self.sub8(self.registers.a,self.registers.a,false); self.cycle=4; },
            0x90 => { self.registers.a=self.sub8(self.registers.a,self.registers.b,false); self.cycle=4; },
            0x91 => { self.registers.a=self.sub8(self.registers.a,self.registers.c,false); self.cycle=4; },
            0x92 => { self.registers.a=self.sub8(self.registers.a,self.registers.d,false); self.cycle=4; },
            0x93 => { self.registers.a=self.sub8(self.registers.a,self.registers.e,false); self.cycle=4; },
            0x94 => { self.registers.a=self.sub8(self.registers.a,self.registers.h,false); self.cycle=4; },
            0x95 => { self.registers.a=self.sub8(self.registers.a,self.registers.l,false); self.cycle=4; },
            0x96 => { byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.a=self.sub8(self.registers.a,byte8,false); self.cycle=8; },
            0xD6 => { byte8 = self.get_next_byte8(); self.registers.a=self.sub8(self.registers.a,byte8,false); self.cycle=8; },

            //SUB WITH BORROW
            0x9F => { self.registers.a=self.sub8(self.registers.a,self.registers.a,true); self.cycle=4; },
            0x98 => { self.registers.a=self.sub8(self.registers.a,self.registers.b,true); self.cycle=4; },
            0x99 => { self.registers.a=self.sub8(self.registers.a,self.registers.c,true); self.cycle=4; },
            0x9A => { self.registers.a=self.sub8(self.registers.a,self.registers.d,true); self.cycle=4; },
            0x9B => { self.registers.a=self.sub8(self.registers.a,self.registers.e,true); self.cycle=4; },
            0x9C => { self.registers.a=self.sub8(self.registers.a,self.registers.h,true); self.cycle=4; },
            0x9D => { self.registers.a=self.sub8(self.registers.a,self.registers.l,true); self.cycle=4; },
            0x9E => { byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.a=self.sub8(self.registers.a,byte8,true); self.cycle=8; },
            0xDE => { byte8 = self.get_next_byte8(); self.registers.a=self.sub8(self.registers.a,byte as u8,true); self.cycle=8; },
            
            //LOGICAL OPERATIONS
            //AND
            0xA7 => {self.registers.a&=self.registers.a; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA0 => {self.registers.a&=self.registers.b; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA1 => {self.registers.a&=self.registers.c; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA2 => {self.registers.a&=self.registers.d; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA3 => {self.registers.a&=self.registers.e; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA4 => {self.registers.a&=self.registers.h; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA5 => {self.registers.a&=self.registers.l; self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA6 => {self.registers.a&=self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=8;},
            0xE6 => {self.registers.a&=self.get_next_byte8(); self.registers.reset_flags(); self.registers.set_h(); if self.registers.a==0x0 {self.registers.set_z();} self.cycle=8;},

            //OR
            0xB7 => {self.registers.a|=self.registers.a; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB0 => {self.registers.a|=self.registers.b; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB1 => {self.registers.a|=self.registers.c; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB2 => {self.registers.a|=self.registers.d; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB3 => {self.registers.a|=self.registers.e; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB4 => {self.registers.a|=self.registers.h; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB5 => {self.registers.a|=self.registers.l; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xB6 => {self.registers.a|=self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=8;},
            0xF6 => {byte8 = self.get_next_byte8(); self.registers.a |= byte8; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=8;},
            
            //XOR
            0xAF => {self.registers.a^=self.registers.a; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA8 => {self.registers.a^=self.registers.b; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xA9 => {self.registers.a^=self.registers.c; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xAA => {self.registers.a^=self.registers.d; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xAB => {self.registers.a^=self.registers.e; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xAC => {self.registers.a^=self.registers.h; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xAD => {self.registers.a^=self.registers.l; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=4;},
            0xAE => {self.registers.a^=self.memory.borrow().get_addr(self.registers.get_hl()); self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=8;},
            0xEE => {byte8=self.get_next_byte8(); self.registers.a^=byte8; self.registers.reset_flags(); if self.registers.a==0x0 {self.registers.set_z();}self.cycle=8;},
            
            //CP
            0xBF => { self.sub8(self.registers.a,self.registers.a,false); self.cycle=4; },
            0xB8 => { self.sub8(self.registers.a,self.registers.b,false); self.cycle=4; },
            0xB9 => { self.sub8(self.registers.a,self.registers.c,false); self.cycle=4; },
            0xBA => { self.sub8(self.registers.a,self.registers.d,false); self.cycle=4; },
            0xBB => { self.sub8(self.registers.a,self.registers.e,false); self.cycle=4; },
            0xBC => { self.sub8(self.registers.a,self.registers.h,false); self.cycle=4; },
            0xBD => { self.sub8(self.registers.a,self.registers.l,false); self.cycle=4; },
            0xBE => { byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); self.sub8(self.registers.a,byte8,false); self.cycle=8; },
            0xFE => { byte8 = self.get_next_byte8(); self.sub8(self.registers.a,byte8 as u8,false); self.cycle=8; },
            
            //INC
            // INC A [- * 0 *]
            0x3C => { flag=self.registers.is_set_c(); self.registers.a=self.add8(self.registers.a,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x04 => { flag=self.registers.is_set_c(); self.registers.b=self.add8(self.registers.b,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x0C => { flag=self.registers.is_set_c(); self.registers.c=self.add8(self.registers.c,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x14 => { flag=self.registers.is_set_c(); self.registers.d=self.add8(self.registers.d,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x1C => { flag=self.registers.is_set_c(); self.registers.e=self.add8(self.registers.e,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x24 => { flag=self.registers.is_set_c(); self.registers.h=self.add8(self.registers.h,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x2C => { flag=self.registers.is_set_c(); self.registers.l=self.add8(self.registers.l,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();}  self.registers.reset_n(); self.cycle=4; },
            0x34 => { flag=self.registers.is_set_c(); byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); byte8 = self.add8(byte8,0x01,false); self.memory.borrow_mut().set_addr(self.registers.get_hl(),byte8); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.registers.reset_n(); self.cycle=12; },
            //DEC
            0x3D => { flag=self.registers.is_set_c(); self.registers.a=self.sub8(self.registers.a,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x05 => { flag=self.registers.is_set_c(); self.registers.b=self.sub8(self.registers.b,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x0D => { flag=self.registers.is_set_c(); self.registers.c=self.sub8(self.registers.c,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x15 => { flag=self.registers.is_set_c(); self.registers.d=self.sub8(self.registers.d,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x1D => { flag=self.registers.is_set_c(); self.registers.e=self.sub8(self.registers.e,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x25 => { flag=self.registers.is_set_c(); self.registers.h=self.sub8(self.registers.h,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x2D => { flag=self.registers.is_set_c(); self.registers.l=self.sub8(self.registers.l,0x01,false); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=4; },
            0x35 => { flag=self.registers.is_set_c(); byte8 = self.memory.borrow().get_addr(self.registers.get_hl()); byte8=self.sub8(byte8,0x01,false); self.memory.borrow_mut().set_addr(self.registers.get_hl(),byte8); if flag {self.registers.set_c();} else{self.registers.reset_c();} self.cycle=12; },

            //16 BIT ALU
            
            //ADD HL 
            0x09 => { flag=self.registers.is_set_z(); byte=self.add16(self.registers.get_hl(),self.registers.get_bc(),false); self.registers.set_hl(byte); if flag { self.registers.set_z(); } else {self.registers.reset_z();} self.cycle=8; },
            0x19 => { flag=self.registers.is_set_z(); byte=self.add16(self.registers.get_hl(),self.registers.get_de(),false); self.registers.set_hl(byte); if flag { self.registers.set_z(); } else {self.registers.reset_z();} self.cycle=8; },
            0x29 => { flag=self.registers.is_set_z(); byte=self.add16(self.registers.get_hl(),self.registers.get_hl(),false); self.registers.set_hl(byte); if flag { self.registers.set_z(); } else {self.registers.reset_z();} self.cycle=8; },
            0x39 => { flag=self.registers.is_set_z(); byte=self.add16(self.registers.get_hl(),self.registers.sp,false); self.registers.set_hl(byte); if flag { self.registers.set_z(); } else {self.registers.reset_z();} self.cycle=8; },
            
            //ADD SP
            0xE8 => { byte8=self.get_next_byte8(); self.registers.sp=self.add16(self.registers.sp,byte8 as u16,false); self.registers.reset_z(); self.registers.reset_n(); self.cycle=16; },
            
            //INC
            0x03 => { byte8=self.registers.f; byte=self.add16(self.registers.get_bc(),0x0001,false); self.registers.set_bc(byte); self.registers.f=byte8; self.cycle=8; },
            0x13 => { byte8=self.registers.f; byte=self.add16(self.registers.get_de(),0x0001,false); self.registers.set_de(byte); self.registers.f=byte8; self.cycle=8; },
            0x23 => { byte8=self.registers.f; byte=self.add16(self.registers.get_hl(),0x0001,false); self.registers.set_hl(byte); self.registers.f=byte8; self.cycle=8; },
            0x33 => { byte8=self.registers.f; byte=self.add16(self.registers.sp,0x0001,false); self.registers.sp=byte; self.registers.f=byte8; self.cycle=8; },
            
            //DEC
            0x0B => { byte8=self.registers.f; byte=self.sub16(self.registers.get_bc(),0x0001,false); self.registers.set_bc(byte); self.registers.f=byte8; self.cycle=8; },
            0x1B => { byte8=self.registers.f; byte=self.sub16(self.registers.get_de(),0x0001,false); self.registers.set_de(byte); self.registers.f=byte8; self.cycle=8; },
            0x2B => { byte8=self.registers.f; byte=self.sub16(self.registers.get_hl(),0x0001,false); self.registers.set_hl(byte); self.registers.f=byte8; self.cycle=8; },
            0x3B => { byte8=self.registers.f; byte=self.sub16(self.registers.sp,0x0001,false); self.registers.sp=byte; self.registers.f=byte8; self.cycle=8; },
            
            //MISCELLANEOUS
            //SWAP 
            0xCB => {
                opcode_cb=self.get_next_byte8();
                match opcode_cb{
                    //RL C [] (check)
                    0x11 => { 
                        self.registers.reset_flags(); 
                        self.registers.c = self.registers.c.rotate_left(1);
                        if self.registers.c == 0x0 { self.registers.set_z(); }
                        if self.registers.c & 0x1 == 0x1 { self.registers.set_c(); }
                        self.cycle=8; 
                    }

                    0x37 => { self.registers.reset_flags(); self.registers.a=((self.registers.a&0x0f)<<4)|((self.registers.a&0xf0)>>4); if self.registers.a==0x0 {self.registers.set_z();} self.cycle=8; },
                    0x30 => { self.registers.reset_flags(); self.registers.b=((self.registers.b&0x0f)<<4)|((self.registers.b&0xf0)>>4); if self.registers.b==0x0 {self.registers.set_z();} self.cycle=8; },
                    0x31 => { self.registers.reset_flags(); self.registers.c=((self.registers.c&0x0f)<<4)|((self.registers.c&0xf0)>>4); if self.registers.c==0x0 {self.registers.set_z();} self.cycle=8; },
                    0x32 => { self.registers.reset_flags(); self.registers.d=((self.registers.d&0x0f)<<4)|((self.registers.d&0xf0)>>4); if self.registers.d==0x0 {self.registers.set_z();} self.cycle=8; },
                    0x33 => { self.registers.reset_flags(); self.registers.e=((self.registers.e&0x0f)<<4)|((self.registers.e&0xf0)>>4); if self.registers.e==0x0 {self.registers.set_z();} self.cycle=8; },
                    0x34 => { self.registers.reset_flags(); self.registers.h=((self.registers.h&0x0f)<<4)|((self.registers.h&0xf0)>>4); if self.registers.h==0x0 {self.registers.set_z();} self.cycle=8; },
                    0x35 => { self.registers.reset_flags(); self.registers.l=((self.registers.l&0x0f)<<4)|((self.registers.l&0xf0)>>4); if self.registers.l==0x0 {self.registers.set_z();} self.cycle=8; },
                    //CHECK
                    0x36 => { self.registers.reset_flags(); byte=self.registers.get_hl(); byte8=self.memory.borrow().get_addr(byte) ; byte8=((byte8&0x0f)<<4)|((byte8&0xf0)>>4); self.memory.borrow_mut().set_addr(byte,byte8); if byte8==0 { self.registers.set_z(); } self.cycle=16;},
                    //BIT 7 H [- 1 0 CP]
                    0x7C => { 
                        if (self.registers.h>>7) == 0x0 { 
                            self.registers.set_z(); 
                        } else { 
                            self.registers.reset_z(); 
                        } self.registers.set_h(); self.registers.reset_n();  self.cycle=8;
                    },
                    _ => panic!("Opcode missing in CPU CB {:#0x?}",opcode_cb),
                };
            },
            //DAA
            0x27 => { self.registers.a=self.daa(self.registers.a); self.registers.reset_h(); self.cycle=4; },
            //CPL
            0x2F => { self.registers.a=!self.registers.a; self.registers.set_n(); self.registers.set_h(); self.cycle=4; },
            //CCF
            0x3F => { if self.registers.is_set_c() {self.registers.reset_c();} else {self.registers.set_c();} self.registers.reset_n(); self.registers.reset_h(); self.cycle=4; },
            //SCF
            0x37 => { self.registers.set_c(); self.registers.reset_n(); self.registers.reset_h(); self.cycle=4; },
            //NOP (there just for formality)
            0x00 => { self.cycle=4; },
            //HALT
            0x76 => { self.state = CPUState::Halt; self.cycle = 4; return; },
            //JP NN (check)
            0xC3 => {
                byte = self.get_next_byte16();
                self.registers.pc=byte; 
                self.cycle=16; 
            }
            //JR NZ i8 (check)
            0x20 => { 
                let flag = self.registers.get_flags();
                let byte = self.get_next_byte8() as i8; 
                if !self.registers.is_set_z(){ 
                    if byte.is_negative() { self.registers.pc = self.sub16(self.registers.pc,byte.abs() as u16,false) } 
                    else { self.registers.pc = self.add16(self.registers.pc,byte as u16,false); }
                    self.registers.set_flags(flag);    
                    self.cycle = 12;
                }else{
                    self.cycle = 8; 
                }
            }
            //JR Z,i8 [- - - -]
            0x28 => {
                let flag = self.registers.get_flags();
                let byte = self.get_next_byte8() as i8; 
                if self.registers.is_set_z(){ 
                    if byte.is_negative() { self.registers.pc = self.sub16(self.registers.pc,byte.abs() as u16,false) } 
                    else { self.registers.pc = self.add16(self.registers.pc,byte as u16,false); }
                    self.registers.set_flags(flag);    
                    self.cycle = 12;
                }else{
                    self.cycle = 8; 
                }
            }
            // JR i8 [- - - -]
            0x18 => {
                let flag = self.registers.get_flags();
                let byte8 = self.get_next_byte8() as i8; 
                if byte8.is_negative() { 
                    self.registers.pc = self.sub16(self.registers.pc,byte8.abs() as u16,false); 
                } else { 
                    self.registers.pc = self.add16(self.registers.pc,byte8 as u16,false); 
                }
                self.registers.set_flags(flag);    
                self.cycle = 12;
            }
            // CALL NN (check)
            0xCD => {
                self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,((self.registers.pc & 0xff00) >> 8) as u8);
                self.registers.sp-=1; self.memory.borrow_mut().set_addr(self.registers.sp,(self.registers.pc & 0x00ff) as u8);
                byte = self.get_next_byte16();
                self.registers.pc = byte;
                self.cycle = 24;
            }
            //RLA
            0x17 => { 
                byte8 = self.registers.get_flags(); 
                self.registers.set_flags(0x0);
                if self.registers.a >> 7 == 0x1 { self.registers.set_c(); } 
                self.registers.a = self.registers.a << 1 | (byte8 & 0b00010000) >> 4 ;
                self.cycle = 4;
            }
            //RET [- - - -]
            0xC9 => {
                byte = self.memory.borrow().get_addr(self.registers.sp) as u16;
                self.registers.sp+=0x1;
                byte = ((self.memory.borrow().get_addr(self.registers.sp) as u16) << 8) | byte;
                self.registers.sp+=0x1;
                self.registers.pc = byte;
                self.cycle = 16;
            }
            //TO REMOVE | FOR TESTING PURPOSE ONLY | ALTERNATIVE UNTIL INTERRUPT IS SETUP
            0xFF => {
                println!("Changing state");
                self.state = CPUState::Exit;
                return
            }
            _ => (panic!("Opcode missing in CPU : {:#0x?}",opcode))
        }
        self.registers.pc+=1;
        println!("{}",self.registers.pc);
    }
    //[Z 0 H C]
    fn add8(&mut self,a:u8,b:u8,carry:bool) -> u8{
        let mut carry_val:u8=0;
        if carry==true && (self.registers.is_set_c()) { carry_val=0x1; }
        self.registers.reset_flags();
        if (a & 0x0f)+(b & 0x0f) + carry_val > 0x0f { self.registers.set_h(); }
        match a.checked_add(b){ 
            Some(x) => match x.checked_add(carry_val){ 
                Some(x) => {if x==0x0 {self.registers.set_z();} x}, 
                None => {self.registers.set_c();(((a as u16)+(carry_val as u16)) & (0x00ff)) as u8}}, 
            None => {self.registers.set_c();let byte=(((a as u16)+(b as u16)) & (0x00ff)) as u8; match byte{ 
                0 => {self.registers.set_z();0x0},
                _ => byte} 
            }
        }
    }
    fn add16(&mut self,a:u16,b:u16,carry:bool) -> u16{
        let mut carry_val:u16=0;
        if carry==true && (self.registers.is_set_c()) { carry_val=0x1; }
        self.registers.reset_flags();
        if (a & 0xff)+(b & 0xff) + carry_val > 0xff { self.registers.set_h(); }
        match a.checked_add(b){ 
            Some(x) => match x.checked_add(carry_val){ 
                Some(x) => {if x==0x0 {self.registers.set_z();} x}, 
                None => {self.registers.set_c();(((a as u32)+(carry_val as u32)) & (0x00ff)) as u16}}, 
            None => {self.registers.set_c();let byte=(((a as u32)+(b as u32)) & (0x00ff)) as u16; match byte{ 
                0 => {self.registers.set_z();0x0},
                _ => byte} 
            }
        }
    }
    fn sub8(&mut self,a:u8,b:u8,carry:bool) -> u8{
        let mut carry_val:u8=0;
        if carry==true && (self.registers.is_set_c()) {carry_val=0x1; }
        self.registers.reset_flags(); self.registers.set_n();
        if a&0x0f < (b + carry_val)&0x0f { self.registers.set_h(); }
        if a < ((((b as u16)+(carry_val as u16)) & (0x00ff)) as u8) { self.registers.set_c(); }
        let result:u8=a.wrapping_sub(b).wrapping_sub(carry_val);
        match result{
            0 => {self.registers.set_z();0},
            _ => result
        }
    }
    fn sub16(&mut self,a:u16,b:u16,carry:bool) -> u16{
        let mut carry_val:u16=0;
        if carry==true && (self.registers.is_set_c()) {println!("inside carry"); carry_val=0x1; }
        self.registers.reset_flags(); self.registers.set_n();
        if a&0xff < (b + carry_val)&0xff { self.registers.set_h(); println!("h set");}
        if a < ((((b as u32)+(carry_val as u32)) & (0x00ff)) as u16) { self.registers.set_c(); }
        let result:u16=a.wrapping_sub(b).wrapping_sub(carry_val);
        match result{
            0 => {self.registers.set_z();0},
            _ => result
        }
    }
    fn daa(&mut self,a:u8) -> u8{
        let mut byte_ms:u8=a>>4;
        let mut byte_ls:u8=a&0x0f;
        if (byte_ls >9) | self.registers.is_set_h() {
            byte_ls+=0x06;
            if byte_ls&0x10 == 0x10{
                byte_ls=byte_ls&0x0f;
                byte_ms+=0x01;
                if byte_ms&0x10 == 0x10{
                    byte_ms=byte_ms&0x0f;
                    self.registers.set_c();
                }
            }
        }
        if (byte_ms>9) | self.registers.is_set_c(){
            byte_ms+=0x06;
            self.registers.set_c();
        }
        let byte=byte_ms<<4|byte_ls;
        match byte {
            0 => {self.registers.set_z(); 0x0},
            _ => byte
        }
    }
    fn print_flags(&self){
        println!("Z:{} N:{} H:{} C:{}",self.registers.get_z(),self.registers.get_n(),self.registers.get_h(),self.registers.get_c());
    }
    pub fn get_registers(&self) -> Registers{
        self.registers
    }
    pub fn get_cycles(&self) -> usize{
        self.cycle
    }
    pub fn get_state(&self) -> CPUState{
        self.state
    }
}

