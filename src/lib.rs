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

//TODO revamp unit tests
#[cfg(test)]
mod alu_intruction_tests {

    use std::cell::RefCell;
    use std::rc::Rc;

    use super::cpu::CPUState;
    use super::cpu::CPU;
    use super::memory::Memory;
    use super::ppu::PPU;
    use super::GumBoi;
    use super::GumBoiState;
    use super::Registers;
    use std::convert::TryInto;

    macro_rules! arr_u8 {
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

    fn get_next_state(current_state: (Registers, Memory, usize)) -> (Registers, Memory, usize) {
        let memory = Rc::new(RefCell::new(current_state.1));
        let memory_cpu = Rc::clone(&memory);
        let memory_ppu = Rc::clone(&memory);
        let memory_1 = Rc::clone(&memory);
        let mut gb = GumBoi {
            cpu: CPU {
                registers: current_state.0,
                memory: memory,
                cycle: current_state.2,
                state: CPUState::Active,
            },
            ppu: PPU::new(memory_ppu),
            memory: memory_cpu,
            cycle: 0,
            state: GumBoiState::Active,
        };
        gb.start();
        let mem = *memory_1.borrow();
        (gb.cpu.get_registers(), mem, gb.cpu.get_cycles())
    }

    test_case![
        OxFF | (registers!(), memory!(0x0=>0xff), 0),
        (registers!(), memory!(0x0=>0xff), 0)
    ];
    //RLC [Z 0 0 C]
    test_case![
        OxCB_Ox11
            | (
                registers!(c: 0x85),
                memory!(0x0=>0xcb,0x1=>0x11,0x2=>0xff),
                0
            ),
        (
            registers!(c: 0x0B,f:SET_C,pc:0x2),
            memory!(0x0=>0xcb,0x1=>0x11,0x2=>0xff),
            8
        )
    ];

    //RLA [0 0 0 C]
    test_case![
        Ox17 | (registers!(a:0x95), memory!(0x0=>0x17,0x1=>0xff), 0),
        (
            registers!(a:0x2A,f:SET_C,pc:1),
            memory!(0x0=>0x17,0x1=>0xff),
            4
        )
    ];

    test_case![
        Ox21 | (
            registers!(),
            memory!(0x0=>0x21,0x1=>0xff,0x2=>0x9f,0x3=>0xff),
            0
        ),
        (
            registers!(h:0x9F,l:0xFF,pc:3),
            memory!(0x0=>0x21,0x1=>0xff,0x2=>0x9f,0x3=>0xff),
            12
        )
    ];

    test_case![
        Ox32 | (
            registers!(a:0x0,h:0x1,l:0x1),
            memory!(0x0=>0x32,0x1=>0xff),
            0
        ),
        (
            registers!(a:0x0,h:0x1,l:0x0,pc:0x1),
            memory!(0x0=>0x32,0x1=>0xff),
            8
        )
    ];

    test_case![
        Ox20_with_z_set
            | (
                registers!(f: SET_Z),
                memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF),
                0
            ),
        (
            registers!(f:SET_Z,pc:2),
            memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF),
            8
        )
    ];

    test_case![
        Ox20_with_z_not_set
            | (
                registers!(),
                memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF,0x5=>0xFF),
                0
            ),
        (
            registers!(pc:5),
            memory!(0x0=>0x20,0x1=>0x03,0x2=>0xFF,0x3=>0xFF,0x4=>0xFF,0x5=>0xFF),
            12
        )
    ];

    test_case![
        OxCB_Ox7C
            | (
                registers!(h:0x80),
                memory!(0x0=>0xcb,0x1=>0x7c,0x2=>0xff),
                0
            ),
        (
            registers!(h:0x80,pc:2,f:SET_H),
            memory!(0x0=>0xcb,0x1=>0x7c,0x2=>0xff),
            8
        )
    ];

    //JR Z,r8
    test_case![
        Ox28_with_z_set
            | (
                registers!(f: SET_Z),
                memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff,0x4=>0xff),
                0
            ),
        (
            registers!(pc:4,f:SET_Z),
            memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff,0x4=>0xff),
            12
        )
    ];

    //JR Z,r8
    test_case![
        Ox28_with_z_not_set
            | (
                registers!(),
                memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff,0x4=>0xff),
                0
            ),
        (
            registers!(pc:2),
            memory!(0x0=>0x28,0x1=>0x02,0x2=>0xff,0x3=>0xff,0x4=>0xff),
            8
        )
    ];

    //JR i8 [- - - -]
    test_case![
        Ox18_negative_jump
            | (
                registers!(a:0x00,pc:3),
                memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0xFC,0x5=>0xff,0x6=>0xff),
                0
            ),
        (
            registers!(a:0x01,pc:2),
            memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0xFC,0x5=>0xff,0x6=>0xff),
            4
        )
    ];

    //JR i8 [- - - -]
    test_case![
        Ox18_positive_jump
            | (
                registers!(a:0x00,pc:3),
                memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0x01,0x5=>0xff,0x6=>0x3c,0x7=>0xff),
                0
            ),
        (
            registers!(a:0x01,pc:7),
            memory!(0x0=>0xFF,0x1=>0x3C,0x2=>0xFF,0x3=>0x18,0x4=>0x01,0x5=>0xff,0x6=>0x3c,0x7=>0xff),
            4
        )
    ];

    //INC A [- * 0 *]
    test_case![
        Ox3C_with_overflow
            | (
                registers!(a:0xFF,f: SET_N | SET_C | SET_N),
                memory!(0x0=>0x3c,0x1=>0xff),
                0
            ),
        (
            registers!(a:0x00,pc:1,f:SET_H | SET_Z | SET_C),
            memory!(0x0=>0x3c,0x1=>0xff),
            4
        )
    ];

    //INC A [- * 0 *]
    test_case![
        Ox3C_without_overflow
            | (
                registers!(a:0x0E,f:SET_C | SET_H | SET_N | SET_Z),
                memory!(0x0=>0x3C,0x1=>0xFF),
                0
            ),
        (
            registers!(a:0x0F,f:SET_C,pc:1),
            memory!(0x0=>0x3C,0x1=>0xFF),
            4
        )
    ];

    // LD INTRUCTIONS //
    //LD A d8
    test_case![
        Ox3E | (registers!(), memory!(0x0=>0x3E,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(a:0xFE,pc:2),
            memory!(0x0=>0x3E,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox06 | (registers!(), memory!(0x0=>0x06,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(b:0xFE,pc:2),
            memory!(0x0=>0x06,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox0E | (registers!(), memory!(0x0=>0x0E,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(c:0xFE,pc:2),
            memory!(0x0=>0x0E,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox16 | (registers!(), memory!(0x0=>0x16,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(d:0xFE,pc:2),
            memory!(0x0=>0x16,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox1E | (registers!(), memory!(0x0=>0x1E,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(e:0xFE,pc:2),
            memory!(0x0=>0x1E,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox26 | (registers!(), memory!(0x0=>0x26,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(h:0xFE,pc:2),
            memory!(0x0=>0x26,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox2E | (registers!(), memory!(0x0=>0x2E,0x1=>0xFE,0x2=>0xFF), 0),
        (
            registers!(l:0xFE,pc:2),
            memory!(0x0=>0x2E,0x1=>0xFE,0x2=>0xFF),
            8
        )
    ];

    test_case![
        Ox36 | (
            registers!(h:0x01,l:0x00),
            memory!(0x0=>0x36,0x1=>0xFE,0x2=>0xFF),
            0
        ),
        (
            registers!(h:0x01,l:0x00,pc:2),
            memory!(0x0=>0x36,0x1=>0xFE,0x2=>0xFF,0x100=>0xFE),
            12
        )
    ];
    test_case![
        Ox87_Zero | (registers!(a:0x0,f:SET_N), memory!(0x0=>0x87,0x1=>0xff), 0),
        (
            registers!(a:0x0,f:SET_Z,pc:1),
            memory!(0x0=>0x87,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox87_With_Half_Carry | (registers!(a:0x0f,f:SET_N), memory!(0x0=>0x87,0x1=>0xff), 0),
        (
            registers!(a:0x1e,f:SET_H,pc:1),
            memory!(0x0=>0x87,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox87_With_Carry | (registers!(a:0x80,f:SET_N), memory!(0x0=>0x87,0x1=>0xff), 0),
        (
            registers!(a:0x00,f:SET_Z|SET_C,pc:1),
            memory!(0x0=>0x87,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox87_With_Both_Carry | (registers!(a:0x88,f:SET_N), memory!(0x0=>0x87,0x1=>0xff), 0),
        (
            registers!(a:0x10,f:SET_H|SET_C,pc:1),
            memory!(0x0=>0x87,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox80_Zero
            | (
                registers!(a:0x0,b:0x0,f:SET_N),
                memory!(0x0=>0x80,0x1=>0xff),
                0
            ),
        (
            registers!(a:0x0,f:SET_Z,pc:1),
            memory!(0x0=>0x80,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox80_With_Half_Carry
            | (
                registers!(a:0x0f,b:0x01,f:SET_N),
                memory!(0x0=>0x80,0x1=>0xff),
                0
            ),
        (
            registers!(a:0x10,b:0x01,f:SET_H,pc:1),
            memory!(0x0=>0x80,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox80_With_Carry
            | (
                registers!(a:0xf0,b:0x10,f:SET_N),
                memory!(0x0=>0x80,0x1=>0xff),
                0
            ),
        (
            registers!(a:0x00,b:0x10,f:SET_Z|SET_C,pc:1),
            memory!(0x0=>0x80,0x1=>0xff),
            4
        )
    ];
    test_case![
        Ox80_With_Both_Carry
            | (
                registers!(a:0xff,b:0x01,f:SET_N),
                memory!(0x0=>0x80,0x1=>0xff),
                0
            ),
        (
            registers!(a:0x00,b:0x01,f:SET_H|SET_C|SET_Z,pc:1),
            memory!(0x0=>0x80,0x1=>0xff),
            4
        )
    ];
}
