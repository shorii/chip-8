use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set I = I + Vx.
/// The values of I and Vx are added, and the results are stored in I.
pub struct Opcode0xfx1e {
    vx: usize,
}

impl Opcode0xfx1e {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx1e { vx }
    }
}

impl Instruction for Opcode0xfx1e {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.i += register.v[self.vx] as u16;
        register.pc += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction = 0xf51e;
        let opcode = Opcode0xfx1e::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.i = 0xa;
        register.v[0x5] = 0x2;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(register.i, 0xc);
        assert_eq!(register.pc, 0x202);
    }
}
