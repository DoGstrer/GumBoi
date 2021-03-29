/*TO TEST -
add8 : 26 Intructions
add16 : 12 instructions
sub8 : 37 instructions
sub16 : 7 instructions
*/

// ANCHOR <Opcode> | <Instruction> | <[Z N H C]> | <Bytes> | <Cycles>

use std::sync::{Mutex,Arc};

use super::memory::Memory;
use super::registers::Flag;
use super::registers::Registers;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CPUState {
    Halt,
    Stop,
    Active,
    Exit,
}

pub struct CPU {
    registers: Registers,
    memory: Arc<Mutex<Memory>>,
    cycle: usize,
    state: CPUState,
    ime: bool // Interrupt Master Enable
}

impl CPU {
    pub fn new(memory: Arc<Mutex<Memory>>) -> CPU {
        CPU {
            registers: Registers::new(),
            cycle: 0,
            state: CPUState::Active,
            memory: memory,
            ime: false,
        }
    }
    fn get_next_byte8(&mut self) -> u8 {
        let mut byte:u8 = self.memory.lock().unwrap().get_addr(self.registers.pc);
        self.registers.pc += 1;
        byte
    }
    fn get_next_byte16(&mut self) -> u16 {
        let mut byte: u16;
        byte = self.memory.lock().unwrap().get_addr(self.registers.pc) as u16;
        self.registers.pc += 1;
        byte |= (self.memory.lock().unwrap().get_addr(self.registers.pc) as u16) << 8;
        self.registers.pc += 1;
        byte
    }
    pub fn execute(&mut self) {
        let opcode: u8 = self.memory.lock().unwrap().get_addr(self.registers.pc);
        //println!("{:#x?}", opcode);
        let mut opcode_cb: u8 = 0x0;
        let mut byte: u16 = 0x0;
        let mut byte8: u8 = 0x0;
        let mut flag: bool = false;

        self.registers.pc+=1;
        // SECTION CPU Instructions
        match opcode {
            //8 bit LD
            // LD r d8 [- - - -]
            0x3E => {
                self.registers.a = self.get_next_byte8();
                self.cycle = 8;
            }
            0x06 => {
                self.registers.b = self.get_next_byte8();
                self.cycle = 8;
            }
            0x0E => {
                self.registers.c = self.get_next_byte8();
                self.cycle = 8;
            }
            0x16 => {
                self.registers.d = self.get_next_byte8();
                self.cycle = 8;
            }
            0x1E => {
                self.registers.e = self.get_next_byte8();
                self.cycle = 8;
            }
            0x26 => {
                self.registers.h = self.get_next_byte8();
                self.cycle = 8;
            }
            0x2E => {
                self.registers.l = self.get_next_byte8();
                self.cycle = 8;
            }
            0x36 => {
                byte8 = self.get_next_byte8();
                self.memory
                    .lock()
                    .unwrap()
                    .set_addr(self.registers.get_hl(), byte8);
                self.cycle = 12;
            }

            0x7F => {
                self.registers.a = self.registers.a;
                self.cycle = 4;
            }
            0x78 => {
                self.registers.a = self.registers.b;
                self.cycle = 4;
            }
            0x79 => {
                self.registers.a = self.registers.c;
                self.cycle = 4;
            }
            0x7A => {
                self.registers.a = self.registers.d;
                self.cycle = 4;
            }
            0x7B => {
                self.registers.a = self.registers.e;
                self.cycle = 4;
            }
            0x7C => {
                self.registers.a = self.registers.h;
                self.cycle = 4;
            }
            0x7D => {
                self.registers.a = self.registers.l;
                self.cycle = 4;
            }
            0x7E => {
                self.registers.a = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x40 => {
                self.registers.b = self.registers.b;
                self.cycle = 4;
            }
            0x41 => {
                self.registers.b = self.registers.c;
                self.cycle = 4;
            }
            0x42 => {
                self.registers.b = self.registers.d;
                self.cycle = 4;
            }
            0x43 => {
                self.registers.b = self.registers.e;
                self.cycle = 4;
            }
            0x44 => {
                self.registers.b = self.registers.h;
                self.cycle = 4;
            }
            0x45 => {
                self.registers.b = self.registers.l;
                self.cycle = 4;
            }
            0x46 => {
                self.registers.b = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x48 => {
                self.registers.c = self.registers.b;
                self.cycle = 4;
            }
            0x49 => {
                self.registers.c = self.registers.c;
                self.cycle = 4;
            }
            0x4A => {
                self.registers.c = self.registers.d;
                self.cycle = 4;
            }
            0x4B => {
                self.registers.c = self.registers.e;
                self.cycle = 4;
            }
            0x4C => {
                self.registers.c = self.registers.h;
                self.cycle = 4;
            }
            0x4D => {
                self.registers.c = self.registers.l;
                self.cycle = 4;
            }
            0x4E => {
                self.registers.c = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x50 => {
                self.registers.d = self.registers.b;
                self.cycle = 4;
            }
            0x51 => {
                self.registers.d = self.registers.c;
                self.cycle = 4;
            }
            0x52 => {
                self.registers.d = self.registers.d;
                self.cycle = 4;
            }
            0x53 => {
                self.registers.d = self.registers.e;
                self.cycle = 4;
            }
            0x54 => {
                self.registers.d = self.registers.h;
                self.cycle = 4;
            }
            0x55 => {
                self.registers.d = self.registers.l;
                self.cycle = 4;
            }
            0x56 => {
                self.registers.d = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x58 => {
                self.registers.e = self.registers.b;
                self.cycle = 4;
            }
            0x59 => {
                self.registers.e = self.registers.c;
                self.cycle = 4;
            }
            0x5A => {
                self.registers.e = self.registers.d;
                self.cycle = 4;
            }
            0x5B => {
                self.registers.e = self.registers.e;
                self.cycle = 4;
            }
            0x5C => {
                self.registers.e = self.registers.h;
                self.cycle = 4;
            }
            0x5D => {
                self.registers.e = self.registers.l;
                self.cycle = 4;
            }
            0x5E => {
                self.registers.e = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x60 => {
                self.registers.h = self.registers.b;
                self.cycle = 4;
            }
            0x61 => {
                self.registers.h = self.registers.c;
                self.cycle = 4;
            }
            0x62 => {
                self.registers.h = self.registers.d;
                self.cycle = 4;
            }
            0x63 => {
                self.registers.h = self.registers.e;
                self.cycle = 4;
            }
            0x64 => {
                self.registers.h = self.registers.h;
                self.cycle = 4;
            }
            0x65 => {
                self.registers.h = self.registers.l;
                self.cycle = 4;
            }
            0x66 => {
                self.registers.h = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x68 => {
                self.registers.l = self.registers.b;
                self.cycle = 4;
            }
            0x69 => {
                self.registers.l = self.registers.c;
                self.cycle = 4;
            }
            0x6A => {
                self.registers.l = self.registers.d;
                self.cycle = 4;
            }
            0x6B => {
                self.registers.l = self.registers.e;
                self.cycle = 4;
            }
            0x6C => {
                self.registers.l = self.registers.h;
                self.cycle = 4;
            }
            0x6D => {
                self.registers.l = self.registers.l;
                self.cycle = 4;
            }
            0x6E => {
                self.registers.l = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.cycle = 8;
            }
            0x70 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.b);
                self.cycle = 8;
            }
            0x71 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.c);
                self.cycle = 8;
            }
            0x72 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.d);
                self.cycle = 8;
            }
            0x73 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.e);
                self.cycle = 8;
            }
            0x74 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.h);
                self.cycle = 8;
            }
            0x75 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.l);
                self.cycle = 8;
            }

            0x0A => {
                self.registers.a = self.memory.lock().unwrap().get_addr(self.registers.get_bc());
                self.cycle = 8;
            }
            0x1A => {
                self.registers.a = self.memory.lock().unwrap().get_addr(self.registers.get_de());
                self.cycle = 8;
            }
            0xFA => {
                byte = self.get_next_byte16();
                self.registers.a = self.memory.lock().unwrap().get_addr(byte);
                self.cycle = 16;
            }

            0x47 => {
                self.registers.b = self.registers.a;
                self.cycle = 4;
            }
            0x4F => {
                self.registers.c = self.registers.a;
                self.cycle = 4;
            }
            0x57 => {
                self.registers.d = self.registers.a;
                self.cycle = 4;
            }
            0x5F => {
                self.registers.e = self.registers.a;
                self.cycle = 4;
            }
            0x67 => {
                self.registers.h = self.registers.a;
                self.cycle = 4;
            }
            0x6F => {
                self.registers.l = self.registers.a;
                self.cycle = 4;
            }
            0x02 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_bc(), self.registers.a);
                self.cycle = 8;
            }
            0x12 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_de(), self.registers.a);
                self.cycle = 8;
            }
            0x77 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.a);
                self.cycle = 8;
            }
            0xEA => {
                byte = self.get_next_byte16();
                self.memory.lock().unwrap().set_addr(byte, self.registers.a);
                self.cycle = 16;
            }
            0xF2 => {
                self.registers.a = self.memory.lock().unwrap().get_addr(0xFF00 | (self.registers.c as u16));
                self.cycle = 8;
            }
            0xE2 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(0xFF00 + (self.registers.c as u16), self.registers.a);
                self.cycle = 8;
            }
            0x3A => {
                self.registers.a = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl() - 0x1);
                self.cycle = 8;
            }
            0x32 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.a);
                self.registers.set_hl(self.registers.get_hl() - 1);
                self.cycle = 8;
            }
            0x2A => {
                self.registers.a = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl() + 0x1);
                self.cycle = 8;
            }
            0x22 => {
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), self.registers.a);
                self.registers.set_hl(self.registers.get_hl() + 0x0001);
                self.cycle = 8;
            }
            0xE0 => {
                byte8 = self.get_next_byte8();
                self.memory
                    .lock().unwrap()
                    .set_addr(0xFF00 + (byte8 as u16), self.registers.a);
                self.cycle = 12;
            }
            0xF0 => {
                byte8 = self.get_next_byte8();
                self.registers.a = self.memory.lock().unwrap().get_addr(0xFF00 + (byte8 as u16));
                self.cycle = 12;
            }

            //16 bit LD
            0x01 => {
                byte = self.get_next_byte16();
                self.registers.set_bc(byte);
                self.cycle = 12;
            }
            0x11 => {
                byte = self.get_next_byte16();
                self.registers.set_de(byte);
                self.cycle = 12;
            }
            0x21 => {
                byte = self.get_next_byte16();
                self.registers.set_hl(byte);
                self.cycle = 12;
            }
            0x31 => {
                byte = self.get_next_byte16();
                self.registers.sp = byte;
                self.cycle = 12;
            }
            0xF9 => {
                self.registers.sp = self.registers.get_hl();
                self.cycle = 8
            }
            0xF8 => {
                byte = self.get_next_byte16();
                byte = self.add16(self.registers.sp, byte, false);
                self.registers.set_hl(byte);
                self.registers.reset_z();
                self.registers.reset_n();
                self.cycle = 12;
            }

            //To be reviewed
            0x08 => {
                byte = self.get_next_byte16();
                self.memory
                    .lock().unwrap()
                    .set_addr(byte, (self.registers.sp & 0x00ff) as u8);
                self.memory
                    .lock().unwrap()
                    .set_addr(byte + 1, (self.registers.sp >> 8) as u8);
                self.cycle = 20;
            }

            // SECTION Stack Operations
            // SECTION PUSH
            // 0xF5 -> PUSH AF | [- - - -] | 1 | 16 
            0xF5 => {
                self.push(self.registers.get_af());
                self.cycle = 16;
            }
            // 0xC5 -> PUSH BC | [- - - -] | 1 | 16
            0xC5 => {
                self.push(self.registers.get_bc());
                self.cycle = 16;
            }
            // 0xD5 -> PUSH DE | [- - - -] | 1 | 16
            0xD5 => {
                self.push(self.registers.get_de());
                self.cycle = 16;
            }
            // 0xE5 -> PUSH HL | [- - - -] | 1 | 16
            0xE5 => {
                self.push(self.registers.get_hl());
                self.cycle = 16;
            }
            // !SECTION
            // SECTION POP
            // 0xF1 -> POP AF | [- - - -] | 1 | 12
            0xF1 => {
                byte = self.pop();
                self.registers.set_af(byte);
                self.cycle = 12;
            }
            // 0xC1 -> POP BC | [- - - -] | 1 | 12
            0xC1 => {
                byte = self.pop();
                self.registers.set_bc(byte);
                self.cycle = 12;
            }
            // 0xD1 -> POP DE | [- - - -] | 1 | 12
            0xD1 => {
                byte = self.pop();
                self.registers.set_de(byte);
                self.cycle = 12;
            }
            // 0xE1 -> POP HL | [- - - -] | 1 | 12
            0xE1 => {
                byte = self.pop();
                self.registers.set_hl(byte);
                self.cycle = 12;
            }
            // !SECTION
            // !SECTION
            // 8 BIT ALU
            //ADD
            0x87 => {
                self.registers.a = self.add8(self.registers.a, self.registers.a, false);
                self.cycle = 4;
            }
            0x80 => {
                self.registers.a = self.add8(self.registers.a, self.registers.b, false);
                self.cycle = 4;
            }
            0x81 => {
                self.registers.a = self.add8(self.registers.a, self.registers.c, false);
                self.cycle = 4;
            }
            0x82 => {
                self.registers.a = self.add8(self.registers.a, self.registers.d, false);
                self.cycle = 4;
            }
            0x83 => {
                self.registers.a = self.add8(self.registers.a, self.registers.e, false);
                self.cycle = 4;
            }
            0x84 => {
                self.registers.a = self.add8(self.registers.a, self.registers.h, false);
                self.cycle = 4;
            }
            0x85 => {
                self.registers.a = self.add8(self.registers.a, self.registers.l, false);
                self.cycle = 4;
            }
            0x86 => {
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.a = self.add8(self.registers.a, byte8, false);
                self.cycle = 8;
            }
            0xC6 => {
                byte8 = self.get_next_byte8();
                self.registers.a = self.add8(self.registers.a, byte8, false);
                self.cycle = 8;
            }

            //ADD WITH CARRY
            0x8F => {
                self.registers.a = self.add8(self.registers.a, self.registers.a, true);
                self.cycle = 4;
            }
            0x88 => {
                self.registers.a = self.add8(self.registers.a, self.registers.b, true);
                self.cycle = 4;
            }
            0x89 => {
                self.registers.a = self.add8(self.registers.a, self.registers.c, true);
                self.cycle = 4;
            }
            0x8A => {
                self.registers.a = self.add8(self.registers.a, self.registers.d, true);
                self.cycle = 4;
            }
            0x8B => {
                self.registers.a = self.add8(self.registers.a, self.registers.e, true);
                self.cycle = 4;
            }
            0x8C => {
                self.registers.a = self.add8(self.registers.a, self.registers.h, true);
                self.cycle = 4;
            }
            0x8D => {
                self.registers.a = self.add8(self.registers.a, self.registers.l, true);
                self.cycle = 4;
            }
            0x8E => {
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.a = self.add8(self.registers.a, byte8, true);
                self.cycle = 8;
            }
            0xCE => {
                byte8 = self.get_next_byte8();
                self.registers.a = self.add8(self.registers.a, byte8, true);
                self.cycle = 8;
            }

            //SUB
            0x97 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.a, false);
                self.cycle = 4;
            }
            0x90 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.b, false);
                self.cycle = 4;
            }
            0x91 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.c, false);
                self.cycle = 4;
            }
            0x92 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.d, false);
                self.cycle = 4;
            }
            0x93 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.e, false);
                self.cycle = 4;
            }
            0x94 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.h, false);
                self.cycle = 4;
            }
            0x95 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.l, false);
                self.cycle = 4;
            }
            0x96 => {
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.a = self.sub8(self.registers.a, byte8, false);
                self.cycle = 8;
            }
            0xD6 => {
                byte8 = self.get_next_byte8();
                self.registers.a = self.sub8(self.registers.a, byte8, false);
                self.cycle = 8;
            }

            //SUB WITH BORROW
            0x9F => {
                self.registers.a = self.sub8(self.registers.a, self.registers.a, true);
                self.cycle = 4;
            }
            0x98 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.b, true);
                self.cycle = 4;
            }
            0x99 => {
                self.registers.a = self.sub8(self.registers.a, self.registers.c, true);
                self.cycle = 4;
            }
            0x9A => {
                self.registers.a = self.sub8(self.registers.a, self.registers.d, true);
                self.cycle = 4;
            }
            0x9B => {
                self.registers.a = self.sub8(self.registers.a, self.registers.e, true);
                self.cycle = 4;
            }
            0x9C => {
                self.registers.a = self.sub8(self.registers.a, self.registers.h, true);
                self.cycle = 4;
            }
            0x9D => {
                self.registers.a = self.sub8(self.registers.a, self.registers.l, true);
                self.cycle = 4;
            }
            0x9E => {
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.a = self.sub8(self.registers.a, byte8, true);
                self.cycle = 8;
            }
            0xDE => {
                byte8 = self.get_next_byte8();
                self.registers.a = self.sub8(self.registers.a, byte as u8, true);
                self.cycle = 8;
            }

            //LOGICAL OPERATIONS
            //AND
            0xA7 => {
                self.registers.a &= self.registers.a;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA0 => {
                self.registers.a &= self.registers.b;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA1 => {
                self.registers.a &= self.registers.c;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA2 => {
                self.registers.a &= self.registers.d;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA3 => {
                self.registers.a &= self.registers.e;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA4 => {
                self.registers.a &= self.registers.h;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA5 => {
                self.registers.a &= self.registers.l;
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA6 => {
                self.registers.a &= self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 8;
            }
            0xE6 => {
                self.registers.a &= self.get_next_byte8();
                self.registers.reset_flags();
                self.registers.set_h();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 8;
            }

            //OR
            0xB7 => {
                self.registers.a |= self.registers.a;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB0 => {
                self.registers.a |= self.registers.b;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB1 => {
                self.registers.a |= self.registers.c;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB2 => {
                self.registers.a |= self.registers.d;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB3 => {
                self.registers.a |= self.registers.e;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB4 => {
                self.registers.a |= self.registers.h;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB5 => {
                self.registers.a |= self.registers.l;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xB6 => {
                self.registers.a |= self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 8;
            }
            0xF6 => {
                byte8 = self.get_next_byte8();
                self.registers.a |= byte8;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 8;
            }

            //XOR
            0xAF => {
                self.registers.a ^= self.registers.a;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA8 => {
                self.registers.a ^= self.registers.b;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xA9 => {
                self.registers.a ^= self.registers.c;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xAA => {
                self.registers.a ^= self.registers.d;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xAB => {
                self.registers.a ^= self.registers.e;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xAC => {
                self.registers.a ^= self.registers.h;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xAD => {
                self.registers.a ^= self.registers.l;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 4;
            }
            0xAE => {
                self.registers.a ^= self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 8;
            }
            0xEE => {
                byte8 = self.get_next_byte8();
                self.registers.a ^= byte8;
                self.registers.reset_flags();
                if self.registers.a == 0x0 {
                    self.registers.set_z();
                }
                self.cycle = 8;
            }

            //CP
            0xBF => {
                self.sub8(self.registers.a, self.registers.a, false);
                self.cycle = 4;
            }
            0xB8 => {
                self.sub8(self.registers.a, self.registers.b, false);
                self.cycle = 4;
            }
            0xB9 => {
                self.sub8(self.registers.a, self.registers.c, false);
                self.cycle = 4;
            }
            0xBA => {
                self.sub8(self.registers.a, self.registers.d, false);
                self.cycle = 4;
            }
            0xBB => {
                self.sub8(self.registers.a, self.registers.e, false);
                self.cycle = 4;
            }
            0xBC => {
                self.sub8(self.registers.a, self.registers.h, false);
                self.cycle = 4;
            }
            0xBD => {
                self.sub8(self.registers.a, self.registers.l, false);
                self.cycle = 4;
            }
            0xBE => {
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                self.sub8(self.registers.a, byte8, false);
                self.cycle = 8;
            }
            0xFE => {
                byte8 = self.get_next_byte8();
                self.sub8(self.registers.a, byte8 as u8, false);
                self.cycle = 8;
            }

            //INC
            // INC A [Z 0 H -]
            0x3C => {
                flag = self.registers.is_set_c();
                self.registers.a = self.add8(self.registers.a, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x04 => {
                flag = self.registers.is_set_c();
                self.registers.b = self.add8(self.registers.b, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x0C => {
                flag = self.registers.is_set_c();
                self.registers.c = self.add8(self.registers.c, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x14 => {
                flag = self.registers.is_set_c();
                self.registers.d = self.add8(self.registers.d, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x1C => {
                flag = self.registers.is_set_c();
                self.registers.e = self.add8(self.registers.e, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x24 => {
                flag = self.registers.is_set_c();
                self.registers.h = self.add8(self.registers.h, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x2C => {
                flag = self.registers.is_set_c();
                self.registers.l = self.add8(self.registers.l, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 4;
            }
            0x34 => {
                flag = self.registers.is_set_c();
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                byte8 = self.add8(byte8, 0x01, false);
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), byte8);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.registers.reset_n();
                self.cycle = 12;
            }
            //DEC
            0x3D => {
                flag = self.registers.is_set_c();
                self.registers.a = self.sub8(self.registers.a, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x05 => {
                flag = self.registers.is_set_c();
                self.registers.b = self.sub8(self.registers.b, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x0D => {
                flag = self.registers.is_set_c();
                self.registers.c = self.sub8(self.registers.c, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x15 => {
                flag = self.registers.is_set_c();
                self.registers.d = self.sub8(self.registers.d, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x1D => {
                flag = self.registers.is_set_c();
                self.registers.e = self.sub8(self.registers.e, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x25 => {
                flag = self.registers.is_set_c();
                self.registers.h = self.sub8(self.registers.h, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x2D => {
                flag = self.registers.is_set_c();
                self.registers.l = self.sub8(self.registers.l, 0x01, false);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 4;
            }
            0x35 => {
                flag = self.registers.is_set_c();
                byte8 = self.memory.lock().unwrap().get_addr(self.registers.get_hl());
                byte8 = self.sub8(byte8, 0x01, false);
                self.memory
                    .lock().unwrap()
                    .set_addr(self.registers.get_hl(), byte8);
                if flag {
                    self.registers.set_c();
                } else {
                    self.registers.reset_c();
                }
                self.cycle = 12;
            }

            //16 BIT ALU

            //ADD HL
            0x09 => {
                flag = self.registers.is_set_z();
                byte = self.add16(self.registers.get_hl(), self.registers.get_bc(), false);
                self.registers.set_hl(byte);
                if flag {
                    self.registers.set_z();
                } else {
                    self.registers.reset_z();
                }
                self.cycle = 8;
            }
            0x19 => {
                flag = self.registers.is_set_z();
                byte = self.add16(self.registers.get_hl(), self.registers.get_de(), false);
                self.registers.set_hl(byte);
                if flag {
                    self.registers.set_z();
                } else {
                    self.registers.reset_z();
                }
                self.cycle = 8;
            }
            0x29 => {
                flag = self.registers.is_set_z();
                byte = self.add16(self.registers.get_hl(), self.registers.get_hl(), false);
                self.registers.set_hl(byte);
                if flag {
                    self.registers.set_z();
                } else {
                    self.registers.reset_z();
                }
                self.cycle = 8;
            }
            0x39 => {
                flag = self.registers.is_set_z();
                byte = self.add16(self.registers.get_hl(), self.registers.sp, false);
                self.registers.set_hl(byte);
                if flag {
                    self.registers.set_z();
                } else {
                    self.registers.reset_z();
                }
                self.cycle = 8;
            }

            //ADD SP
            0xE8 => {
                byte8 = self.get_next_byte8();
                self.registers.sp = self.add16(self.registers.sp, byte8 as u16, false);
                self.registers.reset_z();
                self.registers.reset_n();
                self.cycle = 16;
            }

            //INC
            0x03 => {
                byte8 = self.registers.f;
                byte = self.add16(self.registers.get_bc(), 0x0001, false);
                self.registers.set_bc(byte);
                self.registers.f = byte8;
                self.cycle = 8;
            }
            0x13 => {
                byte8 = self.registers.f;
                byte = self.add16(self.registers.get_de(), 0x0001, false);
                self.registers.set_de(byte);
                self.registers.f = byte8;
                self.cycle = 8;
            }
            0x23 => {
                byte8 = self.registers.f;
                byte = self.add16(self.registers.get_hl(), 0x0001, false);
                self.registers.set_hl(byte);
                self.registers.f = byte8;
                self.cycle = 8;
            }
            0x33 => {
                byte8 = self.registers.f;
                byte = self.add16(self.registers.sp, 0x0001, false);
                self.registers.sp = byte;
                self.registers.f = byte8;
                self.cycle = 8;
            }

            //DEC
            0x0B => {
                byte8 = self.registers.f;
                byte = self.sub16(self.registers.get_bc(), 0x0001, false);
                self.registers.set_bc(byte);
                self.registers.f = byte8;
                self.cycle = 8;
            }
            0x1B => {
                byte8 = self.registers.f;
                byte = self.sub16(self.registers.get_de(), 0x0001, false);
                self.registers.set_de(byte);
                self.registers.f = byte8;
                self.cycle = 8;
            }
            0x2B => {
                byte8 = self.registers.f;
                byte = self.sub16(self.registers.get_hl(), 0x0001, false);
                self.registers.set_hl(byte);
                self.registers.f = byte8;
                self.cycle = 8;
            }
            0x3B => {
                byte8 = self.registers.f;
                byte = self.sub16(self.registers.sp, 0x0001, false);
                self.registers.sp = byte;
                self.registers.f = byte8;
                self.cycle = 8;
            }

            //MISCELLANEOUS
            //SWAP
            0xCB => {
                opcode_cb = self.get_next_byte8();
                match opcode_cb {
                    //RL C [] (check)
                    0x11 => {
                        self.registers.reset_flags();
                        self.registers.c = self.registers.c.rotate_left(1);
                        if self.registers.c == 0x0 {
                            self.registers.set_z();
                        }
                        if self.registers.c & 0x1 == 0x1 {
                            self.registers.set_c();
                        }
                        self.cycle = 8;
                    }

                    0x37 => {
                        self.registers.reset_flags();
                        self.registers.a =
                            ((self.registers.a & 0x0f) << 4) | ((self.registers.a & 0xf0) >> 4);
                        if self.registers.a == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    0x30 => {
                        self.registers.reset_flags();
                        self.registers.b =
                            ((self.registers.b & 0x0f) << 4) | ((self.registers.b & 0xf0) >> 4);
                        if self.registers.b == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    0x31 => {
                        self.registers.reset_flags();
                        self.registers.c =
                            ((self.registers.c & 0x0f) << 4) | ((self.registers.c & 0xf0) >> 4);
                        if self.registers.c == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    0x32 => {
                        self.registers.reset_flags();
                        self.registers.d =
                            ((self.registers.d & 0x0f) << 4) | ((self.registers.d & 0xf0) >> 4);
                        if self.registers.d == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    0x33 => {
                        self.registers.reset_flags();
                        self.registers.e =
                            ((self.registers.e & 0x0f) << 4) | ((self.registers.e & 0xf0) >> 4);
                        if self.registers.e == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    0x34 => {
                        self.registers.reset_flags();
                        self.registers.h =
                            ((self.registers.h & 0x0f) << 4) | ((self.registers.h & 0xf0) >> 4);
                        if self.registers.h == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    0x35 => {
                        self.registers.reset_flags();
                        self.registers.l =
                            ((self.registers.l & 0x0f) << 4) | ((self.registers.l & 0xf0) >> 4);
                        if self.registers.l == 0x0 {
                            self.registers.set_z();
                        }
                        self.cycle = 8;
                    }
                    //CHECK
                    0x36 => {
                        self.registers.reset_flags();
                        byte = self.registers.get_hl();
                        byte8 = self.memory.lock().unwrap().get_addr(byte);
                        byte8 = ((byte8 & 0x0f) << 4) | ((byte8 & 0xf0) >> 4);
                        self.memory.lock().unwrap().set_addr(byte, byte8);
                        if byte8 == 0 {
                            self.registers.set_z();
                        }
                        self.cycle = 16;
                    }
                    //BIT 7 H [- 1 0 CP]
                    0x7C => {
                        if (self.registers.h >> 7) == 0x0 {
                            self.registers.set_z();
                        } else {
                            self.registers.reset_z();
                        }
                        self.registers.set_h();
                        self.registers.reset_n();
                        self.cycle = 8;
                    }
                    _ => panic!("Opcode missing in CPU CB {:#0x?}", opcode_cb),
                };
            }
            //DAA
            0x27 => {
                self.registers.a = self.daa(self.registers.a);
                self.registers.reset_h();
                self.cycle = 4;
            }
            //CPL
            0x2F => {
                self.registers.a = !self.registers.a;
                self.registers.set_n();
                self.registers.set_h();
                self.cycle = 4;
            }
            //CCF
            0x3F => {
                if self.registers.is_set_c() {
                    self.registers.reset_c();
                } else {
                    self.registers.set_c();
                }
                self.registers.reset_n();
                self.registers.reset_h();
                self.cycle = 4;
            }
            //SCF
            0x37 => {
                self.registers.set_c();
                self.registers.reset_n();
                self.registers.reset_h();
                self.cycle = 4;
            }
            //NOP (there just for formality)
            0x00 => {
                self.cycle = 4;
            }
            //HALT
            0x76 => {
                self.state = CPUState::Halt;
                //self.cycle = 4;
            }
            //JP NN (check)
            0xC3 => {
                byte = self.get_next_byte16();
                self.registers.pc = byte;
                self.cycle = 16;
            }
            //JR NZ i8 (check)
            0x20 => {
                let flag = self.registers.get_flags();
                let byte = self.get_next_byte8() as i8;
                if !self.registers.is_set_z() {
                    if byte.is_negative() {
                        self.registers.pc = self.sub16(self.registers.pc, byte.abs() as u16, false)
                    } else {
                        self.registers.pc = self.add16(self.registers.pc, byte as u16, false);
                    }
                    self.registers.set_flags(flag);
                    self.cycle = 12;
                } else {
                    self.cycle = 8;
                }
            }
            //JR Z,i8 [- - - -]
            0x28 => {
                let flag = self.registers.get_flags();
                let byte = self.get_next_byte8() as i8;
                if self.registers.is_set_z() {
                    if byte.is_negative() {
                        self.registers.pc = self.sub16(self.registers.pc, byte.abs() as u16, false)
                    } else {
                        self.registers.pc = self.add16(self.registers.pc, byte as u16, false);
                    }
                    self.registers.set_flags(flag);
                    self.cycle = 12;
                } else {
                    self.cycle = 8;
                }
            }
            // JR i8 [- - - -]
            0x18 => {
                let flag = self.registers.get_flags();
                let byte8 = self.get_next_byte8() as i8;
                if byte8.is_negative() {
                    self.registers.pc = self.sub16(self.registers.pc, byte8.abs() as u16, false);
                } else {
                    self.registers.pc = self.add16(self.registers.pc, byte8 as u16, false);
                }
                self.registers.set_flags(flag);
                self.cycle = 12;
            }
            // SECTION CALL Instructions
            // ANCHOR CALL a16 | [- - - -] | 3 | 24
            0xCD => {
                byte = self.get_next_byte16();
                self.push(self.registers.pc);
                self.registers.pc = byte;
                self.cycle = 24;
            }
            // ANCHOR CALL NZ, a16 | [- - - -] | 3 | 24/12
            0xC4 => {
                byte = self.get_next_byte16();
                if !self.registers.is_set_z(){
                    self.push(self.registers.pc);
                    self.registers.pc = byte;
                    self.cycle = 24;
                }else{
                    self.cycle = 12;
                }
            } 
            // ANCHOR CALL Z, a16 | [- - - -] | 3 | 24/12
            0xCC => {
                byte = self.get_next_byte16();
                if self.registers.is_set_z(){
                    self.push(self.registers.pc);
                    self.registers.pc = byte;
                    self.cycle = 24;
                }else{
                    self.cycle = 12;
                }
            } 
            // ANCHOR CALL NC, a16 | [- - - -] | 3 | 24/12
            0xD4 => {
                byte = self.get_next_byte16();
                if !self.registers.is_set_c(){
                    self.push(self.registers.pc);
                    self.registers.pc = byte;
                    self.cycle = 24;
                }else{
                    self.cycle = 12;
                }
            } 
            // ANCHOR CALL C, a16 | [- - - -] | 3 | 24/12
            0xDC => {
                byte = self.get_next_byte16();
                if self.registers.is_set_c(){
                    self.push(self.registers.pc);
                    self.registers.pc = byte;
                    self.cycle = 24;
                }else{
                    self.cycle = 12;
                }
            } 
            // !SECTION
            //RLA
            0x17 => {
                byte8 = self.registers.get_flags();
                self.registers.set_flags(0x0);
                if self.registers.a >> 7 == 0x1 {
                    self.registers.set_c();
                }
                self.registers.a = self.registers.a << 1 | (byte8 & 0b00010000) >> 4;
                self.cycle = 4;
            }
            // SECTION Return Instructions
            // ANCHOR RET | [- - - -] | 1 | 16
            0xC9 => {
                byte = self.pop();
                self.registers.pc = byte;
                self.cycle = 16;
            }
            // ANCHOR RET NZ | [- - - -] | 1 | 20/8
            0xC0 => {
                if !self.registers.is_set_z(){
                    byte = self.pop();
                    self.registers.pc = byte;
                    self.cycle = 20
                }
                else{
                    self.cycle = 8;
                }
            } 
            // ANCHOR RET Z | [- - - -] | 1 | 20/8
            0xC8 => {                 
                if self.registers.is_set_z(){
                    byte = self.pop();
                    self.registers.pc = byte;
                    self.cycle = 20
                }else{
                self.cycle = 8;
                }
            }
            // ANCHOR RET NC | [- - - -] | 1 | 20/8
            0xD0 => {
                if !self.registers.is_set_c(){
                    byte = self.pop();
                    self.registers.pc = byte;
                    self.cycle = 20
                }
                else{
                    self.cycle = 8;
                }
            } 
            // ANCHOR RET C | [- - - -] | 1 | 20/8
            0xD8 => {                 
                if self.registers.is_set_c(){
                    byte = self.pop();
                    self.registers.pc = byte;
                    self.cycle = 20
                }else{
                self.cycle = 8;
                }
            }
            // ANCHOR RETI | [- - - -] | 1 | 16
            0xD9 => {
                byte = self.pop();
                self.registers.pc = byte;
                self.ime = true;
                self.cycle = 16;
            }
            // !SECTION

            // SECTION Reset
            // ANCHOR RST 00H | [- - - -] | 1 | 16
            0xC7 => {
                self.push(self.registers.pc);
                self.registers.pc = 0x00;
                self.cycle = 16;
            }
            // ANCHOR RST 08H | [- - - -] | 1 | 16
            0xCF => {
                self.push(self.registers.pc);
                self.registers.pc = 0x08;
                self.cycle = 16;
            }
            // ANCHOR RST 10H | [- - - -] | 1 | 16
            0xD7 => {
                self.push(self.registers.pc);
                self.registers.pc = 0x10;
                self.cycle = 16;
            }
            // ANCHOR RST 18H | [- - - -] | 1 | 16
            0xDF => {
                self.push(self.registers.pc);
                self.registers.pc = 0x18;
                self.cycle = 16;
            }
            // ANCHOR RST 20H | [- - - -] | 1 | 16
            0xE7 => {
                self.push(self.registers.pc);
                self.registers.pc = 0x20;
                self.cycle = 16;
            }
            // ANCHOR RST 28H | [- - - -] | 1 | 16
            0xEF => {
                self.push(self.registers.pc);
                self.registers.pc = 0x28;
                self.cycle = 16;
            }
            // ANCHOR RST 30H | [- - - -] | 1 | 16
            0xF7 => {
                self.push(self.registers.pc);
                self.registers.pc = 0x30;
                self.cycle = 16;
            }
            // ANCHOR RST 38H | [- - - -] | 1 | 16 
            0xFF => {
                self.push(self.registers.pc);
                self.registers.pc = 0x38;
                self.cycle = 16;
            }
            // !SECTION

            _ => (panic!("Opcode missing in CPU : {:#0x?}", opcode)),
        }
        // !SECTION
    }
    //[Z 0 H C]
    fn add8(&mut self, a: u8, b: u8, carry: bool) -> u8 {
        let mut carry_val: u8 = 0;
        if carry == true && (self.registers.is_set_c()) {
            carry_val = 0x1;
        }
        self.registers.reset_flags();
        if (a & 0x0f) + (b & 0x0f) + carry_val > 0x0f {
            self.registers.set_h();
        }
        match a.checked_add(b) {
            Some(x) => match x.checked_add(carry_val) {
                Some(x) => {
                    if x == 0x0 {
                        self.registers.set_z();
                    }
                    x
                }
                None => {
                    self.registers.set_c();
                    (((a as u16) + (carry_val as u16)) & (0x00ff)) as u8
                }
            },
            None => {
                self.registers.set_c();
                let byte = (((a as u16) + (b as u16)) & (0x00ff)) as u8;
                match byte {
                    0 => {
                        self.registers.set_z();
                        0x0
                    }
                    _ => byte,
                }
            }
        }
    }
    fn add16(&mut self, a: u16, b: u16, carry: bool) -> u16 {
        let mut carry_val: u16 = 0;
        if carry == true && (self.registers.is_set_c()) {
            carry_val = 0x1;
        }
        self.registers.reset_flags();
        if (a & 0xff) + (b & 0xff) + carry_val > 0xff {
            self.registers.set_h();
        }
        match a.checked_add(b) {
            Some(x) => match x.checked_add(carry_val) {
                Some(x) => {
                    if x == 0x0 {
                        self.registers.set_z();
                    }
                    x
                }
                None => {
                    self.registers.set_c();
                    (((a as u32) + (carry_val as u32)) & (0x00ff)) as u16
                }
            },
            None => {
                self.registers.set_c();
                let byte = (((a as u32) + (b as u32)) & (0x00ff)) as u16;
                match byte {
                    0 => {
                        self.registers.set_z();
                        0x0
                    }
                    _ => byte,
                }
            }
        }
    }
    fn sub8(&mut self, a: u8, b: u8, carry: bool) -> u8 {
        let mut carry_val: u8 = 0;
        if carry == true && (self.registers.is_set_c()) {
            carry_val = 0x1;
        }
        self.registers.reset_flags();
        self.registers.set_n();
        if a & 0x0f < (b + carry_val) & 0x0f {
            self.registers.set_h();
        }
        if a < ((((b as u16) + (carry_val as u16)) & (0x00ff)) as u8) {
            self.registers.set_c();
        }
        let result: u8 = a.wrapping_sub(b).wrapping_sub(carry_val);
        match result {
            0 => {
                self.registers.set_z();
                0
            }
            _ => result,
        }
    }
    fn sub16(&mut self, a: u16, b: u16, carry: bool) -> u16 {
        let mut carry_val: u16 = 0;
        if carry == true && (self.registers.is_set_c()) {
            println!("inside carry");
            carry_val = 0x1;
        }
        self.registers.reset_flags();
        self.registers.set_n();
        if a & 0xff < (b + carry_val) & 0xff {
            self.registers.set_h();
            println!("h set");
        }
        if a < ((((b as u32) + (carry_val as u32)) & (0x00ff)) as u16) {
            self.registers.set_c();
        }
        let result: u16 = a.wrapping_sub(b).wrapping_sub(carry_val);
        match result {
            0 => {
                self.registers.set_z();
                0
            }
            _ => result,
        }
    }
    fn daa(&mut self, a: u8) -> u8 {
        let mut byte_ms: u8 = a >> 4;
        let mut byte_ls: u8 = a & 0x0f;
        if (byte_ls > 9) | self.registers.is_set_h() {
            byte_ls += 0x06;
            if byte_ls & 0x10 == 0x10 {
                byte_ls = byte_ls & 0x0f;
                byte_ms += 0x01;
                if byte_ms & 0x10 == 0x10 {
                    byte_ms = byte_ms & 0x0f;
                    self.registers.set_c();
                }
            }
        }
        if (byte_ms > 9) | self.registers.is_set_c() {
            byte_ms += 0x06;
            self.registers.set_c();
        }
        let byte = byte_ms << 4 | byte_ls;
        match byte {
            0 => {
                self.registers.set_z();
                0x0
            }
            _ => byte,
        }
    }
    fn print_flags(&self) {
        println!(
            "Z:{} N:{} H:{} C:{}",
            self.registers.get_z(),
            self.registers.get_n(),
            self.registers.get_h(),
            self.registers.get_c()
        );
    }
    pub fn get_registers(&self) -> Registers {
        self.registers
    }
    pub fn get_cycles(&self) -> usize {
        self.cycle
    }
    pub fn get_state(&self) -> CPUState {
        self.state
    }
}

// SECTION CPU Stack Trait
pub trait Stack{
    fn push(&mut self,a16: u16);
    fn pop(&mut self) -> u16;
}

impl Stack for CPU{
    fn push(&mut self, a16: u16){
        self.registers.sp-=1;
        self.memory.lock().unwrap().set_addr(self.registers.sp,(a16>>8) as u8);
        self.registers.sp-=1;
        self.memory.lock().unwrap().set_addr(self.registers.sp,(a16 & 0x00ff) as u8);
    }
    fn pop(&mut self) -> u16{
        let mut byte: u16;
        byte = self.memory.lock().unwrap().get_addr(self.registers.sp) as u16;
        self.registers.sp+=1;
        byte |= (self.memory.lock().unwrap().get_addr(self.registers.sp) as u16) << 8;
        self.registers.sp+=1;
        byte
    }
}
// !SECTION

// SECTION CPU Test Cases
#[cfg(test)]
mod cpu_intruction_tests {

    const SET_Z: u8 = 0b10000000;
    const SET_N: u8 = 0b01000000;
    const SET_H: u8 = 0b00100000;
    const SET_C: u8 = 0b00010000;

    const RESET_Z: u8 = 0b01111111;
    const RESET_N: u8 = 0b10111111;
    const RESET_H: u8 = 0b11011111;
    const RESET_C: u8 = 0b11101111;

    const EMPTY_REGISTERS: Registers = Registers {
        a: 0x0,
        b: 0x0,
        c: 0x0,
        d: 0x0,
        e: 0x0,
        f: 0x0,
        h: 0x0,
        l: 0x0,
        sp: 0x0,
        pc: 0x0,
    };

    use std::sync::{Mutex,Arc};
    use super::CPUState;
    use super::CPU;
    use super::Memory;
    use super::Registers;
    use std::convert::TryInto;

    // SECTION Macros
    macro_rules! registers {
        ($($register_name:ident:$register_value:expr),*) => {
            {
                Registers{
                    $(
                        $register_name: $register_value,
                    )*
                    ..EMPTY_REGISTERS
                }
            }
        }
    }

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

    macro_rules! test_case {
        ($test_name:ident|$initial_state:expr,$expected_state:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(get_next_state($initial_state), $expected_state)
            }
        };
    }
    // !SECTION
    fn get_next_state(current_state: (Registers, Memory, usize)) -> (Registers, Memory, usize) {
        let memory = Arc::new(Mutex::new(current_state.1));
        let memory_1 = Arc::clone(&memory);
        let mut cpu = CPU {
            registers: current_state.0,
            memory: memory,
            cycle: current_state.2,
            state: CPUState::Active,
            ime: false,
        };
        while cpu.state == CPUState::Active{
            cpu.execute();
        }
        let mem = *cpu.memory.lock().unwrap();
        (cpu.get_registers(), mem, cpu.get_cycles())
    }

    // ANCHOR 0x76 | HALT | [- - - -] | 1 | 4
    test_case![
        Ox76 | (registers!(), memory!(0x0=>0x76), 0),
        (registers!(pc:1), memory!(0x0=>0x76), 0)
    ];
    // ANCHOR 0xCB 0x11 | RL C | [Z 0 0 C] | 2 | 8
    test_case![
        OxCB_Ox11
            | (
                registers!(c: 0x85),
                memory!(0x0=>0xCB,0x1=>0x11,0x2=>0x76),
                0
            ),
        (
            registers!(c: 0x0B,f:SET_C,pc:0x3),
            memory!(0x0=>0xCB,0x1=>0x11,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x17 | RLA | [0 0 0 C] | 1 | 4
    test_case![
        Ox17 | (registers!(a:0x95), memory!(0x0=>0x17,0x1=>0x76), 0),
        (
            registers!(a:0x2A,f:SET_C,pc:2),
            memory!(0x0=>0x17,0x1=>0x76),
            4
        )
    ];
    // ANCHOR 0x21 | LD HL, d16 | [- - - -] | 3 | 12
    test_case![
        Ox21 | (
            registers!(),
            memory!(0x0=>0x21,0x1=>0xff,0x2=>0x9f,0x3=>0x76),
            0
        ),
        (
            registers!(h:0x9F,l:0xFF,pc:4),
            memory!(0x0=>0x21,0x1=>0xff,0x2=>0x9f,0x3=>0x76),
            12
        )
    ];
    // ANCHOR 0x32 | LD (HL-), A | 1 | 8
    test_case![
        Ox32 | (
            registers!(a:0x0,h:0x1,l:0x1),
            memory!(0x0=>0x32,0x1=>0x76),
            0
        ),
        (
            registers!(a:0x0,h:0x1,l:0x0,pc:0x2),
            memory!(0x0=>0x32,0x1=>0x76),
            8
        )
    ];
    // ANCHOR 0x20 | JR NZ, r8 | 2 | 12/8
    test_case![
        Ox20_with_Z_set
            | (
                registers!(f: SET_Z),
                memory!(0x0=>0x20,0x1=>0x03,0x2=>0x76,0x3=>0xFF,0x4=>0xFF),
                0
            ),
        (
            registers!(f:SET_Z,pc:3),
            memory!(0x0=>0x20,0x1=>0x03,0x2=>0x76,0x3=>0xFF,0x4=>0xFF),
            8
        )
    ];
    test_case![
        Ox20_with_Z_not_set | 
        (
                registers!(),
                memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF,0x5=>0x76),
                0
        ),
        (
            registers!(pc:6),
            memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF,0x5=>0x76),
            12
        )
    ];
    // ANCHOR 0xCB 0x7C | BIT 7, H | [Z 0 1 -] | 2 | 8
    test_case![
        OxCB_Ox7C | 
        (
                registers!(h:0x80),
                memory!(0x0=>0xcb,0x1=>0x7c,0x2=>0x76),
                0
        ),
        (
            registers!(h:0x80,pc:3,f:SET_H),
            memory!(0x0=>0xcb,0x1=>0x7c,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x28 | JR Z, r8 | [- - - -] | 12/8
    test_case![
        Ox28_with_Z_set | 
        (
                registers!(f: SET_Z),
                memory!(0x0=>0x28,0x1=>0x02,0x2=>0x76,0x3=>0x76,0x4=>0x76),
                0
        ),
        (
            registers!(pc:5,f:SET_Z),
            memory!(0x0=>0x28,0x1=>0x02,0x2=>0x76,0x3=>0x76,0x4=>0x76),
            12
        )
    ];
    test_case![
        Ox28_with_Z_not_set
            | (
                registers!(),
                memory!(0x0=>0x28,0x1=>0x02,0x2=>0x76,0x3=>0x76,0x4=>0x76),
                0
            ),
        (
            registers!(pc:3),
            memory!(0x0=>0x28,0x1=>0x02,0x2=>0x76,0x3=>0x76,0x4=>0x76),
            8
        )
    ];
    // ANCHOR 0x18 | JR i8 | [- - - -] | 2 | 12
    test_case![
        Ox18_negative_jump
            | (
                registers!(a:0x00,pc:3),
                memory!(0x0=>0x76,0x1=>0x3C,0x2=>0x76,0x3=>0x18,0x4=>0xFC,0x5=>0x76,0x6=>0x76),
                0
            ),
        (
            registers!(a:0x01,pc:3),
            memory!(0x0=>0x76,0x1=>0x3C,0x2=>0x76,0x3=>0x18,0x4=>0xFC,0x5=>0x76,0x6=>0x76),
            4
        )
    ];
    test_case![
        Ox18_positive_jump
            | (
                registers!(a:0x00,pc:3),
                memory!(0x0=>0x76,0x1=>0x3C,0x2=>0x76,0x3=>0x18,0x4=>0x01,0x5=>0x76,0x6=>0x3c,0x7=>0x76),
                0
            ),
        (
            registers!(a:0x01,pc:8),
            memory!(0x0=>0x76,0x1=>0x3C,0x2=>0x76,0x3=>0x18,0x4=>0x01,0x5=>0x76,0x6=>0x3c,0x7=>0x76),
            4
        )
    ];
    // ANCHOR 0x3C | INC A | [Z 0 H -] | 1 | 4 
    test_case![
        Ox3C_with_overflow
            | (
                registers!(a:0xFF,f: SET_N | SET_C),
                memory!(0x0=>0x3c,0x1=>0x76),
                0
            ),
        (
            registers!(a:0x00,pc:2,f:SET_H | SET_Z | SET_C),
            memory!(0x0=>0x3c,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox3C_without_overflow
            | (
                registers!(a:0x0E,f:SET_C | SET_H | SET_N | SET_Z),
                memory!(0x0=>0x3C,0x1=>0x76),
                0
            ),
        (
            registers!(a:0x0F,f:SET_C,pc:2),
            memory!(0x0=>0x3C,0x1=>0x76),
            4
        )
    ];
    // SECTION 8 Bit Load Operations
    // ANCHOR 0x3E | LD A, d8 | [- - - -] | 2 | 8
    test_case![
        Ox3E | (registers!(), memory!(0x0=>0x3E,0x1=>0xFE,0x2=>0x76), 0),
        (
            registers!(a:0xFE,pc:3),
            memory!(0x0=>0x3E,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x06 | LD B, d8 | [- - - -] | 2 | 8
    test_case![
        Ox06 | 
        (
            registers!(), 
            memory!(0x0=>0x06,0x1=>0xFE,0x2=>0x76), 
            0
        ),
        (
            registers!(b:0xFE,pc:3),
            memory!(0x0=>0x06,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x0E | LD E, d8 | [- - - -] | 2 | 8
    test_case![
        Ox0E | (registers!(), memory!(0x0=>0x0E,0x1=>0xFE,0x2=>0x76), 0),
        (
            registers!(c:0xFE,pc:3),
            memory!(0x0=>0x0E,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x16 | LD D, d8 | [- - - -] | 2 | 8
    test_case![
        Ox16 | (registers!(), memory!(0x0=>0x16,0x1=>0xFE,0x2=>0x76), 0),
        (
            registers!(d:0xFE,pc:3),
            memory!(0x0=>0x16,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x1E | LD E, d8 | [- - - -] | 2 | 8
    test_case![
        Ox1E | (registers!(), memory!(0x0=>0x1E,0x1=>0xFE,0x2=>0x76), 0),
        (
            registers!(e:0xFE,pc:3),
            memory!(0x0=>0x1E,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x26 | LD (HL), d8 | [- - - -] | 2 | 8
    test_case![
        Ox26 | (registers!(), memory!(0x0=>0x26,0x1=>0xFE,0x2=>0x76), 0),
        (
            registers!(h:0xFE,pc:3),
            memory!(0x0=>0x26,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x2E | LD L, d8 | [- - - -] | 2 | 8
    test_case![
        Ox2E | (registers!(), memory!(0x0=>0x2E,0x1=>0xFE,0x2=>0x76), 0),
        (
            registers!(l:0xFE,pc:3),
            memory!(0x0=>0x2E,0x1=>0xFE,0x2=>0x76),
            8
        )
    ];
    // ANCHOR 0x36 | LD (HL), d8 | [- - - -] | 2 | 12
    test_case![
        Ox36 | (
            registers!(h:0x01,l:0x00),
            memory!(0x0=>0x36,0x1=>0xFE,0x2=>0x76),
            0
        ),
        (
            registers!(h:0x01,l:0x00,pc:3),
            memory!(0x0=>0x36,0x1=>0xFE,0x2=>0x76,0x100=>0xFE),
            12
        )
    ];
    // !SECTION
    // ANCHOR 0x87 | ADD A, A | [Z 0 H C] | 1 | 4
    test_case![
        Ox87_Zero | (registers!(a:0x0,f:SET_N), memory!(0x0=>0x87,0x1=>0x76), 0),
        (
            registers!(a:0x0,f:SET_Z,pc:2),
            memory!(0x0=>0x87,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox87_With_Half_Carry | (registers!(a:0x0f,f:SET_N), memory!(0x0=>0x87,0x1=>0x76), 0),
        (
            registers!(a:0x1e,f:SET_H,pc:2),
            memory!(0x0=>0x87,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox87_With_Carry | (registers!(a:0x80,f:SET_N), memory!(0x0=>0x87,0x1=>0x76), 0),
        (
            registers!(a:0x00,f:SET_Z|SET_C,pc:2),
            memory!(0x0=>0x87,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox87_With_Both_Carry | (registers!(a:0x88,f:SET_N), memory!(0x0=>0x87,0x1=>0x76), 0),
        (
            registers!(a:0x10,f:SET_H|SET_C,pc:2),
            memory!(0x0=>0x87,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox80_Zero
            | (
                registers!(a:0x0,b:0x0,f:SET_N),
                memory!(0x0=>0x80,0x1=>0x76),
                0
            ),
        (
            registers!(a:0x0,f:SET_Z,pc:2),
            memory!(0x0=>0x80,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox80_With_Half_Carry
            | (
                registers!(a:0x0f,b:0x01,f:SET_N),
                memory!(0x0=>0x80,0x1=>0x76),
                0
            ),
        (
            registers!(a:0x10,b:0x01,f:SET_H,pc:2),
            memory!(0x0=>0x80,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox80_With_Carry
            | (
                registers!(a:0xf0,b:0x10,f:SET_N),
                memory!(0x0=>0x80,0x1=>0x76),
                0
            ),
        (
            registers!(a:0x00,b:0x10,f:SET_Z|SET_C,pc:2),
            memory!(0x0=>0x80,0x1=>0x76),
            4
        )
    ];
    test_case![
        Ox80_With_Both_Carry
            | (
                registers!(a:0xff,b:0x01,f:SET_N),
                memory!(0x0=>0x80,0x1=>0x76),
                0
            ),
        (
            registers!(a:0x00,b:0x01,f:SET_H|SET_C|SET_Z,pc:2),
            memory!(0x0=>0x80,0x1=>0x76),
            4
        )
    ];
    // SECTION Stack Operations
    // 0xC5 | PUSH BC | [- - - -] | 1 | 16
    test_case![
        OxC5 |
        (
            registers!(sp: 0xfffe,b:0x01,c:0x01),
            memory!(0x0=>0xC5,0x1=>0x76),
            0
        ),
        (
            registers!(sp: 0xfffc,b:0x01,c:0x01,pc:0x02),
            memory!(0x0=>0xC5,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x01),
            16
        )
    ];
    // 0xD5 | PUSH DE | [- - - -] | 1 | 16
    test_case![
        OxD5 |
        (
            registers!(sp: 0xfffe,d:0x01,e:0x01),
            memory!(0x0=>0xD5,0x1=>0x76),
            0
        ),
        (
            registers!(sp: 0xfffc,d:0x01,e:0x01,pc:0x02),
            memory!(0x0=>0xD5,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x01),
            16
        )
    ];
    // 0xE5 | PUSH HL | [- - - -] | 1 | 16
    test_case![
        OxE5 |
        (
            registers!(sp: 0xfffe,h:0x01,l:0x01),
            memory!(0x0=>0xE5,0x1=>0x76),
            0
        ),
        (
            registers!(sp: 0xfffc,h:0x01,l:0x01,pc:0x02),
            memory!(0x0=>0xE5,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x01),
            16
        )
    ];
    // 0xF5 | PUSH AF | [- - - -] | 1 | 16
    test_case![
        OxF5 |
        (
            registers!(sp: 0xfffe,a:0x01,f:0x01),
            memory!(0x0=>0xF5,0x1=>0x76),
            0
        ),
        (
            registers!(sp: 0xfffc,a:0x01,f:0x01,pc:0x02),
            memory!(0x0=>0xF5,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x01),
            16
        )
    ];
    // 0xC1 | POP BC | [- - - -] | 1 | 12 
    test_case![
        OxC1 |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xC1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            0
        ),
        (
            registers!(sp:0xFFFE,b:0x02,c:0x01,pc:2),
            memory!(0x0=>0xC1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            12
        )
    ];
    // 0xD1 | POP DE | [- - - -] | 1 | 12 
    test_case![
        OxD1 |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xD1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            0
        ),
        (
            registers!(sp:0xFFFE,d:0x02,e:0x01,pc:2),
            memory!(0x0=>0xD1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            12
        )
    ];
    // 0xE1 | POP HL | [- - - -] | 1 | 12 
    test_case![
        OxE1 |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xE1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            0
        ),
        (
            registers!(sp:0xFFFE,h:0x02,l:0x01,pc:2),
            memory!(0x0=>0xE1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            12
        )
    ];
    // 0xF1 | POP AF | [- - - -] | 1 | 12 
    test_case![
        OxF1 |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xF1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            0
        ),
        (
            registers!(sp:0xFFFE,a:0x02,f:0x01,pc:2),
            memory!(0x0=>0xF1,0x1=>0x76,0xFFFC=>0x01,0xFFFD=>0x02),
            12
        )
    ];
    // !SECTION

    // SECTION CALL Instructions
    // ANCHOR 0xCD | CALL a16 | [- - - -] | 3 | 24
    test_case![
        OxCD | 
        (
            registers!(sp:0xFFFE),
            memory!(0x0=>0xCD,0x1=>0xFE,0x2=>0x7F,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x7FFF),
            memory!(0x0=>0xCD,0x1=>0xFE,0x2=>0x7F,0x7FFE=>0x76,0xFFFC=>0x03),
            24
        )
    ];
    // ANCHOR 0XC4 | CALL NZ, a16 | [- - - -] | 1 | 24/12
    test_case![
        OxC4_with_Z_reset |
        (
            registers!(sp:0xFFFE),
            memory!(0x0=>0xC4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x7FFF),
            memory!(0x0=>0xC4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76,0xFFFC=>0x03),
            24
        )
    ];
    test_case![
        OxC4_with_Z_set |
        (
            registers!(sp:0xFFFE,f: SET_Z),
            memory!(0x0=>0xC4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFE,f: SET_Z,pc:0x04),
            memory!(0x0=>0xC4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            12
        )
    ];
    // ANCHOR 0xCC | CALL Z, a16 | [- - - -] | 1 | 24/12
    test_case![
        OxCC_with_Z_set |
        (
            registers!(sp:0xFFFE,f:SET_Z),
            memory!(0x0=>0xCC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,f:SET_Z,pc:0x7FFF),
            memory!(0x0=>0xCC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76,0xFFFC=>0x03),
            24
        )
    ];
    test_case![
        OxCC_with_Z_reset |
        (
            registers!(sp:0xFFFE),
            memory!(0x0=>0xCC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x0=>0xCC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            12
        )
    ];
    // ANCHOR 0xD4 | CALL NC, a16 | [- - - -] | 1 | 24/12
    test_case![
        OxD4_with_C_reset |
        (
            registers!(sp:0xFFFE),
            memory!(0x0=>0xD4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x7FFF),
            memory!(0x0=>0xD4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76,0xFFFC=>0x03),
            24
        )
    ];
    test_case![
        OxD4_with_C_set |
        (
            registers!(sp:0xFFFE,f: SET_C),
            memory!(0x0=>0xD4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFE,f: SET_C,pc:0x04),
            memory!(0x0=>0xD4,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            12
        )
    ];
    // ANCHOR 0xDC | CALL C, a16 | [- - - -] | 1 | 24/12
    test_case![
        OxDC_with_C_set |
        (
            registers!(sp:0xFFFE,f:SET_C),
            memory!(0x0=>0xDC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,f:SET_C,pc:0x7FFF),
            memory!(0x0=>0xDC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76,0xFFFC=>0x03),
            24
        )
    ];
    test_case![
        OxDC_with_C_reset |
        (
            registers!(sp:0xFFFE),
            memory!(0x0=>0xDC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x0=>0xDC,0x1=>0xFE,0x2=>0x7F,0x3=>0x76,0x7FFE=>0x76),
            12
        )
    ];
    // !SECTION
    
    // SECTION Return Instructions
    // ANCHOR 0XC9 | RET | [- - - -] | 1 | 16
    test_case![
        OxC9 |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xC9,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFE,pc:0x8000),
            memory!(0x0=>0xC9,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            16
        )
    ];
    // ANCHOR 0xC0 | RET NZ | [- - - -] | 1 | 20/8
    test_case![
        OxC0_with_Z_reset |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xC0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFE,pc:0x8000),
            memory!(0x0=>0xC0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            20
        )
    ];
    test_case![
        OxC0_with_Z_set |
        (
            registers!(sp:0xFFFC,f:SET_Z),
            memory!(0x0=>0xC0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFC,f:SET_Z,pc:0x02),
            memory!(0x0=>0xC0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            8
        )
    ];
    // ANCHOR 0xC8 | RET Z | [- - - -] | 1 | 20/8
    test_case![
        OxC8_with_Z_set |
        (
            registers!(sp:0xFFFC,f:SET_Z),
            memory!(0x0=>0xC8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFE,f:SET_Z,pc:0x8000),
            memory!(0x0=>0xC8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            20
        )
    ];
    test_case![
        OxC8_with_Z_reset |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xC8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x02),
            memory!(0x0=>0xC8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            8
        )
    ];
    // ANCHOR 0xD0 | RET NC | [- - - -] | 1 | 20/8
    test_case![
        OxD0_with_C_reset |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xD0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFE,pc:0x8000),
            memory!(0x0=>0xD0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            20
        )
    ];
    test_case![
        OxC0_with_C_set |
        (
            registers!(sp:0xFFFC,f:SET_C),
            memory!(0x0=>0xD0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFC,f:SET_C,pc:0x02),
            memory!(0x0=>0xD0,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            8
        )
    ];
    // ANCHOR 0xD8 | RET C | [- - - -] | 1 | 20/8
    test_case![
        OxD8_with_C_set |
        (
            registers!(sp:0xFFFC,f:SET_C),
            memory!(0x0=>0xD8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFE,f:SET_C,pc:0x8000),
            memory!(0x0=>0xD8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            20
        )
    ];
    test_case![
        OxD8_with_Z_reset |
        (
            registers!(sp:0xFFFC),
            memory!(0x0=>0xD8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x02),
            memory!(0x0=>0xD8,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            8
        )
    ];
    // ANCHOR RETI | [- - - -] | 1 | 16
    #[test]
    fn OxD9(){
        let current_state = (            
            registers!(sp:0xFFFC),
            memory!(0x0=>0xD9,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            0,
            false
        );
        let expected_state = (            
            registers!(sp:0xFFFE,pc:0x8000),
            memory!(0x0=>0xD9,0x1=>0x76,0x7FFF=>0x76,0xFFFC=>0xFF,0xFFFD=>0x7F),
            16,
            true
        );

        let memory = Arc::new(Mutex::new(current_state.1));
        let memory_1 = Arc::clone(&memory);
        let mut cpu = CPU {
            registers: current_state.0,
            memory: memory,
            cycle: current_state.2,
            state: CPUState::Active,
            ime: false,
        };
        while cpu.state == CPUState::Active{
            cpu.execute();
        }

        let mem = *cpu.memory.lock().unwrap();
        assert_eq!((cpu.get_registers(), mem, cpu.get_cycles(), cpu.ime),expected_state);
    }
    // !SECTION

    // SECTION Reset Instructions
    // ANCHOR 0xC7 | RST 00H | [- - - -] | 1 | 16
    test_case![
        OxC7 |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x0=>0x76,0x04=>0xC7,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x01),
            memory!(0x0=>0x76,0x04=>0xC7,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xCF | RST 08H | [- - - -] | 1 | 16
    test_case![
        OxCF |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x08=>0x76,0x04=>0xCF,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x09),
            memory!(0x08=>0x76,0x04=>0xCF,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xD7 | RST 10H | [- - - -] | 1 | 16
    test_case![
        OxD7 |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x10=>0x76,0x04=>0xD7,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x11),
            memory!(0x10=>0x76,0x04=>0xD7,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xDF | RST 18H | [- - - -] | 1 | 16
    test_case![
        OxDF |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x18=>0x76,0x04=>0xDF,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x19),
            memory!(0x18=>0x76,0x04=>0xDF,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xE7 | RST 20H | [- - - -] | 1 | 16
    test_case![
        OxE7 |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x20=>0x76,0x04=>0xE7,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x21),
            memory!(0x20=>0x76,0x04=>0xE7,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xEF | RST 28H | [- - - -] | 1 | 16
    test_case![
        OxEF |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x28=>0x76,0x04=>0xEF,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x29),
            memory!(0x28=>0x76,0x04=>0xEF,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xF7 | RST 30H | [- - - -] | 1 | 16
    test_case![
        OxF7 |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x30=>0x76,0x04=>0xF7,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x31),
            memory!(0x30=>0x76,0x04=>0xF7,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // ANCHOR 0xFF | RST 38H | [- - - -] | 1 | 16 
    test_case![
        OxFF |
        (
            registers!(sp:0xFFFE,pc:0x04),
            memory!(0x38=>0x76,0x04=>0xFF,0x05=>0x76),
            0
        ),
        (
            registers!(sp:0xFFFC,pc:0x39),
            memory!(0x38=>0x76,0x04=>0xFF,0x05=>0x76,0xFFFC=>0x05),
            16
        )
    ];
    // !SECTION
}
// !SECTION