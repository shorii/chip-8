use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set I = nnn.
/// The value of register I is set to nnn.
pub struct Opcode0xannn {
    address: u16,
}

impl Opcode0xannn {
    pub fn new(instruction: u16) -> Self {
        let address = (instruction & 0x0FFF) as u16;
        Opcode0xannn { address }
    }
}

impl Instruction for Opcode0xannn {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.i = self.address;
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
    fn test_execute() {
        let instruction: u16 = 0xa12e;
        let opcode = Opcode0xannn::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.i, 0x12e);
    }
}