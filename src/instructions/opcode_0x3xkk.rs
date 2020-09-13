use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Skip next instruction if Vx = kk.
/// The interpreter compares register Vx to kk, and if they are equal, increments the pragram
/// counter by 2.(an opcode takes up 2 bytes)
pub struct Opcode0x3xkk {
    vx: usize,
    byte: u8,
}

impl Opcode0x3xkk {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let byte = (instruction & 0x00FF) as u8;
        Opcode0x3xkk { vx, byte }
    }
}

impl Instruction for Opcode0x3xkk {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        if register.v[self.vx] == self.byte {
            register.pc += 4;
        } else {
            register.pc += 2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_if_equal() {
        let instruction: u16 = 0x3123;
        let opcode = Opcode0x3xkk::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x23;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x204);
    }

    #[test]
    fn test_execute_if_not_equal() {
        let instruction: u16 = 0x3123;
        let opcode = Opcode0x3xkk::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x24;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x202);
    }
}
