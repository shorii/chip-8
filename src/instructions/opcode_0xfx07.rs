use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Set Vx = delay timer value.
/// The value of DT is placed into Vx.
pub struct Opcode0xfx07 {
    vx: usize,
}

impl Opcode0xfx07 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx07 { vx }
    }
}

impl Instruction for Opcode0xfx07 {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        match keyboard_bus.try_recv() {
            Ok(value) if value == register.v[self.vx] => { /* do nothing */ },
            _ => {
                register.pc = match register.pc.checked_add(2) {
                    Some(value) => value,
                    None => panic!("program counter exceeds limitation")
                }
            }
        };

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
    fn test_execute_match() {
        let instruction = 0xeaa1;
        let opcode = Opcode0xfx07::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0xa] = 0x4;
        register.pc = 0;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x4);
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
    }

    #[test]
    fn test_execute_not_match() {
        let instruction = 0xeba1;
        let opcode = Opcode0xfx07::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0xb] = 0x5;
        register.pc = 0;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x4);
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 4);
    }
}
