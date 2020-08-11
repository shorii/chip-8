use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Set Vx = Vx SHR 1.
/// If the most-significant bit of Vx is 1, the VF is set to 1, otherwise 0.
/// Then Vx is multipled by 2.
pub struct Opcode0x8xye {
    vx: usize,
}

impl Opcode0x8xye {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0x8xye { vx }
    }
}

impl Instruction for Opcode0x8xye {
    fn execute(&self, _memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
        let most_significant_bit = ((register.v[self.vx] & 0x80) >> 7) as u8;
        if most_significant_bit == 1 {
            register.v[0xF] = 1;
        } else {
            register.v[0xF] = 0;
        }
        register.v[self.vx] <<= 1;
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
    fn test_execute_if_most_significant_bit_is_one() {
        let instruction: u16 = 0x812e;
        let opcode = Opcode0x8xye::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 200;
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 144);
        assert_eq!(register.v[15], 1);
    }

    #[test]
    fn test_execute_if_most_significant_bit_is_zero() {
        let instruction: u16 = 0x812e;
        let opcode = Opcode0x8xye::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 25;
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 50);
        assert_eq!(register.v[15], 0);
    }
}
