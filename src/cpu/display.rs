use bitvec::{BigEndian, BitVec, Bits};

const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

type Sprite = Vec<BitVec>;

pub struct Display([[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);

impl Display {
    pub fn new() -> Display {
        Display([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT])
    }

    pub fn clear(&mut self) {
        self.0 = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite_data: &[u8]) -> bool {
        let mut collision: bool = false;
        let sprite = generate_sprite(sprite_data);

        for line in y..y + sprite.len() {
            for column in x..x + 8 {
                let old_pixel_value = self.0[line][column % 64];
                let pixel = sprite[line - y][column - x];

                self.0[line][column % 64] ^= pixel;

                if old_pixel_value && pixel {
                    collision = true;
                }
            }
        }

        collision
    }
}

fn generate_sprite(sprite_data: &[u8]) -> Sprite {
    sprite_data
        .iter()
        .map(|byte| {
            (0..8)
                .map(move |bit_index| byte.get::<BigEndian>(bit_index.into()))
                .collect()
        })
        .collect()
}
