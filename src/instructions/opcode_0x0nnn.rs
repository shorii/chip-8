use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Jump to a machine code routine at nnn.
/// This instruction is only used on the old computers on which Chip-8 was originally implemented.
/// It is ignored by modern interpreters.
pub struct Opcode0x0nnn;

impl Opcode0x0nnn {
    pub fn new() -> Self {
        Opcode0x0nnn
    }
}

impl Instruction for Opcode0x0nnn {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
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
        let opcode = Opcode0x0nnn::new();
        let mut memory = Memory::new();
        let mut register = Register::new();

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0);
        assert_eq!(register.sp, 0);
    }
}