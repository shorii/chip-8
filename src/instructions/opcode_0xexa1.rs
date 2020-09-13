use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Skip next instruction if key with the value of Vx is not pressed.
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up
/// position, PC is increased by 2.
pub struct Opcode0xexa1 {
    vx: usize,
}

impl Opcode0xexa1 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xexa1 { vx }
    }
}

impl Instruction for Opcode0xexa1 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        match keyboard_bus.try_recv() {
            Ok(value) if value == register.v[self.vx] => {
                register.pc += 2;
            }
            _ => {
                register.pc += 4;
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_match() {
        let instruction = 0xeaa1;
        let opcode = Opcode0xexa1::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0xa] = 0x4;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x4).unwrap();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x202);
    }

    #[test]
    fn test_execute_not_match() {
        let instruction = 0xeba1;
        let opcode = Opcode0xexa1::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0xb] = 0x5;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x4).unwrap();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x204);
    }
}
