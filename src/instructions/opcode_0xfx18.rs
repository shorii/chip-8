use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

/// Set sound timer = Vx.
/// ST is set equal to the value of Vx.
pub struct Opcode0xfx18 {
    vx: usize,
}

impl Opcode0xfx18 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx18 { vx }
    }
}

impl Instruction for Opcode0xfx18 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.sound_timer = Arc::new(Mutex::new(register.v[self.vx]));
        register.pc += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction = 0xf518;
        let opcode = Opcode0xfx18::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.v[0x5] = 0xb;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(*register.sound_timer.lock().unwrap(), 0xb);
        assert_eq!(register.pc, 0x202);
    }
}
