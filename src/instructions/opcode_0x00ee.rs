use crate::emulator::{Graphic, Memory, Register};
use crate::instructions::Instruction;
use std::sync::mpsc;

/// Return from a subroutine.
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack.
pub struct Opcode0x00ee;

impl Opcode0x00ee {
    pub fn new() -> Self {
        Opcode0x00ee
    }
}

impl Instruction for Opcode0x00ee {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        _graphic: &mut Graphic,
        _keyboard_bus: &mpsc::Receiver<u8>,
    ) {
        register.pc = memory.stack.pop().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let opcode = Opcode0x00ee::new();
        let mut memory = Memory::new();
        memory.stack.push(1);
        memory.stack.push(2);
        memory.stack.push(3);
        let mut register = Register::new();

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(memory.stack, vec![1, 2]);
        assert_eq!(register.pc, 3);
    }
}
