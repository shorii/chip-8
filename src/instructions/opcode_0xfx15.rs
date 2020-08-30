use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Set delay time = Vx.
/// DT is set equal to the value of Vx.
pub struct Opcode0xfx15 {
    vx: usize,
}

impl Opcode0xfx15 {
    pub fn new(instruction: u16) -> Self{
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        Opcode0xfx15 { vx }
    }
}

impl Instruction for Opcode0xfx15 {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.v[self.vx] = *register.delay_timer.lock().unwrap();
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
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
        let instruction = 0xf515;
        let opcode = Opcode0xfx15::new(instruction);
        let mut memory = Memory::new();

        let mut register = Register::new();
        register.delay_timer = Arc::new(Mutex::new(0xa));

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);

        assert_eq!(register.v[0x5], 0xa);
        assert_eq!(register.pc, 2);
    }
}
