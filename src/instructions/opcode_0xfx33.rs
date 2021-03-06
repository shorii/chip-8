use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Store BCD representaion of Vx in memory locations, I, I+1, and I+2.
/// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
/// location I, the ten digit at location I+1, and the ones digit at location I+2.
pub struct Opcode0xfx33 {
    vx: usize,
}

impl Opcode0xfx33 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx33 { vx }
    }
}

impl Instruction for Opcode0xfx33 {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        memory.all[register.i as usize] = register.v[self.vx] / 100;
        memory.all[register.i as usize + 1] = (register.v[self.vx] % 100) / 10;
        memory.all[register.i as usize + 2] = register.v[self.vx] % 10;
        register.pc += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction = 0xf533;
        let opcode = Opcode0xfx33::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.i = 0xa;
        register.v[0x5] = 0x7b; // 123

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(memory.all[0xa], 0x1);
        assert_eq!(memory.all[0xb], 0x2);
        assert_eq!(memory.all[0xc], 0x3);
        assert_eq!(register.pc, 0x202);
    }
}
