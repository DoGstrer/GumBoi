/*For Module system : http://www.sheshbabu.com/posts/rust-module-system/
The syntax is 2015 specific but the core concept remains the same in 2018 as well.
TL;DR : We need to explicitely build the module tree in Rust, there's no implicit
mapping between file system to module tree*/

#![allow(unused)]

use std::fs;
use std::env;
use std::convert::TryInto;

use gumboi::GumBoi;

fn main(){
    let dmg_rom_file_loc: String = env::args().nth(1).unwrap();
    let catridge_rom_file_loc: String = env::args().nth(2).unwrap();

    let dmg_rom: Vec<u8> = read_bin(dmg_rom_file_loc);
    let catridge_rom: Vec<u8> = read_bin(catridge_rom_file_loc);

    let mut gumboi = GumBoi::new(dmg_rom[0..256].try_into().expect("DMG ROM exceeds 256 Bytes!"));
    gumboi.load(catridge_rom);
    gumboi.start();
    gumboi.exit();
    
}

fn read_bin(file_name: String) -> Vec<u8>{
    let binary = fs::read(file_name).unwrap();
    binary
}