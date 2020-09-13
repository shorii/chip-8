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
        for index in 0..self.vx + 1 {
            let value = memory.all[register.i as usize + index as usize];
            register.v[index] = value;
        }
        register.i += (self.vx + 1) as u16;
        register.pc += 2;
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
        assert_eq!(register.pc, 0x202);
    }
}
