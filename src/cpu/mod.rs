use std::io::prelude::*;
use std::io::{self, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rand::Rng;

#[macro_use]
mod opcode;
use opcode::Opcode;

mod sprite;
use sprite::Sprite;

pub struct Cpu {
    gpr: [u8; 16],
    program_counter: u16,
    index: u16,
    stack_pointer: u16,
    memory: io::Cursor<Vec<u8>>,
    display: [bool; 64 * 32],
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
            gpr: [0; 16],
            program_counter: Cpu::PROGRAM_BASE,
            index: 0,
            stack_pointer: Cpu::STACK_BASE,
            memory: io::Cursor::new(memory),
            display: [false; 64 * 32],
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

    fn draw(&mut self, x: u8, y: u8, z: u8) {
        assert!(z <= 15, "Sprite size is {} when the maximum allowed sprite size is 15", z);

        let mut sprite_data = vec![0u8; z as usize];
        self.memory.seek(SeekFrom::Start(self.index as u64)).unwrap();
        self.memory.read_exact(sprite_data.as_mut_slice()).unwrap();

        let sprite = Sprite::new(&sprite_data);
        let screen_mask = sprite.get_screen_mask(x as usize, y as usize);

        for (pixel, mask) in self.display.iter_mut().zip(screen_mask.iter()) {
            *pixel ^= *mask;
            if !*pixel {
                self.gpr[0xf] |= *mask as u8; // collision detection
            }
        }
    }

    fn skip_if(&mut self, predicate: bool) {
        if predicate {
            self.program_counter += Cpu::WORD_SIZE;
        }
    }

    fn bcd(&mut self, number: u8) {
        self.memory.seek(SeekFrom::Start(self.index as u64)).unwrap();
        let bcd = [((number / 100) % 10) as u8, ((number / 10) % 10) as u8, (number % 10) as u8];
        self.memory.write_all(&bcd).unwrap();
    }
    pub fn execute(&mut self) {
        let opcode = self.fetch_instruction();

        match opcode.to_nibble_tuple() {
            opcode!("JMP addr")             => { self.program_counter = opcode.tribble(); },
            opcode!("JMP V0, addr")         => { self.program_counter = opcode.tribble() + self.gpr[0] as u16; },
            opcode!("CALL addr")            => { self.call(opcode.tribble()); }
            opcode!("RET")                  => { self.ret(); }
            opcode!("MOV Vx, byte")         => { self.gpr[opcode.reg1()] = opcode.byte(); },
            opcode!("MOV Vx, Vy")           => { self.gpr[opcode.reg1()] = self.gpr[opcode.reg2()]; },
            opcode!("ADD Vx, byte")         => { self.gpr[opcode.reg1()] += opcode.byte(); },
            opcode!("ADD Vx, Vy")           => { self.gpr[opcode.reg1()] += self.gpr[opcode.reg2()]; },
            opcode!("ADD I, Vx")            => { self.index += self.gpr[opcode.reg1()] as u16; },
            opcode!("MOV I, addr")          => { self.index = opcode.tribble(); },
            opcode!("SUB Vx, Vy")           => { self.gpr[opcode.reg1()] -= self.gpr[opcode.reg2()]; },
            opcode!("RSUB Vx, Vy")          => { self.gpr[opcode.reg1()] = self.gpr[opcode.reg2()] - self.gpr[opcode.reg1()]; },
            opcode!("OR Vx, Vy")            => { self.gpr[opcode.reg1()] |= self.gpr[opcode.reg2()]; },
            opcode!("AND Vx, Vy")           => { self.gpr[opcode.reg1()] &= self.gpr[opcode.reg2()]; },
            opcode!("XOR Vx, Vy")           => { self.gpr[opcode.reg1()] ^= self.gpr[opcode.reg2()]; },
            // @TODO: update Vf
            opcode!("SHR Vx")               => { self.gpr[opcode.reg1()] >>= 1; },
            opcode!("SHL Vx")               => { self.gpr[opcode.reg1()] <<= 1; },
            opcode!("RND Vx, tribble")      => { self.gpr[opcode.reg1()] = self.rng.gen_range(0, opcode.byte() + 1); },
            opcode!("SKE Vx, byte")         => { self.skip_if(self.gpr[opcode.reg1()] == opcode.byte()) },
            opcode!("SKE Vx, Vy")           => { self.skip_if(self.gpr[opcode.reg1()] == self.gpr[opcode.reg2()]) },
            opcode!("SKNE Vx, byte")        => { self.skip_if(self.gpr[opcode.reg1()] != opcode.byte()) },
            opcode!("SKNE Vx, Vy")          => { self.skip_if(self.gpr[opcode.reg1()] != self.gpr[opcode.reg2()]) },
            opcode!("DRW Vx, Vy, nibble")   => { self.draw(self.gpr[opcode.reg1()], self.gpr[opcode.reg2()], opcode.nibble()); },
            opcode!("BCD Vx")               => { self.bcd(self.gpr[opcode.reg1()]); },
        }
    }
}