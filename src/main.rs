use std::env;
use std::io;

pub mod cpu;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <program_path>", &args[0]);
    }

    let rom_contents = std::fs::read(&args[1])?;
    let mut cpu = cpu::Cpu::new(rom_contents);
    while true {
        cpu.execute();
    }

    Ok(())
}
