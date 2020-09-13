use super::graphic::Graphic;
use super::instructions::Instruction;
use super::memory::Memory;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

pub struct Register {
    pub pc: u16,
    pub i: u16,
    pub v: [u8; 16],
    pub delay_timer: Arc<Mutex<u8>>,
    pub sound_timer: Arc<Mutex<u8>>,
}

impl Register {
    pub fn new() -> Self {
        Register {
            pc: 0x200,
            i: 0,
            v: [0; 16],
            delay_timer: Arc::new(Mutex::new(0)),
            sound_timer: Arc::new(Mutex::new(0)),
        }
    }

    pub fn run_timer(&self, terminated: Arc<AtomicBool>) {
        let delay_timer = Arc::clone(&self.delay_timer);
        let sound_timer = Arc::clone(&self.sound_timer);
        let interval = time::Duration::from_millis(15);
        thread::spawn(move || {
            while !terminated.load(Ordering::Relaxed) {
                thread::sleep(interval);
                let mut dt = delay_timer.lock().unwrap();
                let mut st = sound_timer.lock().unwrap();
                if *dt > 0 {
                    *dt -= 1;
                }
                if *st > 0 {
                    // TODO beeping here
                    *st -= 1;
                }
            }
        });
    }
}

pub struct Cpu {
    memory: Memory,
    register: Register,
    graphic: Graphic,
    keyboard_bus: mpsc::Receiver<u8>,
}

impl Cpu {
    pub fn new(
        memory: Memory,
        register: Register,
        graphic: Graphic,
        keyboard_bus: mpsc::Receiver<u8>,
    ) -> Self {
        Cpu {
            memory,
            register,
            graphic,
            keyboard_bus,
        }
    }

    pub fn execute(&mut self, terminated: Arc<AtomicBool>) {
        self.register.run_timer(Arc::clone(&terminated));
        while !terminated.load(Ordering::Relaxed) {
            // fetch using pc?
            let opcode = self.memory.read(self.register.pc);
            // recog instruction
            let instruction = Box::<dyn Instruction>::from(opcode);
            // execute
            thread::sleep(time::Duration::from_millis(2));
            instruction.execute(
                &mut self.memory,
                &mut self.register,
                &mut self.graphic,
                &mut self.keyboard_bus,
            );
            // draw
            self.graphic.draw();
        }
    }
}
