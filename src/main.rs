mod cpu;
mod memory_bus;
use crate::cpu::CPU;
use std::time::{Duration, SystemTime};
use std::{thread,time};
use std::fs;

fn main(){
    let mut cycles:u64=0;
    let mut cpu = CPU::initialize();

    let rom_bytes = fs::read("TEST_ROM.bin")
    .expect("Error Reading File");
    println!("{:?}",rom_bytes);
    cpu.bulk_load_addr(rom_bytes.clone());

    let program_time=time::Instant::now();
    while cpu.get_pc() as usize!=rom_bytes.len(){
        cpu.execute();
        cycles+=cpu.get_cycle_count() as u64;
    }
    println!("Total Loop Time : {:?}",program_time.elapsed());

    cpu.print_registers();
    cpu.print_flags();

    println!("{}",cycles);
}