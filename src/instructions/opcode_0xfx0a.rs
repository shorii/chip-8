use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Wait for a key press, store the value of the key in Vx.
/// All execution stops until a key is pressed, then the value of that key is stored in Vx
pub struct Opcode0xfx0a {
    vx: usize,
}

impl Opcode0xfx0a {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx0a { vx }
    }
}

impl Instruction for Opcode0xfx0a {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        match keyboard_bus.recv() {
            Ok(key) => {
                register.v[self.vx] = key;
            },
            _ => { /* do nothing*/ },
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
    fn test_execute() {
        let instruction = 0xf50a;
        let opcode = Opcode0xfx0a::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (sender, receiver) = mpsc::channel();
        sender.send(0x9);

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(register.v[0x5], 0x9);
        assert_eq!(register.pc, 2);
    }
}
