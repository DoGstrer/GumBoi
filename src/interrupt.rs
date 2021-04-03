use super::Memory;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

const IF_ADDR: u16 = 0xFF0F;
const IE_ADDR: u16 = 0xFFFF;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum InterruptType {
    VBLANK,
    LCD_STAT,
    TIMER,
    SERIAL,
    JOYPAD,
    EXIT,
}

impl InterruptType {
    fn get_register_bit(&self) -> u8 {
        *self as u8
    }
    fn get_rst_addr(&self) -> u16 {
        match self {
            InterruptType::VBLANK => 0x40,
            InterruptType::LCD_STAT => 0x48,
            InterruptType::TIMER => 0x50,
            InterruptType::SERIAL => 0x58,
            InterruptType::JOYPAD => 0x60,
            _ => panic!("Invalid Interrupt"),
        }
    }
    fn get_interrupt_from_bit(bit: u8) -> InterruptType {
        match bit {
            0 => InterruptType::VBLANK,
            1 => InterruptType::LCD_STAT,
            2 => InterruptType::TIMER,
            3 => InterruptType::SERIAL,
            4 => InterruptType::JOYPAD,
            _ => panic!("Invalid Interrupt bit"),
        }
    }
}

pub struct InterruptController {
    memory: Arc<Mutex<Memory>>,
    ime: Arc<Mutex<bool>>,
}

impl InterruptController {
    pub fn new(
        memory: Arc<Mutex<Memory>>,
        ime: Arc<Mutex<bool>>,
        interrupt_rx: mpsc::Receiver<InterruptType>,
    ) -> InterruptController {
        let memory_interrupt = Arc::clone(&memory);
        thread::spawn(move || {
            InterruptController::request_handler(memory_interrupt, interrupt_rx);
        });
        return InterruptController {
            memory: memory,
            ime: ime,
        };
    }
    pub fn request_handler(
        memory: Arc<Mutex<Memory>>,
        interrupt_rx: mpsc::Receiver<InterruptType>,
    ) {
        for interrupt in interrupt_rx {
            match interrupt {
                InterruptType::EXIT => return,
                InterruptType::VBLANK
                | InterruptType::TIMER
                | InterruptType::SERIAL
                | InterruptType::LCD_STAT
                | InterruptType::JOYPAD => {
                    let if_register = memory.lock().unwrap().get_addr(IF_ADDR);
                    memory
                        .lock()
                        .unwrap()
                        .set_addr(IF_ADDR, if_register | 1 << interrupt.get_register_bit());
                }
                _ => panic!("Invalid Interrupt Raised"),
            }
        }
    }
    pub fn execute(&self) -> Option<u16> {
        match self.get_interrupt_request() {
            Some(interrupt) => {
                self.clear_ime();
                self.disable_interrupt_request(interrupt);
                Some(interrupt.get_rst_addr())
            }
            None => None,
        }
    }
    fn get_interrupt_request(&self) -> Option<InterruptType> {
        let interrupt_requests_register = self.memory.lock().unwrap().get_addr(IF_ADDR);
        let interrupt_enable_register = self.memory.lock().unwrap().get_addr(IE_ADDR);
        if self.is_set_ime() {
            for i in 0..5 {
                if (interrupt_requests_register >> i & 1 == 1)
                    && (interrupt_enable_register >> i & 1 == 1)
                {
                    return Some(InterruptType::get_interrupt_from_bit(i));
                }
            }
        }
        None
    }
    fn is_interrupt_enabled(&self, interrupt: InterruptType) -> bool {
        let ie_register = self.memory.lock().unwrap().get_addr(IE_ADDR);
        match (ie_register >> interrupt.get_register_bit() & 0x01) {
            0 => false,
            1 => true,
            _ => panic!("Invalid Register Value"),
        }
    }
    fn disable_interrupt_request(&self, interrupt: InterruptType) {
        let if_register = self.memory.lock().unwrap().get_addr(IF_ADDR);
        self.memory
            .lock()
            .unwrap()
            .set_addr(IF_ADDR, if_register & !(1 << interrupt.get_register_bit()));
    }
    fn clear_ime(&self) {
        *self.ime.lock().unwrap() = false;
    }
    fn is_set_ime(&self) -> bool {
        *self.ime.lock().unwrap()
    }
}

#[cfg(test)]
mod interrupt_tests {
    use super::Memory;
    use super::{InterruptController, InterruptType, IE_ADDR, IF_ADDR};
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread;

    macro_rules! memory {
        ($($addr:expr=>$value:expr),*) => {
            {
                let mut mem = Memory::new();
                $(
                    mem.set_addr($addr, $value);
                )*
                mem
            }
        }
    }

    #[test]
    fn test_is_set_ime_true() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(true)),
        };
        assert_eq!(interrupt_controller.is_set_ime(), true);
    }
    #[test]
    fn test_is_set_ime_false() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(false)),
        };
        assert_eq!(interrupt_controller.is_set_ime(), false);
    }
    #[test]
    fn test_clear_ime_initially_set() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(true)),
        };
        interrupt_controller.clear_ime();
        assert_eq!(*interrupt_controller.ime.lock().unwrap(), false);
    }
    #[test]
    fn test_clear_ime_initially_clear() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(false)),
        };
        interrupt_controller.clear_ime();
        assert_eq!(*interrupt_controller.ime.lock().unwrap(), false);
    }
    #[test]
    fn test_disable_interrupt_request_vblank() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00000001))),
            ime: Arc::new(Mutex::new(false)),
        };
        interrupt_controller.disable_interrupt_request(InterruptType::VBLANK);
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_lcd_stat() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00000010))),
            ime: Arc::new(Mutex::new(false)),
        };
        interrupt_controller.disable_interrupt_request(InterruptType::LCD_STAT);
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_timer() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00000100))),
            ime: Arc::new(Mutex::new(false)),
        };
        interrupt_controller.disable_interrupt_request(InterruptType::TIMER);
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_serial() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00001000))),
            ime: Arc::new(Mutex::new(false)),
        };
        interrupt_controller.disable_interrupt_request(InterruptType::SERIAL);
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_joypad() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00010000))),
            ime: Arc::new(Mutex::new(false)),
        };
        interrupt_controller.disable_interrupt_request(InterruptType::JOYPAD);
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000000);
    }

    #[test]
    fn test_is_interrupt_enabled_true() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00011111))),
            ime: Arc::new(Mutex::new(false)),
        };
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::VBLANK),
            true
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::LCD_STAT),
            true
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::TIMER),
            true
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::SERIAL),
            true
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::JOYPAD),
            true
        );
    }
    #[test]
    fn test_is_interrupt_enabled_false() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b11100000))),
            ime: Arc::new(Mutex::new(false)),
        };
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::VBLANK),
            false
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::LCD_STAT),
            false
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::TIMER),
            false
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::SERIAL),
            false
        );
        assert_eq!(
            interrupt_controller.is_interrupt_enabled(InterruptType::JOYPAD),
            false
        );
    }
    #[test]
    fn test_get_interrupt_request_vblank_enable_set() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00011111,0xFF0F => 0b00011111),
            )),
            ime: Arc::new(Mutex::new(true)),
        };
        assert_eq!(
            interrupt_controller.get_interrupt_request(),
            Some(InterruptType::VBLANK)
        );
    }
    #[test]
    fn test_get_interrupt_request_vblank_enable_clear() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00000000,0xFF0F => 0b00000001),
            )),
            ime: Arc::new(Mutex::new(true)),
        };
        assert_eq!(interrupt_controller.get_interrupt_request(), None);
    }
    #[test]
    fn test_get_interrupt_request_timer_enable_set() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00000101,0xFF0F => 0b00011110),
            )),
            ime: Arc::new(Mutex::new(true)),
        };
        assert_eq!(
            interrupt_controller.get_interrupt_request(),
            Some(InterruptType::TIMER)
        );
    }
    #[test]
    fn test_execute_some() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00000101,0xFF0F => 0b00011110),
            )),
            ime: Arc::new(Mutex::new(true)),
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(
            (ime, interrupt_request_register, rst_addr),
            (false, 0b00011010, Some(0x50))
        );
    }
    #[test]
    fn test_execute_none_ime_clear() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00000000,0xFF0F => 0b00011111),
            )),
            ime: Arc::new(Mutex::new(false)),
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(
            (ime, interrupt_request_register, rst_addr),
            (false, 0b00011111, None)
        );
    }
    #[test]
    fn test_execute_none_no_enable() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00000000,0xFF0F => 0b00011111),
            )),
            ime: Arc::new(Mutex::new(true)),
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(
            (ime, interrupt_request_register, rst_addr),
            (true, 0b00011111, None)
        );
    }
    #[test]
    fn test_execute_none_no_request() {
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_controller = InterruptController {
            memory: Arc::new(Mutex::new(
                memory!(0xFFFF => 0b00011111,0xFF0F => 0b00000000),
            )),
            ime: Arc::new(Mutex::new(true)),
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller
            .memory
            .lock()
            .unwrap()
            .get_addr(IF_ADDR);
        assert_eq!(
            (ime, interrupt_request_register, rst_addr),
            (true, 0b00000000, None)
        );
    }
    #[test]
    fn test_request_handler_timer_initially_clear() {
        let memory = Arc::new(Mutex::new(memory!(0xFF0F => 0b00000000)));
        let memory_interrupt = Arc::clone(&memory);
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_handler = thread::spawn(move || {
            InterruptController::request_handler(memory_interrupt, rx);
        });
        tx.send(InterruptType::TIMER).unwrap();
        tx.send(InterruptType::EXIT).unwrap();
        interrupt_handler.join().unwrap();
        let interrupt_request_register = memory.lock().unwrap().get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000100);
    }
    #[test]
    fn test_request_handler_timer_initially_set() {
        let memory = Arc::new(Mutex::new(memory!(0xFF0F => 0b00000100)));
        let memory_interrupt = Arc::clone(&memory);
        let (tx, rx): (mpsc::Sender<InterruptType>, mpsc::Receiver<InterruptType>) =
            mpsc::channel();
        let interrupt_handler = thread::spawn(move || {
            InterruptController::request_handler(memory_interrupt, rx);
        });
        tx.send(InterruptType::TIMER).unwrap();
        tx.send(InterruptType::EXIT).unwrap();
        interrupt_handler.join().unwrap();
        let interrupt_request_register = memory.lock().unwrap().get_addr(IF_ADDR);
        assert_eq!(interrupt_request_register, 0b00000100);
    }
}
