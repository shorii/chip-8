use super::emulator::{Memory, Register, Graphic};
use std::sync::mpsc;

mod opcode_0x0nnn;
mod opcode_0x00e0;
mod opcode_0x00ee;
mod opcode_0x1nnn;
mod opcode_0x2nnn;
mod opcode_0x3xkk;
mod opcode_0x4xkk;
mod opcode_0x5xy0;
mod opcode_0x6xkk;
mod opcode_0x7xkk;
mod opcode_0x8xy0;
mod opcode_0x8xy1;
mod opcode_0x8xy2;
mod opcode_0x8xy3;
mod opcode_0x8xy4;
mod opcode_0x8xy5;
mod opcode_0x8xy6;
mod opcode_0x8xy7;
mod opcode_0x8xye;
mod opcode_0x9xy0;
mod opcode_0xannn;
mod opcode_0xbnnn;
mod opcode_0xcxkk;
mod opcode_0xdxyn;
mod opcode_0xex9e;
mod opcode_0xexa1;
mod opcode_0xfx07;
mod opcode_0xfx0a;
mod opcode_0xfx15;
mod opcode_0xfx18;
mod opcode_0xfx1e;
mod opcode_0xfx29;
mod opcode_0xfx33;
mod opcode_0xfx55;
mod opcode_0xfx65;

pub trait Instruction {
    fn execute(
        &self,
        memory: &mut Memory,
        register: &mut Register,
        graphic: &mut Graphic,
        keyboard_bus: &mpsc::Receiver<u8>,
    );
}
 
impl From<[u8; 2]> for Box<dyn Instruction> {
    fn from(opcode: [u8; 2]) -> Box<dyn Instruction> {
        let operator = ((opcode[0] & 0xF0) >> 4) as u8;
        let instruction = ((opcode[0] as u16) << 8) | opcode[1] as u16;
        match operator {
            0x0 => {
                match instruction {
                    0x00EE => Box::new(opcode_0x00ee::Opcode0x00ee::new()),
                    0x00E0 => Box::new(opcode_0x00e0::Opcode0x00e0::new()),
                    _ => Box::new(opcode_0x0nnn::Opcode0x0nnn::new()),
                }
                
            },
            0x1 => Box::new(opcode_0x1nnn::Opcode0x1nnn::new(instruction)),
            0x2 => Box::new(opcode_0x2nnn::Opcode0x2nnn::new(instruction)),
            0x3 => Box::new(opcode_0x3xkk::Opcode0x3xkk::new(instruction)),
            0x4 => Box::new(opcode_0x4xkk::Opcode0x4xkk::new(instruction)),
            0x5 => Box::new(opcode_0x5xy0::Opcode0x5xy0::new(instruction)),
            0x6 => Box::new(opcode_0x6xkk::Opcode0x6xkk::new(instruction)),
            0x7 => Box::new(opcode_0x7xkk::Opcode0x7xkk::new(instruction)),
            0x8 => {
                let suffix = (opcode[1] & 0x0F) as u8;
                match suffix {
                    0x0 => Box::new(opcode_0x8xy0::Opcode0x8xy0::new(instruction)),
                    0x1 => Box::new(opcode_0x8xy1::Opcode0x8xy1::new(instruction)),
                    0x2 => Box::new(opcode_0x8xy2::Opcode0x8xy2::new(instruction)),
                    0x3 => Box::new(opcode_0x8xy3::Opcode0x8xy3::new(instruction)),
                    0x4 => Box::new(opcode_0x8xy4::Opcode0x8xy4::new(instruction)),
                    0x5 => Box::new(opcode_0x8xy5::Opcode0x8xy5::new(instruction)),
                    0x6 => Box::new(opcode_0x8xy6::Opcode0x8xy6::new(instruction)),
                    0x7 => Box::new(opcode_0x8xy7::Opcode0x8xy7::new(instruction)),
                    0xe => Box::new(opcode_0x8xye::Opcode0x8xye::new(instruction)),
                    _ => panic!(
                        "unsupported operator {prefix:x?}{suffix:x?}",
                        prefix=opcode[0],
                        suffix=opcode[1],
                    )
                }
            },
            0x9 => Box::new(opcode_0x9xy0::Opcode0x9xy0::new(instruction)),
            0xa => Box::new(opcode_0xannn::Opcode0xannn::new(instruction)),
            0xb => Box::new(opcode_0xbnnn::Opcode0xbnnn::new(instruction)),
            0xc => Box::new(opcode_0xcxkk::Opcode0xcxkk::new(instruction)),
            0xd => Box::new(opcode_0xdxyn::Opcode0xdxyn::new(instruction)),
            0xe => {
                let suffix = opcode[1];
                match suffix {
                    0x9E => Box::new(opcode_0xex9e::Opcode0xex9e::new(instruction)),
                    0xA1 => Box::new(opcode_0xexa1::Opcode0xexa1::new(instruction)),
                    _ => panic!(
                        "unsupported operator {prefix:x?}{suffix:x?}",
                        prefix=opcode[0],
                        suffix=opcode[1],
                    )
                }
            },
            0xf => {
                let suffix = opcode[1];
                match suffix {
                    0x07 => Box::new(opcode_0xfx07::Opcode0xfx07::new(instruction)),
                    0x0a => Box::new(opcode_0xfx0a::Opcode0xfx0a::new(instruction)),
                    0x15 => Box::new(opcode_0xfx15::Opcode0xfx15::new(instruction)),
                    0x18 => Box::new(opcode_0xfx18::Opcode0xfx18::new(instruction)),
                    0x1e => Box::new(opcode_0xfx1e::Opcode0xfx1e::new(instruction)),
                    0x29 => Box::new(opcode_0xfx29::Opcode0xfx29::new(instruction)),
                    0x33 => Box::new(opcode_0xfx33::Opcode0xfx33::new(instruction)),
                    0x55 => Box::new(opcode_0xfx55::Opcode0xfx55::new(instruction)),
                    0x65 => Box::new(opcode_0xfx65::Opcode0xfx65::new(instruction)),
                    _ => panic!(
                        "unsupported operator {prefix:x?}{suffix:x?}",
                        prefix=opcode[0],
                        suffix=opcode[1],
                    )
                }
            }
            _ => panic!(
                "unsupported operator {prefix:x?}{suffix:x?}",
                prefix=opcode[0],
                suffix=opcode[1],
            )
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
