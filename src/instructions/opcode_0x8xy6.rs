use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set Vx = Vx SHR 1.
/// If the least-significant bit of Vx is 1, the VF is set to 1, otherwise 0.
/// Then Vx is divided by 2.
pub struct Opcode0x8xy6 {
    vx: usize,
}

impl Opcode0x8xy6 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0x8xy6 { vx }
    }
}

impl Instruction for Opcode0x8xy6 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let least_significant_bit = (register.v[self.vx] & 0x0001) as u8;
        if least_significant_bit == 1 {
            register.v[0xF] = 1;
        } else {
            register.v[0xF] = 0;
        }
        register.v[self.vx] >>= 1;
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
    fn test_execute_if_least_significant_bit_is_one() {
        let instruction: u16 = 0x8126;
        let opcode = Opcode0x8xy6::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 255;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 127);
        assert_eq!(register.v[15], 1);
    }

    #[test]
    fn test_execute_if_least_significant_bit_is_zero() {
        let instruction: u16 = 0x8126;
        let opcode = Opcode0x8xy6::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 254;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 127);
        assert_eq!(register.v[15], 0);
    }
}