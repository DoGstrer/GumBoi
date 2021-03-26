/*
Objs : FE00 0 FE9F
Each tile row : 2 bytes
Each tile : 16 bytes
Each sprite : 4 bytes [0:y-pos 1:x-pos 2:tile-number 3:sprite-flags]
sprite flag : [0-3 : Unused 4:Palette_Number 5:x-flip 6:y-flip 7:obj_to_bg priority]
Tile addressing methods : 8000 | 8800
PPU operates on pixel basis
LY : Register to store current scanline being processed
VRAM : BG Tiles + Sprite Tiles | Window & BG Map
Vertical timing (LCY)| Horizontal timing 
PPU owns Video RAM + OAM RAM
if CPU accesses OAM RAM when PPU isn in OAM Search or pixel transfer mode 0xff is returned
DMA Transfer : 0x3F7D (Super Mario ROM) code  | 0x21 7d 3f , 0x21 7d 3e (1ED starting address of loading DMA procedure to HRAM , here 0xffb6) 
CALL B6 FF at 0x6D FOR Super Mario
*/

use super::GumBoi;
use super::Memory;

use std::cell::RefCell;
use std::rc::Rc;

const LCDC:u16 = 0xFF40;

enum PPUModes{
    OAMSCAN,//OAM RAM --> Buffer 
    DRAWING,//Buffer --> LCD
    HBLANK,// PPU Does nothing | takes place at the end of every scanline
    VBLANK,//PPU Does nothing | Takes place at the end of every frame
}

pub struct PPU{
    buffer: [u8;16],
    mode: PPUModes,
    memory: Rc<RefCell<Memory>>,
}

impl PPU{
    pub fn new(memory: Rc<RefCell<Memory>>) -> PPU{
        PPU{
            buffer: [0u8;16],
            mode: PPUModes::OAMSCAN,
            memory: memory,
        }
    }
    fn start(){}
}

