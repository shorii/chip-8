use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Set Vx = Vy -Vx, set VF = NOT borrow.
/// If Vy > Vx, set VF is set to 1, otherwise 0.
/// The Vx is subtracted from Vy, and the results stored in Vx.
pub struct Opcode0x8xy7 {
    vx: usize,
    vy: usize,
}

impl Opcode0x8xy7 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        Opcode0x8xy7 { vx, vy }
    }
}

impl Instruction for Opcode0x8xy7 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let (result, borrowing) = register.v[self.vy].overflowing_sub(register.v[self.vx]);
        register.v[self.vx] = result;
        if borrowing {
            register.v[0xF] = 0;
        } else {
            register.v[0xF] = 1;
        }
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
    fn test_execute_if_not_borrowing() {
        let instruction: u16 = 0x8127;
        let opcode = Opcode0x8xy7::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 5;
        register.v[2] = 10;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 5);
        assert_eq!(register.v[15], 1);
    }

    #[test]
    fn test_execute_if_borrowing() {
        let instruction: u16 = 0x8127;
        let opcode = Opcode0x8xy7::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[1] = 15;
        register.v[2] = 10;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[1], 251);
        assert_eq!(register.v[15], 0);
    }
}
