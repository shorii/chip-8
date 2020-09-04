use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Read registers V0 through Vx from memory starting at location I.
/// The interpreter reads values from memory starting at location I into registers V0 through Vx.
pub struct Opcode0xfx65 {
    vx: usize,
}

impl Opcode0xfx65 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx65 { vx }
    }
}

impl Instruction for Opcode0xfx65 {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let start = register.i as usize;
        let end = register.i.checked_add(self.vx as u16).unwrap() as usize;
        memory.all[start..=end].swap_with_slice(&mut register.v[0..=self.vx as usize]);
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation"),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction = 0xf565;
        let opcode = Opcode0xfx65::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.i = 0xa;

        memory.all[0xa] = 0x1;
        memory.all[0xb] = 0x2;
        memory.all[0xc] = 0x3;
        memory.all[0xd] = 0x4;
        memory.all[0xe] = 0x5;
        memory.all[0xf] = 0x6;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(register.v[0x0], 0x1);
        assert_eq!(register.v[0x1], 0x2);
        assert_eq!(register.v[0x2], 0x3);
        assert_eq!(register.v[0x3], 0x4);
        assert_eq!(register.v[0x4], 0x5);
        assert_eq!(register.v[0x5], 0x6);
        assert_eq!(register.pc, 2);
    }
}