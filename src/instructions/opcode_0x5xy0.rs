use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Skip next instruction if Vx = Vy.
/// The interpreter compares register Vx to Vy, and if they are equal, increments the pragram
/// counter by 2.(an opcode takes up 2 bytes)
pub struct Opcode0x5xy0 {
    vx: usize,
    vy: usize,
}

impl Opcode0x5xy0 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x5xy0 { vx, vy }
    }
}

impl Instruction for Opcode0x5xy0 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let mut increment = 0;
        if register.v[self.vx] == register.v[self.vy] {
            increment += 2;
        }
        increment += 2;
        register.pc = match register.pc.checked_add(increment) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_if_equal() {
        let instruction: u16 = 0x31a0;
        let opcode = Opcode0x5xy0::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x23;
        register.v[10] = 0x23;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 4);
    }

    #[test]
    fn test_execute_if_not_equal() {
        let instruction: u16 = 0x31a0;
        let opcode = Opcode0x5xy0::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x23;
        register.v[10] = 0x24;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
    }
}