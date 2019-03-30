use std::io::prelude::*;
use std::io::{self, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt};
use rand::Rng;

#[macro_use]
mod opcode;
use opcode::Opcode;

pub struct Cpu {
    gpr: [u8; 0xf],
    program_counter: u16,
    index: u16,
    stack_pointer: u16,
    memory: io::Cursor<Vec<u8>>,
    rng: rand::rngs::ThreadRng
}

impl Cpu {
    const PROGRAM_BASE: u16 = 0x200;
    const STACK_BASE: u16 = 0xefe;
    pub fn new(rom: Vec<u8>) -> Cpu {
        let mut memory = vec![0; 0x1000];
        memory.splice(0x200 .. (0x200 + rom.len()), rom.iter().cloned());

        Cpu {
            gpr: [0; 0xf],
            program_counter: Cpu::PROGRAM_BASE,
            index: 0,
            stack_pointer: Cpu::STACK_BASE,
            memory: io::Cursor::new(memory),
            rng: rand::thread_rng()
        }
    }

    fn fetch_instruction(&mut self) -> Opcode {
        self.memory.seek(SeekFrom::Start(self.program_counter as u64)).unwrap();
        self.program_counter += 2;
        Opcode(self.memory.read_u16::<BigEndian>().unwrap())
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_instruction();

        match opcode.to_nibble_tuple() {
            opcode!("JMP addr")         => { self.program_counter = opcode.get_tribble(); },
            opcode!("JMP V0, addr")     => { self.program_counter = opcode.get_tribble() + self.gpr[0] as u16; },
            opcode!("MOV Vx, byte")     => { self.gpr[opcode.get_reg1() as usize] = opcode.get_byte(); },
            opcode!("MOV Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] = self.gpr[opcode.get_reg2() as usize]; },
            opcode!("ADD Vx, byte")     => { self.gpr[opcode.get_reg1() as usize] += opcode.get_byte(); },
            opcode!("ADD Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] += self.gpr[opcode.get_reg2() as usize]; },
            opcode!("ADD I, Vx")        => { self.index += self.gpr[opcode.get_reg1() as usize] as u16; },
            opcode!("MOV I, addr")      => { self.index = opcode.get_tribble(); },
            opcode!("SUB Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] -= self.gpr[opcode.get_reg2() as usize]; },
            opcode!("RSUB Vx, Vy")      => { self.gpr[opcode.get_reg1() as usize] = self.gpr[opcode.get_reg2() as usize] - self.gpr[opcode.get_reg1() as usize]; },
            opcode!("OR Vx, Vy")        => { self.gpr[opcode.get_reg1() as usize] |= self.gpr[opcode.get_reg2() as usize]; },
            opcode!("AND Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] &= self.gpr[opcode.get_reg2() as usize]; },
            opcode!("XOR Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] ^= self.gpr[opcode.get_reg2() as usize]; },
            // @TODO: update Vf
            opcode!("SHR Vx")           => { self.gpr[opcode.get_reg1() as usize] >>= 1; },
            opcode!("SHL Vx")           => { self.gpr[opcode.get_reg1() as usize] <<= 1; },
            opcode!("RND Vx, tribble")  => { self.gpr[opcode.get_reg1() as usize] = self.rng.gen_range(0, opcode.get_byte() + 1); },
            _                           => { println!("Unsupported opcode: 0x{:X}", opcode.0); std::process::exit(1); }
        }
    }
}