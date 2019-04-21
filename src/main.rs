use std::env;
use std::io;
use std::thread;

pub mod cpu;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: chip8.exe <program_path>");
        std::process::exit(1);
    }

    let rom_contents = std::fs::read(&args[1])?;

    thread::spawn(move || {
        let mut cpu = cpu::Cpu::new(rom_contents);
        loop {
            cpu.execute()
        }
    });
    loop {}
    Ok(())
}
