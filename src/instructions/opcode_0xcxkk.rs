use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};
use rand::{thread_rng, Rng};

/// Set Vx = random byte AND kk.
/// The interpreter generates a random number from 0 to 255, which is the ANDed with the value kk.
/// The results are stored in Vx.
pub struct Opcode0xcxkk {
    vx: usize,
    byte: u8,
}

impl Opcode0xcxkk {
    pub fn new(instruction: u16) -> Self{
        let vx = (instruction & 0x0F00 >> 8) as usize;
        let byte = (instruction & 0x00FF) as u8;
        Opcode0xcxkk { vx, byte }
    }
}

impl Instruction for Opcode0xcxkk {
    fn execute(&self, _memory: &mut Memory, register: &mut Register, _graphic: &mut Graphic) {
        let mut rng = rand::thread_rng();
        let random_byte = rng.gen::<u8>();
        register.v[self.vx] = random_byte & self.byte;
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
        }
    }
}

// TODO consider how to test
//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn test_execute() {
//        let instruction: u16 = 0xc32e;
//        let opcode = Opcode0xcxkk::new(instruction);
//        let mut memory = Memory::new();
//        let mut register = Register::new();
//        register.v[0] = 0x1;
//        let mut graphic = Graphic::new();
//        opcode.execute(&mut memory, &mut register, &mut graphic);
//        assert_eq!(register.pc, 0x12f);
//    }
//}
