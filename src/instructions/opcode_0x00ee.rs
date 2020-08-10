use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Return from a subroutine.
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack.
pub struct Opcode0x00ee;

impl Opcode0x00ee {
    pub fn new() -> Self{
        Opcode0x00ee
    }
}

impl Instruction for Opcode0x00ee {
    fn execute(&self, memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
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
        let mut graphic = Graphic::new();
        opcode.execute(&mut memory, &mut register, &mut graphic);
        assert_eq!(register.pc, 3);
        assert_eq!(register.sp, 1);
    }
}
