use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set Vx = Vx AND Vy.
/// Performs a bitwise ADN on the values of Vx and Vy, then stores the result in Vx.
/// A bitwise AND compares the corresponding bits from two values, and if both bits are 1, then the
/// same bit in the result is also 1. Otherwise, it is 0.
pub struct Opcode0x8xy2 {
    vx: usize,
    vy: usize,
}

impl Opcode0x8xy2 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x8xy2 { vx, vy }
    }
}

impl Instruction for Opcode0x8xy2 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.v[self.vx] &= register.v[self.vy];
        register.pc += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction: u16 = 0x8122;
        let opcode = Opcode0x8xy2::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0b0101;
        register.v[2] = 0b1001;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x202);
        assert_eq!(register.v[1], 0b0001);
    }
}
