const SET_Z:u8=0b10000000;
const SET_N:u8=0b01000000;
const SET_H:u8=0b00100000;
const SET_C:u8=0b00010000;

#[derive(Debug)]
pub struct Registers{
    pub a:u8,
    pub b:u8,
    pub c:u8,
    pub d:u8,
    pub e:u8,
    pub f:u8,
    pub h:u8,
    pub l:u8,
    pub sp:u16,
    pub pc:u16,
}

impl Registers{
    pub fn new() -> Registers{
        Registers{
            a:0x0,b:0x0,c:0x0,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x0
        }
    }

    //16 bit register combination operations
    pub fn get_hl(&self) -> u16{ ((self.h as u16 )<< 8|self.l as u16).into() }
    pub fn set_hl(&mut self,value:u16){ self.h=((value&(0x00ff<<8))>>8) as u8; self.l=(value&0x00ff) as u8; }
    pub fn get_bc(&self) -> u16{ ((self.b as u16 )<< 8|self.c as u16).into() }
    pub fn set_bc(&mut self,value:u16){ self.b=((value&(0x00ff<<8))>>8) as u8; self.c=(value&0x00ff) as u8; }
    pub fn get_de(&self) -> u16{ ((self.d as u16 )<< 8|self.e as u16).into() }
    pub fn set_de(&mut self,value:u16){ self.d=((value&(0x00ff<<8))>>8) as u8; self.e=(value&0x00ff) as u8; }
    pub fn get_af(&self) -> u16{ ((self.a as u16 )<< 8|self.f as u16).into() }
    pub fn set_af(&mut self,value:u16){ self.a=((value&(0x00ff<<8))>>8) as u8; self.f=(value&0x00ff) as u8; }
    
    //flag operations
    pub fn set_z(&mut self){ self.f|=SET_Z; }
    pub fn reset_z(&mut self){ self.f&=!SET_Z; }
    pub fn get_z(&self) -> bool { self.f&SET_Z==SET_Z }
    pub fn set_n(&mut self){ self.f|=SET_N; }
    pub fn reset_n(&mut self){ self.f&=!SET_N; }
    pub fn get_n(&self) -> bool { self.f&SET_N==SET_N }
    pub fn set_h(&mut self){ self.f|=SET_H; }
    pub fn reset_h(&mut self){ self.f&=!SET_H; }
    pub fn get_h(&self) -> bool { self.f&SET_H==SET_H }
    pub fn set_c(&mut self){ self.f|=SET_C; }
    pub fn reset_c(&mut self){ self.f&=!SET_C; }
    pub fn get_c(&self) -> bool { self.f&SET_C==SET_C }

}