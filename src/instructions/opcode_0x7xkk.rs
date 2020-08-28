use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// ADD Vx = Vx + kk.
/// Adds the value kk to the value of register Vx, the stores the result in Vx.
pub struct Opcode0x7xkk {
    vx: usize,
    byte: u8,
}

impl Opcode0x7xkk {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let byte = (instruction & 0x00FF) as u8;
        Opcode0x7xkk { vx, byte }
    }
}

impl Instruction for Opcode0x7xkk {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mut mpsc::Receiver<u8>,
    ) {
        register.v[self.vx] = register.v[self.vx].wrapping_add(self.byte);
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
        let instruction: u16 = 0x7123;
        let opcode = Opcode0x7xkk::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x1;
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 0x24);
    }
}
