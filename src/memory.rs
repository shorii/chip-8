use std::convert::TryFrom;

// 0x000 - 0x1FF reserved by interpreter
// 0xEA0 - 0xEFF reserved for call stack (16 layer)
// 0xF00 - 0xFFF reserved for display refresh
pub struct Memory{
    pub all: [u8; 4096],
    pub stack: [u16; 16],
}

// XXX depricated
impl Memory {
    pub fn new() -> Self {
        Memory {
            all: [0; 4096],
            stack: [0; 16],
        }
    }

    pub fn read(&self, program_counter: u16) -> [u8; 2] {
        let s = program_counter as usize;
        let e = s + 2;
        <[u8; 2]>::try_from(&self.all[s..e]).unwrap()
    }
}
