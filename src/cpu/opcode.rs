#[macro_export]
macro_rules! opcode {
    ("CLS")                 =>    ((0x0, 0x0, 0xE, 0));
    ("RET")                 =>    ((0x0, 0x0, 0xE, 0xE));
    ("JMP addr")            =>    ((0x1, _, _, _));
    ("CALL addr")           =>    ((0x2, _, _, _));
    ("SKE Vx, byte")        =>    ((0x3, _, _, _));
    ("SKNE Vx, byte")       =>    ((0x4, _, _, _));
    ("SKE Vx, Vy")          =>    ((0x5, _, _, 0x0));
    ("MOV Vx, byte")        =>    ((0x6, _, _, _));
    ("ADD Vx, byte")        =>    ((0x7, _, _, _));
    ("MOV Vx, Vy")          =>    ((0x8, _, _, 0x0));
    ("OR Vx, Vy")           =>    ((0x8, _, _, 0x1));
    ("AND Vx, Vy")          =>    ((0x8, _, _, 0x2));
    ("XOR Vx, Vy")          =>    ((0x8, _, _, 0x3));
    ("ADD Vx, Vy")          =>    ((0x8, _, _, 0x4));
    ("SUB Vx, Vy")          =>    ((0x8, _, _, 0x5));
    ("SHR Vx")              =>    ((0x8, _, _, 0x6));
    ("RSUB Vx, Vy")         =>    ((0x8, _, _, 0x7));
    ("SHL Vx")              =>    ((0x8, _, _, 0xE));
    ("SKNE Vx, Vy")         =>    ((0x9, _, _, 0x0));
    ("MOV I, addr")         =>    ((0xA, _, _, _));
    ("JMP V0, addr")        =>    ((0xB, _, _, _));
    ("RND Vx, tribble")     =>    ((0xC, _, _, _));
    ("DRW Vx, Vy, nibble")  =>    ((0xD, _, _, _));
    ("SKP Vx")              =>    ((0xE, _, 0x9, 0xE));
    ("SKNP Vx")             =>    ((0xE, _, 0xA, 0x1));
    ("MOV Vx, DT")          =>    ((0xF, _, 0x0, 0x7));
    ("MOV Vx, K")           =>    ((0xF, _, 0x0, 0xA));
    ("MOV DT, Vx")          =>    ((0xF, _, 0x1, 0x5));
    ("MOV ST, Vx")          =>    ((0xF, _, 0x1, 0x8));
    ("ADD I, Vx")           =>    ((0xF, _, 0x1, 0xE));
    ("FONT Vx")             =>    ((0xF, _, 0x2, 0x9));
    ("BCD Vx")              =>    ((0xF, _, 0x3, 0x3));
    ("STR [I], Vx")         =>    ((0xF, _, 0x5, 0x5));
    ("LD Vx, [I]")          =>    ((0xF, _, 0x6, 0x5));
}

fn bit_slice(number: u16, offset: u8, size: u8) -> u16 {
    (number >> offset) & (2u16.pow(size.into()) - 1)
}

fn get_nibble(number: u16, index: u8) -> u8 {
    bit_slice(number, index * 4, 4) as u8
}

pub struct Opcode(pub u16);
impl Opcode {
    pub fn reg1(&self) -> usize {
        get_nibble(self.0, 2) as usize
    }

    pub fn reg2(&self) -> usize {
        get_nibble(self.0, 1) as usize
    }

    pub fn byte(&self) -> u8 {
        bit_slice(self.0, 0, 8) as u8
    }

    pub fn nibble(&self) -> u8 {
        get_nibble(self.0, 0) as u8
    }

    pub fn tribble(&self) -> u16 {
        bit_slice(self.0, 0, 12)
    }

    pub fn to_nibble_tuple(&self) -> (u8, u8, u8, u8) {
        (
            get_nibble(self.0, 3),
            get_nibble(self.0, 2),
            get_nibble(self.0, 1),
            get_nibble(self.0, 0),
        )
    }
}
