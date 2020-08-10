use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Set Vx = Vx OR Vy.
/// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
/// A bitwise OR compares the corresponding bits from two values, and if either bit is 1, then the
/// same bit in the result is also 1. Otherwise, it is 0.
pub struct Opcode0x8xy1 {
    vx: usize,
    vy: usize,
}

impl Opcode0x8xy1 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x8xy1 { vx, vy }
    }
}

impl Instruction for Opcode0x8xy1 {
    fn execute(&self, _memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
        register.v[self.vx] |= register.v[self.vy];
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
        let instruction: u16 = 0x8121;
        let opcode = Opcode0x8xy1::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0b0101;
        register.v[2] = 0b1001;
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 0b1101);
    }
}
