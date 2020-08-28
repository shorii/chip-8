use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

/// Return from a subroutine.
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack.
pub struct Opcode0x00ee;

impl Opcode0x00ee {
    pub fn new() -> Self{
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
        let return_address = memory.stack[register.sp as usize];
        register.sp = match register.sp.checked_sub(1) {
            Some(value) => value,
            None => panic!("stack pointer exceed limitation"),
        };
        register.pc = return_address;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let opcode = Opcode0x00ee::new();
        let mut memory = Memory::new();
        memory.stack[0] = 1;
        memory.stack[1] = 2;
        memory.stack[2] = 3;
        let mut register = Register::new();
        register.sp = 2;

        let (sender, _) = mpsc::channel();
        let mut graphic = Graphic::new(sender);

        let (_, receiver) = mpsc::channel();

        opcode.execute(&mut memory, &mut register, &mut graphic, &receiver);
        assert_eq!(register.pc, 3);
        assert_eq!(register.sp, 1);
    }
}
