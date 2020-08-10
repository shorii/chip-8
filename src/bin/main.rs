use chip_8::emulator::{Memory, Register, Graphic, Cpu};

fn main() {
    let memory = Memory::new();
    let register = Register::new();
    let graphic = Graphic::new();
    let mut cpu = Cpu::new(memory, register, graphic);
    cpu.execute();
}
