use super::emulator::{Memory, Register, Graphic};

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
mod opcode_0xannn;
mod opcode_0xbnnn;
mod opcode_0xcxkk;

pub trait Instruction {
    fn execute(&self, memory: &mut Memory, register: &mut Register, graphic: &mut Graphic);
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
                    _ => panic!("unsupported operator")
                }
            },
            0xa => Box::new(opcode_0xannn::Opcode0xannn::new(instruction)),
            0xb => Box::new(opcode_0xbnnn::Opcode0xbnnn::new(instruction)),
            0xc => Box::new(opcode_0xcxkk::Opcode0xcxkk::new(instruction)),
            _ => panic!("unsupported operator"),
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
