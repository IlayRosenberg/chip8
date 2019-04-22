use piston_window::*;
use std::collections::HashMap;
use std::env;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

mod cpu;
use cpu::user_interface::{PistonUI, DISPLAY_HEIGHT, DISPLAY_WIDTH, UI};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: chip8.exe <program_path>");
        std::process::exit(1);
    }

    let rom_contents = std::fs::read(&args[1])?;

    let mut keypad_map = HashMap::new();
    keypad_map.insert(Button::Keyboard(Key::D1), 1);
    keypad_map.insert(Button::Keyboard(Key::D2), 2);
    keypad_map.insert(Button::Keyboard(Key::D3), 3);
    keypad_map.insert(Button::Keyboard(Key::D4), 0xC);
    keypad_map.insert(Button::Keyboard(Key::Q), 4);
    keypad_map.insert(Button::Keyboard(Key::W), 5);
    keypad_map.insert(Button::Keyboard(Key::E), 6);
    keypad_map.insert(Button::Keyboard(Key::R), 0xD);
    keypad_map.insert(Button::Keyboard(Key::A), 7);
    keypad_map.insert(Button::Keyboard(Key::S), 8);
    keypad_map.insert(Button::Keyboard(Key::D), 9);
    keypad_map.insert(Button::Keyboard(Key::F), 0xE);
    keypad_map.insert(Button::Keyboard(Key::Z), 0xA);
    keypad_map.insert(Button::Keyboard(Key::X), 0);
    keypad_map.insert(Button::Keyboard(Key::C), 0xB);
    keypad_map.insert(Button::Keyboard(Key::V), 0xF);

    let display = Arc::new(Mutex::new([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]));
    let cpu_thread_display = Arc::clone(&display);
    let keypad = Arc::new(Mutex::new([false; 16]));
    let cpu_thread_keypad = Arc::clone(&keypad);
    let mut ui = PistonUI {
        display: display,
        keypad: keypad,
    };
    let cpu_thread_ui = PistonUI {
        display: cpu_thread_display,
        keypad: cpu_thread_keypad,
    };

    thread::spawn(move || {
        let mut cpu = cpu::Cpu::new(rom_contents, cpu_thread_ui);
        loop {
            cpu.execute();
        }
    });

    let mut window: PistonWindow = WindowSettings::new(
        "CHIP-8 Interpreter",
        [DISPLAY_WIDTH as u32 * 10, DISPLAY_HEIGHT as u32 * 10],
    )
    .build()
    .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for (j, line) in ui.get_display().iter().enumerate() {
                for (i, pixel) in line.iter().enumerate() {
                    if *pixel {
                        rectangle(
                            [1.0, 1.0, 1.0, 1.0],           // red
                            [i as f64, j as f64, 1.0, 1.0], // rectangle
                            c.zoom(10.0).transform,
                            g,
                        );
                    }
                }
            }
        });

        if let Some(button) = e.press_args() {
            if let Some(key_code) = keypad_map.get(&button) {
                ui.set_key_pressed(*key_code, true);
            }
        }

        if let Some(button) = e.release_args() {
            if let Some(key_code) = keypad_map.get(&button) {
                ui.set_key_pressed(*key_code, false);
            }
        }
    }

    Ok(())
}
