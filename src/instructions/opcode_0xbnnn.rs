use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Jump to location nnn + V0.
/// The program counter is set to nnn plus the value of V0.
pub struct Opcode0xbnnn {
    address: u16,
}

impl Opcode0xbnnn {
    pub fn new(instruction: u16) -> Self{
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
        _keyboard_bus: &mut mpsc::Receiver<u8>,
    ) {
        let address = match self.address.checked_add(register.v[0] as u16) {
            Some(value) => value,
            None => panic!("invalid address access")
        };
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
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 0x12f);
    }
}
