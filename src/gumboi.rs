mod cpu;
mod memory_bus;
mod ppu;
mod sound;
mod memory;

use memory_bus::MemoryBus;
use cpu::CPU;

pub struct GumBoi<'a>{
    cpu: CPU<'a>,
}

impl<'a> GumBoi<'a>{
    fn run(){
        //TODO
    }
}

