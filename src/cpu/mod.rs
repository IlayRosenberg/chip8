use std::io::prelude::*;
use std::io::{self, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
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
    const WORD_SIZE: u16 = std::mem::size_of::<u16>() as u16;
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
        self.program_counter += Cpu::WORD_SIZE;
        Opcode(self.memory.read_u16::<BigEndian>().unwrap())
    }

    fn push(&mut self, value: u16) {
        self.memory.seek(SeekFrom::Start(self.stack_pointer as u64)).unwrap();
        self.memory.write_u16::<BigEndian>(value).unwrap();
        self.stack_pointer -= Cpu::WORD_SIZE;
    }

    fn pop(&mut self) -> u16 {
        self.memory.seek(SeekFrom::Start(self.stack_pointer as u64)).unwrap();
        self.stack_pointer += Cpu::WORD_SIZE;
        self.memory.read_u16::<BigEndian>().unwrap()
    }

    fn call(&mut self, addr: u16) {
        self.push(self.program_counter);
        self.program_counter = addr;
    }

    fn ret(&mut self) {
        self.program_counter = self.pop();
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_instruction();

        match opcode.to_nibble_tuple() {
            opcode!("JMP addr")         => { self.program_counter = opcode.get_tribble(); },
            opcode!("JMP V0, addr")     => { self.program_counter = opcode.get_tribble() + self.gpr[0] as u16; },
            opcode!("CALL addr")        => { self.call(opcode.get_tribble()); }
            opcode!("RET")              => { self.ret(); }
            opcode!("MOV Vx, byte")     => { self.gpr[opcode.get_reg1()] = opcode.get_byte(); },
            opcode!("MOV Vx, Vy")       => { self.gpr[opcode.get_reg1()] = self.gpr[opcode.get_reg2()]; },
            opcode!("ADD Vx, byte")     => { self.gpr[opcode.get_reg1()] += opcode.get_byte(); },
            opcode!("ADD Vx, Vy")       => { self.gpr[opcode.get_reg1()] += self.gpr[opcode.get_reg2()]; },
            opcode!("ADD I, Vx")        => { self.index += self.gpr[opcode.get_reg1()] as u16; },
            opcode!("MOV I, addr")      => { self.index = opcode.get_tribble(); },
            opcode!("SUB Vx, Vy")       => { self.gpr[opcode.get_reg1()] -= self.gpr[opcode.get_reg2()]; },
            opcode!("RSUB Vx, Vy")      => { self.gpr[opcode.get_reg1()] = self.gpr[opcode.get_reg2()] - self.gpr[opcode.get_reg1()]; },
            opcode!("OR Vx, Vy")        => { self.gpr[opcode.get_reg1()] |= self.gpr[opcode.get_reg2()]; },
            opcode!("AND Vx, Vy")       => { self.gpr[opcode.get_reg1()] &= self.gpr[opcode.get_reg2()]; },
            opcode!("XOR Vx, Vy")       => { self.gpr[opcode.get_reg1()] ^= self.gpr[opcode.get_reg2()]; },
            // @TODO: update Vf
            opcode!("SHR Vx")           => { self.gpr[opcode.get_reg1()] >>= 1; },
            opcode!("SHL Vx")           => { self.gpr[opcode.get_reg1()] <<= 1; },
            opcode!("RND Vx, tribble")  => { self.gpr[opcode.get_reg1()] = self.rng.gen_range(0, opcode.get_byte() + 1); },
            opcode!("SKE Vx, byte")     => {
                if self.gpr[opcode.get_reg1()] == opcode.get_byte() {
                    self.program_counter += Cpu::WORD_SIZE;
                }
            },
            opcode!("SKE Vx, Vy")       => {
                if self.gpr[opcode.get_reg1()] == self.gpr[opcode.get_reg2()] {
                    self.program_counter += Cpu::WORD_SIZE;
                }
            },
            opcode!("SKNE Vx, byte")    => {
                if self.gpr[opcode.get_reg1()] != opcode.get_byte() {
                    self.program_counter += Cpu::WORD_SIZE;
                }
            },
            opcode!("SKNE Vx, Vy")    => {
                if self.gpr[opcode.get_reg1()] != self.gpr[opcode.get_reg2()] {
                    self.program_counter += Cpu::WORD_SIZE;
                }
            },
            _                           => { println!("Unsupported opcode: 0x{:X}", opcode.0); std::process::exit(1); }
        }
    }
}