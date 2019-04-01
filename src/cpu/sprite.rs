use bitvec::{BitVec, Bits};

// @TODO: refactor the whole Sprite struct, it is hideous

pub struct Sprite(Vec<BitVec>);

impl Sprite {
    pub fn new(sprite_data: &Vec<u8>) -> Sprite {
        Sprite(
            sprite_data.iter()
                       .map(|byte| (0..8).rev()
                                         .map(move |bit_index| byte.get::<bitvec::BigEndian>(bit_index.into()))
                                         .collect())
                       .collect()
        )
    }

    pub fn get_screen_mask(&self, x: usize, y: usize) -> [bool; 64 * 32] {
        let mut mask = [false; 64 * 32];
        for line in y..y+self.0.len() {
            for column in x..x+8 {
                mask[line * 32 + column % 64] = self.0[line - y][column - x];
            }
        }
        
        mask
    }
}