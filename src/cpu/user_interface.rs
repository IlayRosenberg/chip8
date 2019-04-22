use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;

pub trait UI {
    fn read_pixel(&self, x: usize, y: usize) -> bool;
    fn write_pixel(&mut self, x: usize, y: usize, value: bool);
    fn clear_display(&mut self);
    fn is_key_pressed(&self, key_code: usize) -> bool;
}

type Screen = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
type KeyPad = [bool; 16];

pub struct PistonUI {
    pub display: Arc<Mutex<Screen>>,
    pub keypad: Arc<Mutex<KeyPad>>,
}

impl PistonUI {
    pub fn get_display(&self) -> MutexGuard<Screen> {
        self.display.lock().unwrap()
    }
}

impl UI for PistonUI {
    fn read_pixel(&self, x: usize, y: usize) -> bool {
        self.display.lock().unwrap()[x][y]
    }

    fn write_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.display.lock().unwrap()[x][y] = value;
    }

    fn clear_display(&mut self) {
        *self.display.lock().unwrap() = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    fn is_key_pressed(&self, key_code: usize) -> bool {
        self.keypad.lock().unwrap()[key_code]
    }
}
