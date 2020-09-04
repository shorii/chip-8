use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use rand::prelude::*;
use std::sync::mpsc;

/// Set Vx = random byte AND kk.
/// The interpreter generates a random number from 0 to 255, which is the ANDed with the value kk.
/// The results are stored in Vx.
pub struct Opcode0xcxkk {
    vx: usize,
    byte: u8,
    random_byte: u8,
}

impl Opcode0xcxkk {
    pub fn new(instruction: u16) -> Self {
        let vx = (instruction & 0x0F00 >> 8) as usize;
        let byte = (instruction & 0x00FF) as u8;
        let random_byte = rand::thread_rng().gen::<u8>();
        Opcode0xcxkk {
            vx,
            byte,
            random_byte,
        }
    }
}

impl Instruction for Opcode0xcxkk {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.v[self.vx] = self.random_byte & self.byte;
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
        // instruction 0xc32e
        let opcode = Opcode0xcxkk {
            vx: 3 as usize,
            byte: 0x2e,
            random_byte: 0x13,
        };
        let mut memory = Memory::new();
        let mut register = Register::new();
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 2);
        assert_eq!(register.v[3], 2);
    }
}