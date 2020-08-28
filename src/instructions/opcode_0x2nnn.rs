use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Call subroutine at nnn.
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
/// The PC is then set to nnn.
pub struct Opcode0x2nnn {
    address: u16,
}

impl Opcode0x2nnn {
    pub fn new(instruction: u16) -> Self{
        let address = instruction & 0x0FFF;
        Opcode0x2nnn { address }
    }
}

impl Instruction for Opcode0x2nnn {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>
    ) {
        memory.stack[register.sp as usize] = register.pc;
        register.sp = match register.sp.checked_add(1) {
            Some(value) => value,
            None => panic!("stack pointer exceed limitation"),
        };
        register.pc = self.address;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let instruction: u16 = 0x2123;
        let opcode = Opcode0x2nnn::new(instruction);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.pc = 1;
        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);
        let (_, receiver) = mpsc::channel();
        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 0x123);
        assert_eq!(register.sp, 1);
        assert_eq!(memory.stack[0], 1);
    }
}
