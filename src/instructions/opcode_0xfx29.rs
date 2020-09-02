use crate::instructions::Instruction;
use crate::emulator::{Memory, FONT_BASE, FONT_LENGTH, Register, Graphic};
use std::sync::mpsc;

/// Set I = location of sprite for digit Vx.
/// The value of I is set to the location for the hexadecimal sprite corresponding to the value of
/// Vx.
pub struct Opcode0xfx29 {
    vx: usize,
}

impl Opcode0xfx29 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx29 { vx }
    }
}

impl Instruction for Opcode0xfx29 {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let digit = register.v[self.vx] as u16;
        let font_base = FONT_BASE as u16;
        let font_length = FONT_LENGTH as u16;

        register.i = font_base.checked_add(
            digit.checked_mul(font_length).unwrap()
        ).unwrap();

        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction = 0xf529;
        let opcode = Opcode0xfx29::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.i = 0xa;
        register.v[0x5] = 0xe;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(register.i, 0x46);
        assert_eq!(register.pc, 2);
    }
}
