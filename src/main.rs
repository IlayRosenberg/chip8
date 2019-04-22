use piston_window::*;
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

    let display = Arc::new(Mutex::new([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]));
    let cpu_thread_display = Arc::clone(&display);
    let keypad = Arc::new(Mutex::new([false; 16]));
    let cpu_thread_keypad = Arc::clone(&keypad);
    let ui = PistonUI {
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
    }

    Ok(())
}
