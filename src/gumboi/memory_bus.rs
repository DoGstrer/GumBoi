use super::memory::Memory;

pub struct MemoryBus{
    memory:Memory,
}

impl MemoryBus{
    pub fn new() -> MemoryBus{
        MemoryBus{
            memory:Memory::new(),
        }
    }
    pub fn get_addr(&self,addr:u16) -> u8{
        self.memory.get_addr(addr)
    }
    pub fn set_addr(&mut self,addr:u16,val:u8){
        self.memory.set_addr(addr,val);
    }
}

