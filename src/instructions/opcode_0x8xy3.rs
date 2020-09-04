use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set Vx = Vx XOR Vy.
/// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
/// A exclusive OR compares the corresponding bits from two values, and if both bits are not the
/// same, then the corresponding bit in the result is set to 1.
pub struct Opcode0x8xy3 {
    vx: usize,
    vy: usize,
}

impl Opcode0x8xy3 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x8xy3 { vx, vy }
    }
}

impl Instruction for Opcode0x8xy3 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.v[self.vx] ^= register.v[self.vy];
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction: u16 = 0x8123;
        let opcode = Opcode0x8xy3::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0b0101;
        register.v[2] = 0b1001;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 0b1100);
    }
}