use crate::instructions::Instruction;
use crate::emulator::{Memory, Register, Graphic};

/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
/// The interpreter reads n bytes from memory, starting at the address stored in I.
/// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
/// Sprites are XORed onto the existing screen.
/// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
/// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps
/// around to the opposite side of the screen.
pub struct Opcode0xdxyn {
    vx: usize,
    vy: usize,
    nibble: u8,
}

impl Opcode0xdxyn {
    pub fn new(instruction: u16) -> Self{
        let vx = (instruction & 0x0F00 >> 8) as usize;
        let vy = (instruction & 0x00F0 >> 4) as usize;
        let nibble = (instruction & 0x00F) as u8;
        Opcode0xdxyn { vx, vy, nibble }
    }
}

impl Instruction for Opcode0xdxyn {
    fn execute(&self, memory: &mut Memory, register: &mut Register, graphic: &mut Graphic) {
        let start = register.i;
        let end = register.i + self.nibble;
        let sprite = memory[start..end];
        let collision = graphic.set_sprite(self.x, self.y, &sprite);
        if collision {
            register.v[0xF] = 1;
        } else {
            register.v[0xF] = 0;
        }
        register.pc = match register.pc.checked_add(2) {
            Some(value) => value,
            None => panic!("program counter exceeds limitation")
        }
    }
}

// TODO add unit test
//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn test_execute() {
//        // instruction 0xc32e
//        let opcode = Opcode0xdxyn {
//            vx: 3 as usize,
//            byte: 0x2e,
//            random_byte: 0x13,
//        };
//        let mut memory = Memory::new();
//        let mut register = Register::new();
//        let mut graphic = Graphic::new();
//        opcode.execute(&mut memory, &mut register, &mut graphic);
//        assert_eq!(register.pc, 2);
//        assert_eq!(register.v[3], 2);
//    }
//}
