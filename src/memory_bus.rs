struct Memory{
    bank: [u8;65536],
}

impl Memory{
    fn initialize() -> Memory{
        Memory{
            bank:[0u8;65536]
        }
    }
    fn get_addr(&self,addr:u16) -> u8{
        return self.bank[addr as usize];
    }
    fn set_addr(&mut self,addr:u16,val:u8){
        self.bank[addr as usize]=val;
    }
}

pub struct MemoryBus{
    memory:Memory,
}

impl MemoryBus{
    pub fn initialize() -> MemoryBus{
        MemoryBus{
            memory:Memory::initialize(),
        }
    }
    pub fn get_addr(&self,addr:u16) -> u8{
        self.memory.get_addr(addr)
    }
    pub fn set_addr(&mut self,addr:u16,val:u8){
        self.memory.set_addr(addr,val);
    }
}