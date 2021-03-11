#![allow(unused)]

const SET_Z:u8=0b10000000;
const SET_N:u8=0b01000000;
const SET_H:u8=0b00100000;
const SET_C:u8=0b00010000;

pub trait Flag{
    fn reset(&mut self);

    fn set_z(&mut self);
    fn set_n(&mut self);
    fn set_h(&mut self);
    fn set_c(&mut self);

    fn reset_z(&mut self);
    fn reset_n(&mut self);
    fn reset_h(&mut self);
    fn reset_c(&mut self);

    fn get_z(&self) -> u8;
    fn get_n(&self) -> u8;
    fn get_h(&self) -> u8;
    fn get_c(&self) -> u8;

    fn is_set_z(&self) -> bool;
    fn is_set_n(&self) -> bool;
    fn is_set_h(&self) -> bool;
    fn is_set_c(&self) -> bool;
}

#[derive(Debug,PartialEq,Default)]
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
            a:0x0,b:0x0,c:0x0,d:0x0,e:0x0,f:0x0,h:0x0,l:0x0,sp:0x0,pc:0x0,
        }
    }

    //16 bit register combination operations
    pub fn get_hl(&self) -> u16{ ((self.h as u16 )<< 8|self.l as u16).into() }
    pub fn get_bc(&self) -> u16{ ((self.b as u16 )<< 8|self.c as u16).into() }
    pub fn get_de(&self) -> u16{ ((self.d as u16 )<< 8|self.e as u16).into() }
    pub fn get_af(&self) -> u16{ ((self.a as u16 )<< 8|self.f as u16).into() }

    pub fn set_hl(&mut self,value:u16){ self.h=((value&(0x00ff<<8))>>8) as u8; self.l=(value&0x00ff) as u8; }
    pub fn set_bc(&mut self,value:u16){ self.b=((value&(0x00ff<<8))>>8) as u8; self.c=(value&0x00ff) as u8; }
    pub fn set_de(&mut self,value:u16){ self.d=((value&(0x00ff<<8))>>8) as u8; self.e=(value&0x00ff) as u8; }
    pub fn set_af(&mut self,value:u16){ self.a=((value&(0x00ff<<8))>>8) as u8; self.f=(value&0x00ff) as u8; }
}


impl Flag for Registers{

    fn reset(&mut self){ self.f = 0x0; }

    fn set_z(&mut self){ self.f|=SET_Z; }
    fn set_n(&mut self){ self.f|=SET_N; }
    fn set_h(&mut self){ self.f|=SET_H; }
    fn set_c(&mut self){ self.f|=SET_C; }

    fn reset_z(&mut self){ self.f&=!SET_Z; }
    fn reset_n(&mut self){ self.f&=!SET_N; }
    fn reset_h(&mut self){ self.f&=!SET_H; }
    fn reset_c(&mut self){ self.f&=!SET_C; }

    fn get_z(&self) -> u8 { self.f&SET_Z }
    fn get_n(&self) -> u8 { self.f&SET_N }
    fn get_h(&self) -> u8 { self.f&SET_H }
    fn get_c(&self) -> u8 { self.f&SET_C }

    fn is_set_z(&self) -> bool { self.f&SET_Z==SET_Z }
    fn is_set_n(&self) -> bool { self.f&SET_N==SET_N }
    fn is_set_h(&self) -> bool { self.f&SET_H==SET_H }
    fn is_set_c(&self) -> bool { self.f&SET_C==SET_C }
}

