use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Jump to location nnn + V0.
/// The program counter is set to nnn plus the value of V0.
pub struct Opcode0xbnnn {
    address: u16,
}

impl Opcode0xbnnn {
    pub fn new(instruction: u16) -> Self {
        let address = (instruction & 0x0FFF) as u16;
        Opcode0xbnnn { address }
    }
}

impl Instruction for Opcode0xbnnn {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let address = self.address + register.v[0] as u16;
        register.pc = address;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction: u16 = 0xb12e;
        let opcode = Opcode0xbnnn::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.v[0] = 0x1;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x12f);
    }
}
