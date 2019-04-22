use super::user_interface::{DISPLAY_HEIGHT, DISPLAY_WIDTH, UI};
use bitvec::{BigEndian, BitVec, Bits};

type Sprite = Vec<BitVec>;

pub fn draw_sprite(ui: &mut UI, x: usize, y: usize, sprite_data: &[u8]) -> bool {
    let mut collision: bool = false;
    let sprite = generate_sprite(sprite_data);

    for line in y..y + sprite.len() {
        for column in x..x + 8 {
            let old_pixel_value = ui.read_pixel(line % DISPLAY_HEIGHT, column % DISPLAY_WIDTH);
            let pixel = sprite[line - y][column - x];

            ui.write_pixel(
                line % DISPLAY_HEIGHT,
                column % DISPLAY_WIDTH,
                old_pixel_value ^ pixel,
            );

            if old_pixel_value && pixel {
                collision = true;
            }
        }
    }

    collision
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
