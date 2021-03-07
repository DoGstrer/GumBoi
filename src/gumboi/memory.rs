/*https://www.reddit.com/r/rust/comments/8k4vwc/rust_noob_using_a_value_from_an_array_as_an_index/
Arrays cannot be indexed with u16/u8 etc types which are dependednt on underlying architecture. usize is a good fit
 RUST DEF : fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
*/

pub struct Memory{
    bank: [u8;65536],
}

impl Memory{
    pub fn new() -> Memory{
        Memory{
            bank:[0u8;65536]
        }
    }
    pub fn get_addr(&self,addr:u16) -> u8{
        return self.bank[addr as usize];
    }
    pub fn set_addr(&mut self,addr:u16,val:u8){
        self.bank[addr as usize]=val;
    }
}
