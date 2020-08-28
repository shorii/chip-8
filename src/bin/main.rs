use chip_8::emulator::{Memory, Register, Graphic, Cpu};
use console::{Keyboard, Graphic as ConsoleGraphic, Console};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

struct Keypad {
    keyMap: HashMap<char, u8>,
    bus: Arc<Mutex<mpsc::Sender<u8>>>,
}

impl Keypad {
    fn new(bus: Arc<Mutex<mpsc::Sender<u8>>>) -> Self {
        let map: [(char, u8); 16] = [
            ('1', 0x1),
            ('2', 0x2),
            ('3', 0x3),
            ('4', 0xc),

            ('q', 0x4),
            ('w', 0x5),
            ('e', 0x6),
            ('r', 0xd),

            ('a', 0x7),
            ('s', 0x8),
            ('d', 0x9),
            ('f', 0xe),

            ('z', 0xa),
            ('x', 0x0),
            ('c', 0xb),
            ('v', 0xf),
        ];
        let keyMap = map.iter().cloned().collect::<HashMap<_, _>>();
        Keypad { keyMap, bus }
    }
}

impl Keyboard for Keypad {
    fn press(&self, key: char) {
        match self.keyMap.get(&key) {
            Some(value) => {
                let bus = self.bus.lock().unwrap();
                bus.send(*value);
            },
            None => {/* do nothing */}
        }
    }
}


fn main() {
    let terminated = Arc::new(AtomicBool::new(false));

    // graphic setup
    let (graphic_sender, graphic_receiver) = mpsc::channel();
    let graphic_receiver = Arc::new(Mutex::new(graphic_receiver));
    let graphic = Graphic::new(graphic_sender);

    let (key_event_sender, key_event_receiver) = mpsc::channel();
    let key_event_sender = Arc::new(Mutex::new(key_event_sender));
    let key_event_receiver = Arc::new(Mutex::new(key_event_receiver));
    let keypad = Keypad::new(key_event_sender);

    let memory = Memory::new();
    let register = Register::new();
    let mut emulator = Cpu::new(memory, register, graphic, Arc::clone(&terminated));

    let mut console = Console::new(graphic_receiver, Box::new(keypad), Arc::clone(&terminated)).unwrap();
    console.run();
    emulator.execute();
    console.join();
}
