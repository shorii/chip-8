use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Set Vx = delay timer value.
/// The value of DT is placed into Vx.
pub struct Opcode0xfx07 {
    vx: usize,
}

impl Opcode0xfx07 {
    pub fn new(instruction: u16) -> Self {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx07 { vx }
    }
}

impl Instruction for Opcode0xfx07 {
    fn execute(
        &self,
        _memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        let dt = register.delay_timer.lock().unwrap();
        register.v[self.vx] = *dt;
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_execute() {
        let instruction = 0xf507;
        let opcode = Opcode0xfx07::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.delay_timer = Arc::new(Mutex::new(7));

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(*register.delay_timer.lock().unwrap(), 7);
        assert_eq!(register.pc, 2);
    }
}