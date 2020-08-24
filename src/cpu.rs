use super::instructions::Instruction;
use super::memory::Memory;
use super::graphic::Graphic;

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
        } }
}

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
        let opcode = self.memory.read(self.register.pc);
        //// recog instruction
        let instruction = Box::<dyn Instruction>::from(opcode);
        //// execute
        instruction.execute(&mut self.memory, &mut self.register, &mut self.graphic);
    }
}