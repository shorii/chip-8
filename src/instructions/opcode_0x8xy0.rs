use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Set Vx = Vy.
/// Stores the value of register Vy in register Vx.
pub struct Opcode0x8xy0 {
    vx: usize,
    vy: usize,
}

impl Opcode0x8xy0 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x8xy0 { vx, vy }
    }
}

impl Instruction for Opcode0x8xy0 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mut mpsc::Receiver<u8>,
    ) {
        register.v[self.vx] = register.v[self.vy];
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
        let instruction: u16 = 0x8120;
        let opcode = Opcode0x8xy0::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x1;
        register.v[2] = 0x2;
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 0x2);
    }
}
