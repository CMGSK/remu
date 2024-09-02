use std::process::id;


pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384]
}

impl GPU {

    pub fn read_vram(&self, addr:usize) -> u8 {
        self.vram[addr]
    }

    pub fn write_vram(&mut self, idx: usize, val: u8) {
        self.vram[idx] = val;
        if idx >= 0x1800 { return }

        let normal = idx & 0xFFFE;
        let byte1 = self.vram[normal];
        let byte2 = self.vram[normal + 1];

        let tiling = idx / 16;
        let row = (idx % 16) / 2;

        for pixel_idx in 0..8 {
            let mask = 1 << (7 - pixel_idx);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let val = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, false) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, true) => TilePixelValue::Zero,
            };

            self.tile_set[tiling][row][pixel_idx] = val
        }
    }
}
