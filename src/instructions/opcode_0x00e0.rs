use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Clear the display.
pub struct Opcode0x00e0;

impl Opcode0x00e0 {
    pub fn new() -> Self {
        Opcode0x00e0
    }
}

impl Instruction for Opcode0x00e0 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        graphic.clear();
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
        let opcode = Opcode0x00e0::new();
        let mut memory = Memory::new();
        let mut register = Register::new();
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        graphic.gfx = [1; 2048];
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        for gfx in graphic.gfx.iter() {
            assert_eq!(*gfx, 0);
        }
    }
}