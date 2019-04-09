use bitvec::{BigEndian, BitVec, Bits};

const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

pub struct Display([[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);

impl Display {
    pub fn new() -> Display {
        Display([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT])
    }

    pub fn clear(&mut self) {
        self.0 = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn draw(&mut self, sprite: &Sprite, x: usize, y: usize) -> bool {
        let mut collision: bool = false;

        for line in y..y + sprite.0.len() {
            for column in x..x + 8 {
                let old_pixel_value = self.0[line][column % 64];
                let pixel = sprite.0[line - y][column - x];

                self.0[line][column % 64] ^= pixel;

                if old_pixel_value && pixel {
                    collision = true;
                }
            }
        }

        collision
    }
}

pub struct Sprite(pub Vec<BitVec>);

impl Sprite {
    pub fn new(sprite_data: &[u8]) -> Sprite {
        Sprite(
            sprite_data
                .iter()
                .map(|byte| {
                    (0..8)
                        .map(move |bit_index| byte.get::<BigEndian>(bit_index.into()))
                        .collect()
                })
                .collect(),
        )
    }
}
