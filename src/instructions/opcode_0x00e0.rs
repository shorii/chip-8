use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Clear the display.
pub struct Opcode0x00e0;

impl Opcode0x00e0 {
    pub fn new() -> Self{
        Opcode0x00e0
    }
}

impl Instruction for Opcode0x00e0 {
    fn execute(&self, _memory: &mut Memory, _register: &mut Register, graphic: &mut Graphic) {
        graphic.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let opcode = Opcode0x00e0::new();
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut graphic = Graphic::new();
        graphic.gfx = [1; 2048];
        opcode.execute(&mut memory, &mut register, &mut graphic);
        for gfx in graphic.gfx.iter() {
            assert_eq!(*gfx, 0);
        }
    }
}
