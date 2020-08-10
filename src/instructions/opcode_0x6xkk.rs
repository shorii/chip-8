use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Set Vx = kk.
/// The interpreter puts the value kk into register Vx.
pub struct Opcode0x6xkk {
    vx: usize,
    byte: u8,
}

impl Opcode0x6xkk {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let byte = (instruction & 0x00FF) as u8;
        Opcode0x6xkk { vx, byte }
    }
}

impl Instruction for Opcode0x6xkk {
    fn execute(&self, _memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
        register.v[self.vx] = self.byte;
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction: u16 = 0x6123;
        let opcode = Opcode0x6xkk::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 0x23);
    }
}
