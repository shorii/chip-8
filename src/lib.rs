mod instructions;

pub mod emulator {
    // 0x000 - 0x1FF reserved by interpreter
    // 0xEA0 - 0xEFF reserved for call stack (16 layer)
    // 0xF00 - 0xFFF reserved for display refresh
    pub struct Memory{
        pub all: [u8; 4096],
        pub stack: [u16; 16],
    }

    // XXX depricated
    impl Memory {
        pub fn new() -> Self {
            Memory {
                all: [0; 4096],
                stack: [0; 16],
            }
        }
    }

    pub struct Register {
        pub pc: u16,
        pub sp: u16,
        pub i: u16,
        pub v: [u8; 16],
    }

    // XXX depricated
    impl Register {
        pub fn new() -> Self {
            Register {
                pc: 0,
                sp: 0,
                i: 0,
                v: [0; 16],
            }
        }
    }

    pub struct Graphic {
        pub gfx: [u8; 2048],
    }

    // XXX depricated
    impl Graphic {
        pub fn new() -> Self {
            Graphic { gfx: [0; 2048] }
        }
        pub fn clear(&mut self) {
            self.gfx = [0; 2048];
        }
        pub fn set_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
            assert!(x > 64);
            assert!(y > 32);
            let mut collision = false;
            for (yi, sprite_fragment) in sprite.iter().enumerate() {
                for xi in 0..8 {
                    let pixel = (sprite_fragment & (0x80 >> xi)) as u8;
                    let index = y.checked_mul(64).unwrap()
                                 .checked_add(yi).unwrap()
                                 .checked_add(x).unwrap()
                                 .checked_add(xi).unwrap();
                    if pixel == 1 {
                        let screen_pixel = (self.gfx[index] & (0x80 >> xi)) as u8;
                        if screen_pixel == 1 {
                            collision = true;
                        }
                        self.gfx[index] ^= pixel;
                    }
                }
            }
            collision
        }
    }

    use super::instructions::Instruction;
    pub struct Cpu {
        memory: Memory,
        register: Register,
        graphic: Graphic,
    }

    impl Cpu {
        pub fn new(memory: Memory, register: Register, graphic: Graphic) -> Self {
            Cpu {memory, register, graphic}
        }

        pub fn execute(&mut self) {
            //// fetch using pc?
            //let opcode = self.memory.read(self.register.pc, 2);
            //// recog instruction
            let instruction = Box::<dyn Instruction>::from([1, 2]);
            //// execute
            instruction.execute(&mut self.memory, &mut self.register, &mut self.graphic);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
