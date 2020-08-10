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
