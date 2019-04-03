use rand::Rng;
use bitvec::Bits;

#[macro_use]
mod opcode;
use opcode::Opcode;

mod memory;

mod sprite;
use sprite::Sprite;

mod timer;

pub struct Cpu {
    gpr: [u8; 16],
    program_counter: u16,
    index: u16,
    stack_pointer: u16,
    memory: memory::Memory,
    display: [bool; 64 * 32],
    rng: rand::rngs::ThreadRng,
    delay_timer: timer::Timer
}

impl Cpu {
    const PROGRAM_BASE: u16 = 0x200;
    const STACK_BASE: u16 = 0xefe;
    const WORD_SIZE: u16 = std::mem::size_of::<u16>() as u16;
    pub fn new(rom: Vec<u8>) -> Cpu {
        Cpu {
            gpr: [0; 16],
            program_counter: Cpu::PROGRAM_BASE,
            index: 0,
            stack_pointer: Cpu::STACK_BASE,
            memory: memory::Memory::new(&rom),
            display: [false; 64 * 32],
            rng: rand::thread_rng(),
            delay_timer: timer::Timer::new()
        }
    }

    fn fetch_instruction(&mut self) -> Opcode {
        let instruction = Opcode(self.memory.read_u16_at(self.program_counter as usize));
        self.program_counter += Cpu::WORD_SIZE;
        instruction
    }

    fn push(&mut self, value: u16) {
        self.stack_pointer -= Cpu::WORD_SIZE;
        self.memory.write_u16_at(value, self.stack_pointer as usize);
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer += Cpu::WORD_SIZE;
        self.memory.read_u16_at(self.stack_pointer as usize)
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
        self.memory.read_at(sprite_data.as_mut_slice(), self.index as usize);

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
        let bcd = [((number / 100) % 10) as u8, ((number / 10) % 10) as u8, (number % 10) as u8];
        self.memory.write_at(&bcd, self.index as usize);
    }

    fn load_regs(&mut self, reg_count: usize) {
        self.memory.read_at(&mut self.gpr[0 .. reg_count+1], self.index as usize);
    }

    fn store_regs(&mut self, reg_count: usize) {
        self.memory.write_at(&self.gpr[0 .. reg_count+1], self.index as usize);
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
            opcode!("MOV I, addr")          => { self.index = opcode.tribble(); },
            opcode!("MOV DT, Vx")           => { self.delay_timer.set(self.gpr[opcode.reg1()] as u64); },
            opcode!("MOV Vx, DT")           => { self.gpr[opcode.reg1()] = self.delay_timer.get() as u8; },
            opcode!("ADD Vx, byte")         => { self.gpr[opcode.reg1()].wrapping_add(opcode.byte()); },
            opcode!("ADD Vx, Vy")           => { self.gpr[opcode.reg1()].wrapping_add(self.gpr[opcode.reg2()]); },
            opcode!("ADD I, Vx")            => { self.index.wrapping_add(self.gpr[opcode.reg1()] as u16); },
            opcode!("SUB Vx, Vy")           => { self.gpr[opcode.reg1()].wrapping_sub(self.gpr[opcode.reg2()]); },
            opcode!("RSUB Vx, Vy")          => { self.gpr[opcode.reg1()] = self.gpr[opcode.reg2()].wrapping_sub(self.gpr[opcode.reg1()]); },
            opcode!("OR Vx, Vy")            => { self.gpr[opcode.reg1()] |= self.gpr[opcode.reg2()]; },
            opcode!("AND Vx, Vy")           => { self.gpr[opcode.reg1()] &= self.gpr[opcode.reg2()]; },
            opcode!("XOR Vx, Vy")           => { self.gpr[opcode.reg1()] ^= self.gpr[opcode.reg2()]; },
            opcode!("SHR Vx")               => { 
                self.gpr[opcode.reg1()].get::<bitvec::BigEndian>(0.into());
                self.gpr[opcode.reg1()] >>= 1;
                },
            opcode!("SHL Vx")               => {
                self.gpr[opcode.reg1()].get::<bitvec::BigEndian>(7.into());
                self.gpr[opcode.reg1()] <<= 1; },
            opcode!("RND Vx, tribble")      => { self.gpr[opcode.reg1()] = self.rng.gen_range(0, opcode.byte() + 1); },
            opcode!("SKE Vx, byte")         => { self.skip_if(self.gpr[opcode.reg1()] == opcode.byte()) },
            opcode!("SKE Vx, Vy")           => { self.skip_if(self.gpr[opcode.reg1()] == self.gpr[opcode.reg2()]) },
            opcode!("SKNE Vx, byte")        => { self.skip_if(self.gpr[opcode.reg1()] != opcode.byte()) },
            opcode!("SKNE Vx, Vy")          => { self.skip_if(self.gpr[opcode.reg1()] != self.gpr[opcode.reg2()]) },
            opcode!("DRW Vx, Vy, nibble")   => { self.draw(self.gpr[opcode.reg1()], self.gpr[opcode.reg2()], opcode.nibble()); },
            opcode!("BCD Vx")               => { self.bcd(self.gpr[opcode.reg1()]); },
            opcode!("LD Vx, [I]")           => { self.load_regs(opcode.reg1()) },
            opcode!("STR [I], Vx")          => { self.store_regs(opcode.reg1()) },
            opcode!("FONT Vx")              => { self.index = (self.gpr[opcode.reg1()] * 5) as u16; }
            _                               => { println!("Unsupported opcode: 0x{:X} at 0x{:X}", opcode.0, self.program_counter - 2); std::process::exit(1); }
        }
    }
}