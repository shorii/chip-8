use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Set Vx = Vx SHR 1.
/// If the least-significant bit of Vx is 1, the VF is set to 1, otherwise 0.
/// Then Vx is divided by 2.
pub struct Opcode0x8xy6 {
    vx: usize,
}

impl Opcode0x8xy6 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0x8xy6 { vx }
    }
}

impl Instruction for Opcode0x8xy6 {
    fn execute(&self, _memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
        let least_significant_bit = (register.v[self.x] & 0x0001) as u8;
        if least_significant_bit == 1 {
            register.v[15] = 1;
        } else {
            register.v[15] = 0;
        }
        register.v[vx] >>= 1;
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
        }
    }
}
//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn test_execute_if_not_borrowing() {
//        let instruction: u16 = 0x8125;
//        let opcode = Opcode0x8xy6::new(instruction);
//        let mut memory = Memory::new();
//        let mut register = Register::new();
//        register.v[1] = 250;
//        register.v[2] = 10;
//        let mut graphic = Graphic::new();
//        opcode.execute(&mut memory, &mut register, &mut graphic);
//        assert_eq!(register.pc, 2);
//        assert_eq!(register.v[1], 240);
//        assert_eq!(register.v[15], 1);
//    }
//
//    #[test]
//    fn test_execute_if_borrowing() {
//        let instruction: u16 = 0x8124;
//        let opcode = Opcode0x8xy6::new(instruction);
//        let mut memory = Memory::new();
//        let mut register = Register::new();
//        register.v[1] = 5;
//        register.v[2] = 10;
//        let mut graphic = Graphic::new();
//        opcode.execute(&mut memory, &mut register, &mut graphic);
//        assert_eq!(register.pc, 2);
//        assert_eq!(register.v[1], 251);
//        assert_eq!(register.v[15], 0);
//    }
//}
