use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Skip next instruction if key with the value of Vx is pressed.
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
/// position, PC is increased by 2.
pub struct Opcode0xex9e {
    vx: usize,
}

impl Opcode0xex9e {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xex9e { vx }
    }
}

impl Instruction for Opcode0xex9e {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let increment = match keyboard_bus.try_recv() {
            Ok(value) if value == register.v[self.vx] => 4,
            _ => 2,
        };

        register.pc = match register.pc.checked_add(increment) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_match() {
        let instruction = 0xea9e;
        let opcode = Opcode0xex9e::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0xa] = 0x4;
        register.pc = 0;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x4).unwrap();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 4);
    }

    #[test]
    fn test_execute_not_match() {
        let instruction = 0xeb9e;
        let opcode = Opcode0xex9e::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0xb] = 0x5;
        register.pc = 0;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x4).unwrap();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
    }
}