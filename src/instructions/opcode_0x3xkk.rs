use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Skip next instruction if Vx = kk.
/// The interpreter compares register Vx to kk, and if they are equal, increments the pragram
/// counter by 2.(an opcode takes up 2 bytes)
pub struct Opcode0x3xkk {
    vx: usize,
    byte: u8,
}

impl Opcode0x3xkk {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let byte = (instruction & 0x00FF) as u8;
        Opcode0x3xkk { vx, byte }
    }
}

impl Instruction for Opcode0x3xkk {
    fn execute(&self, _memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
        let mut increment = 0;
        if register.v[self.vx] == self.byte {
            increment += 2;
        }
        increment += 2;
        register.pc = match register.pc.checked_add(increment) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
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
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 4);
    }

    #[test]
    fn test_execute_if_not_equal() {
        let instruction: u16 = 0x3123;
        let opcode = Opcode0x3xkk::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 0x24;
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
    }
}
