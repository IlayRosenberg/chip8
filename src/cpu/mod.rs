use std::io;
use byteorder::{BigEndian, ReadBytesExt};
use rand::Rng;
#[macro_use]
mod opcode;
use opcode::Opcode;

pub struct Cpu {
    gpr: [u8; 0xf],
    pc: u16,
    index: u16,
    rom: io::Cursor<Vec<u8>>,
    rng: rand::rngs::ThreadRng
}

impl Cpu {

    pub fn new(rom: Vec<u8>) -> Cpu {
        Cpu {
            gpr: [0; 0xf],
            pc: 0,
            index: 0,
            rom: io::Cursor::new(rom),
            rng: rand::thread_rng()
        }
    }

    pub fn execute(&mut self) {
        self.rom.set_position(self.pc.into());
        let opcode = Opcode(self.rom.read_u16::<BigEndian>().unwrap());

        match opcode.to_nibble_tuple() {
            opcode!("JMP addr")         => { self.pc = opcode.get_tribble() },
            opcode!("JMP V0, addr")     => { self.pc = opcode.get_tribble() + self.gpr[0] as u16 },
            opcode!("MOV Vx, byte")     => { self.gpr[opcode.get_reg1() as usize] = opcode.get_byte() },
            opcode!("MOV Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] = self.gpr[opcode.get_reg2() as usize] },
            opcode!("ADD Vx, byte")     => { self.gpr[opcode.get_reg1() as usize] += opcode.get_byte() },
            opcode!("ADD Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] += self.gpr[opcode.get_reg2() as usize] },
            opcode!("ADD I, Vx")        => { self.index += self.gpr[opcode.get_reg1() as usize] as u16 },
            opcode!("MOV I, addr")      => { self.index = opcode.get_tribble() },
            opcode!("SUB Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] -= self.gpr[opcode.get_reg2() as usize] },
            opcode!("RSUB Vx, Vy")      => { self.gpr[opcode.get_reg1() as usize] = self.gpr[opcode.get_reg2() as usize] - self.gpr[opcode.get_reg1() as usize] },
            opcode!("OR Vx, Vy")        => { self.gpr[opcode.get_reg1() as usize] |= self.gpr[opcode.get_reg2() as usize] },
            opcode!("AND Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] &= self.gpr[opcode.get_reg2() as usize] },
            opcode!("XOR Vx, Vy")       => { self.gpr[opcode.get_reg1() as usize] ^= self.gpr[opcode.get_reg2() as usize] },
            opcode!("SHR Vx")           => { self.gpr[opcode.get_reg1() as usize] >>= 1 },
            opcode!("SHL Vx")           => { self.gpr[opcode.get_reg1() as usize] <<= 1 },
            opcode!("RND Vx, tribble")  => { self.gpr[opcode.get_reg1() as usize] = self.rng.gen_range(0, opcode.get_byte() + 1) },
            _                           => { panic!("Unsupported opcode: 0x{:X}", opcode.0) }
        }
    }
}