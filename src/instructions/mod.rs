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

pub trait Instruction {
    fn execute(&self, memory: &mut Memory, register: &mut Register, graphic: &mut Graphic);
}
 
impl From<[u8; 2]> for Box<dyn Instruction> {
    fn from(opcode: [u8; 2]) -> Box<dyn Instruction> {
        let operator = ((opcode[0] & 0xF0) >> 4) as u8;
        let instruction = ((opcode[0] as u16) << 8) | opcode[1] as u16;
        match operator {
            0 => {
                match instruction {
                    0x00EE => Box::new(opcode_0x00ee::Opcode0x00ee::new()),
                    0x00E0 => Box::new(opcode_0x00e0::Opcode0x00e0::new()),
                    _ => Box::new(opcode_0x0nnn::Opcode0x0nnn::new()),
                }
                
            },
            1 => Box::new(opcode_0x1nnn::Opcode0x1nnn::new(instruction)),
            2 => Box::new(opcode_0x2nnn::Opcode0x2nnn::new(instruction)),
            3 => Box::new(opcode_0x3xkk::Opcode0x3xkk::new(instruction)),
            4 => Box::new(opcode_0x4xkk::Opcode0x4xkk::new(instruction)),
            5 => Box::new(opcode_0x5xy0::Opcode0x5xy0::new(instruction)),
            6 => Box::new(opcode_0x6xkk::Opcode0x6xkk::new(instruction)),
            7 => Box::new(opcode_0x7xkk::Opcode0x7xkk::new(instruction)),
            8 => {
                let suffix = (opcode[1] & 0x0F) as u8;
                match suffix {
                    0 => Box::new(opcode_0x8xy0::Opcode0x8xy0::new(instruction)),
                    1 => Box::new(opcode_0x8xy1::Opcode0x8xy1::new(instruction)),
                    2 => Box::new(opcode_0x8xy2::Opcode0x8xy2::new(instruction)),
                    3 => Box::new(opcode_0x8xy3::Opcode0x8xy3::new(instruction)),
                    4 => Box::new(opcode_0x8xy4::Opcode0x8xy4::new(instruction)),
                    5 => Box::new(opcode_0x8xy5::Opcode0x8xy5::new(instruction)),
                    _ => panic!("unsupported operation")
                }
            },
            _ => panic!("not supported operator"),
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
