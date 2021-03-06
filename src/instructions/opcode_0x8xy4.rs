use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set Vx = Vx + Vy, set VF = carry.
/// The values of Vx and Vy are added together.
/// If the result is greater than 8 bits, VF is set to 1, otherwise 0.
/// Only the lowest 8 bits of the result are kept, and stored in Vx.
pub struct Opcode0x8xy4 {
    vx: usize,
    vy: usize,
}

impl Opcode0x8xy4 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x8xy4 { vx, vy }
    }
}

impl Instruction for Opcode0x8xy4 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let (result, overflowing) = register.v[self.vx].overflowing_add(register.v[self.vy]);
        register.v[self.vx] = result;
        if overflowing {
            register.v[0xF] = 1;
        } else {
            register.v[0xF] = 0;
        }
        register.pc += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_if_overflowing() {
        let instruction: u16 = 0x8124;
        let opcode = Opcode0x8xy4::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 250;
        register.v[2] = 10;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x202);
        assert_eq!(register.v[1], 4);
        assert_eq!(register.v[15], 1);
    }

    #[test]
    fn test_execute_if_not_overflowing() {
        let instruction: u16 = 0x8124;
        let opcode = Opcode0x8xy4::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 5;
        register.v[2] = 10;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x202);
        assert_eq!(register.v[1], 15);
        assert_eq!(register.v[15], 0);
    }
}
