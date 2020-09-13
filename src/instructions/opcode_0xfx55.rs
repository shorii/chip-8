use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Store registers V0 through Vx in memory starting at location I.
/// The interpreter copies of registers V0 through Vx into memory, starting at the address in I.
pub struct Opcode0xfx55 {
    vx: usize,
}

impl Opcode0xfx55 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx55 { vx }
    }
}

impl Instruction for Opcode0xfx55 {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        for index in 0..self.vx + 1 {
            let value = register.v[index];
            memory.all[register.i as usize + index as usize] = value;
        }
        register.i += self.vx as u16 + 1;
        register.pc += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction = 0xf555;
        let opcode = Opcode0xfx55::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.i = 0xa;
        register.v[0x0] = 0x1;
        register.v[0x1] = 0x2;
        register.v[0x2] = 0x3;
        register.v[0x3] = 0x4;
        register.v[0x4] = 0x5;
        register.v[0x5] = 0x6;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(memory.all[0x9], 0x0);
        assert_eq!(memory.all[0xa], 0x1);
        assert_eq!(memory.all[0xb], 0x2);
        assert_eq!(memory.all[0xc], 0x3);
        assert_eq!(memory.all[0xd], 0x4);
        assert_eq!(memory.all[0xe], 0x5);
        assert_eq!(memory.all[0xf], 0x6);
        assert_eq!(memory.all[0x10], 0x0);
        assert_eq!(register.pc, 0x202);
    }
}
