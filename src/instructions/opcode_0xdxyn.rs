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
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        let nibble = (instruction & 0x000F) as u8;
        Opcode0xdxyn { vx, vy, nibble }
    }
}

impl Instruction for Opcode0xdxyn {
    fn execute(&self, memory: &mut Memory, register: &mut Register, graphic: &mut Graphic) {
        let start = register.i as usize;
        let end = register.i as usize + self.nibble as usize;
        let sprite = &memory.all[start..end];
        let collision = graphic.set_sprite(
            register.v[self.vx] as usize, register.v[self.vy] as usize, sprite
        );
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_no_collision() {
        let instruction = 0xd123;
        let opcode = Opcode0xdxyn::new(instruction);

        let mut memory = Memory::new();
        memory.all[0x12] = 0xF0; // 0b11110000
        memory.all[0x13] = 0x0F; // 0b00001111
        memory.all[0x14] = 0xFF; // 0b11111111

        let (x, y): (usize, usize) = (1, 2);

        let mut register = Register::new();
        register.i = 0x12;
        register.v[0x1] = x as u8;
        register.v[0x2] = y as u8;

        let mut graphic = Graphic::new();

        opcode.execute(&mut memory, &mut register, &mut graphic);

        assert_eq!(graphic.gfx[y * 64 + (x + 0)], 1);
        assert_eq!(graphic.gfx[y * 64 + (x + 1)], 1);
        assert_eq!(graphic.gfx[y * 64 + (x + 2)], 1);
        assert_eq!(graphic.gfx[y * 64 + (x + 3)], 1);
        assert_eq!(graphic.gfx[y * 64 + (x + 4)], 0);
        assert_eq!(graphic.gfx[y * 64 + (x + 5)], 0);
        assert_eq!(graphic.gfx[y * 64 + (x + 6)], 0);
        assert_eq!(graphic.gfx[y * 64 + (x + 7)], 0);

        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 0)], 0);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 1)], 0);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 2)], 0);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 3)], 0);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 4)], 1);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 5)], 1);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 6)], 1);
        assert_eq!(graphic.gfx[(y + 1) * 64 + (x + 7)], 1);

        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 0)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 1)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 2)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 3)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 4)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 5)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 6)], 1);
        assert_eq!(graphic.gfx[(y + 2) * 64 + (x + 7)], 1);

        assert_eq!(register.v[0xF], 0);
        assert_eq!(register.pc, 2);
    }
}
