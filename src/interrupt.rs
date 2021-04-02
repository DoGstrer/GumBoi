use std::sync::{Mutex,Arc};
use std::sync::mpsc;
use std::thread;
use super::Memory;

// REVIEW InterruptType better API ?
#[derive(Copy,Clone,PartialEq,Debug,Eq)]
pub enum InterruptType{
    VBLANK(u8,u16),
    LCD_STAT(u8,u16),
    TIMER(u8,u16),
    SERIAL(u8,u16),
    JOYPAD(u8,u16),
    EXIT,
}

pub struct Interrupt;
impl Interrupt {
    
    pub const IE_ADDR:u16 = 0xFFFF;
    pub const IF_ADDR:u16 = 0xFF0F;
    pub const VBLANK: InterruptType = InterruptType::VBLANK(0,0x40);
    pub const LCD_STAT: InterruptType = InterruptType::LCD_STAT(1,0x48);
    pub const TIMER: InterruptType = InterruptType::TIMER(2,0x50);
    pub const SERIAL: InterruptType = InterruptType::SERIAL(3,0x58);
    pub const JOYPAD: InterruptType = InterruptType::JOYPAD(4,0x60);

    fn get_register_bit(interrupt: InterruptType) -> u8{
        match interrupt{
            InterruptType::VBLANK(bit,addr) => bit,
            InterruptType::LCD_STAT(bit,addr) => bit,
            InterruptType::TIMER(bit,addr) => bit,
            InterruptType::SERIAL(bit,addr) => bit,
            InterruptType::JOYPAD(bit,addr) => bit,
            _ => panic!("Invalid Interrupt")
        }
    }
    fn get_rst_addr(interrupt: InterruptType) -> u16 {
        match interrupt{
            InterruptType::VBLANK(bit,addr) => addr,
            InterruptType::LCD_STAT(bit,addr) => addr,
            InterruptType::TIMER(bit,addr) => addr,
            InterruptType::SERIAL(bit,addr) => addr,
            InterruptType::JOYPAD(bit,addr) => addr,
            _ => panic!("Invalid Interrupt")
        }
    }
    fn get_interrupt_from_bit(bit: u8) -> InterruptType {
        match bit{
            0 => Interrupt::VBLANK,
            1 => Interrupt::LCD_STAT,
            2 => Interrupt::TIMER,
            3 => Interrupt::SERIAL,
            4 => Interrupt::JOYPAD,
            _ => panic!("Invalid Interrupt bit"),
        }
    }
}

pub struct InterruptController{
    memory: Arc<Mutex<Memory>>,
    ime: Arc<Mutex<bool>>,
}

impl InterruptController{
    pub fn new(memory: Arc<Mutex<Memory>>,ime: Arc<Mutex<bool>>, interrupt_rx: mpsc::Receiver<InterruptType>) -> InterruptController {
        let memory_interrupt = Arc::clone(&memory);
        thread::spawn(move || {InterruptController::request_handler(memory_interrupt,interrupt_rx);});
        return InterruptController{
            memory: memory,
            ime: ime,
        };
    }
    pub fn request_handler(memory: Arc<Mutex<Memory>>,interrupt_rx: mpsc::Receiver<InterruptType>){
        for interrupt in interrupt_rx {
            match interrupt{
                InterruptType::EXIT => return,
                Interrupt::VBLANK | Interrupt::TIMER | Interrupt::SERIAL | Interrupt::LCD_STAT | Interrupt::JOYPAD => {
                    let if_register = memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
                    memory.lock().unwrap().set_addr(Interrupt::IF_ADDR,if_register | 1 << Interrupt::get_register_bit(interrupt));
                },
                _ => panic!("Invalid Interrupt Raised"),
            }
        }
    }
    pub fn execute(&self) -> Option<u16>{
        match self.get_interrupt_request() {
            Some(interrupt) => {
                self.clear_ime();
                self.disable_interrupt_request(interrupt);
                Some(Interrupt::get_rst_addr(interrupt))
            },
            None => None,
        }
    }
    fn get_interrupt_request(&self) -> Option<InterruptType>{
        let interrupt_requests_register = self.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        let interrupt_enable_register = self.memory.lock().unwrap().get_addr(Interrupt::IE_ADDR);
        if self.is_set_ime(){
            for i in 0..5 {
                if (interrupt_requests_register >> i & 1 == 1) && (interrupt_enable_register >> i & 1 == 1) {
                    return Some(Interrupt::get_interrupt_from_bit(i));
                }
            }
        }
        None
    }
    fn is_interrupt_enabled(&self,interrupt: InterruptType) -> bool {
        let ie_register = self.memory.lock().unwrap().get_addr(Interrupt::IE_ADDR);
        match (ie_register >> Interrupt::get_register_bit(interrupt) & 0x01) {
            0 => false,
            1 => true,
            _ => panic!("Invalid Register Value")
        } 
    }
    fn disable_interrupt_request(&self,interrupt: InterruptType){
        let if_register = self.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        self.memory.lock().unwrap().set_addr(Interrupt::IF_ADDR,if_register & !(1 << Interrupt::get_register_bit(interrupt)));
    }

    fn clear_ime(&self){
        *self.ime.lock().unwrap() = false;
    }
    fn is_set_ime(&self) -> bool{
        *self.ime.lock().unwrap()
    }
}

#[cfg(test)]
mod interrupt_tests{
    use super::{Interrupt,InterruptController,InterruptType};
    use super::Memory;
    use std::sync::{Arc,Mutex};
    use std::sync::mpsc;
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
    fn test_is_set_ime_true(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(true)),
            
        };
        assert_eq!(interrupt_controller.is_set_ime(),true);
    }
    #[test]
    fn test_is_set_ime_false(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(false)),
            
        };
        assert_eq!(interrupt_controller.is_set_ime(),false);
    }
    #[test]
    fn test_clear_ime_initially_set(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(true)),
            
        };
        interrupt_controller.clear_ime();
        assert_eq!(*interrupt_controller.ime.lock().unwrap(),false);
    } 
    #[test]
    fn test_clear_ime_initially_clear(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(Memory::new())),
            ime: Arc::new(Mutex::new(false)),
            
        };
        interrupt_controller.clear_ime();
        assert_eq!(*interrupt_controller.ime.lock().unwrap(),false);
    }
    #[test]
    fn test_disable_interrupt_request_vblank(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00000001))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        interrupt_controller.disable_interrupt_request(Interrupt::VBLANK);
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_lcd_stat(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00000010))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        interrupt_controller.disable_interrupt_request(Interrupt::LCD_STAT);
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_timer(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00000100))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        interrupt_controller.disable_interrupt_request(Interrupt::TIMER);
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_serial(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00001000))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        interrupt_controller.disable_interrupt_request(Interrupt::SERIAL);
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000000);
    }
    #[test]
    fn test_disable_interrupt_request_joypad(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFF0F => 0b00010000))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        interrupt_controller.disable_interrupt_request(Interrupt::JOYPAD);
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000000);
    }

    #[test]
    fn test_is_interrupt_enabled_true(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00011111))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::VBLANK),true);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::LCD_STAT),true);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::TIMER),true);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::SERIAL),true);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::JOYPAD),true);
    }
    #[test]
    fn test_is_interrupt_enabled_false(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b11100000))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::VBLANK),false);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::LCD_STAT),false);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::TIMER),false);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::SERIAL),false);
        assert_eq!(interrupt_controller.is_interrupt_enabled(Interrupt::JOYPAD),false);
    }
    #[test]
    fn test_get_interrupt_request_vblank_enable_set(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00011111,0xFF0F => 0b00011111))),
            ime: Arc::new(Mutex::new(true)),
            
        };
        assert_eq!(interrupt_controller.get_interrupt_request(),Some(Interrupt::VBLANK));
    }
    #[test]
    fn test_get_interrupt_request_vblank_enable_clear(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00000000,0xFF0F => 0b00000001))),
            ime: Arc::new(Mutex::new(true)),
            
        };
        assert_eq!(interrupt_controller.get_interrupt_request(),None);
    }
    #[test]
    fn test_get_interrupt_request_timer_enable_set(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00000101,0xFF0F => 0b00011110))),
            ime: Arc::new(Mutex::new(true)),       
        };
        assert_eq!(interrupt_controller.get_interrupt_request(),Some(Interrupt::TIMER));
    }
    #[test]
    fn test_execute_some(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00000101,0xFF0F => 0b00011110))),
            ime: Arc::new(Mutex::new(true)),     
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!((ime,interrupt_request_register,rst_addr),(false,0b00011010,Some(0x50)));
    }
    #[test]
    fn test_execute_none_ime_clear(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00000000,0xFF0F => 0b00011111))),
            ime: Arc::new(Mutex::new(false)),
            
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!((ime,interrupt_request_register,rst_addr),(false,0b00011111,None));
    }
    #[test]
    fn test_execute_none_no_enable(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00000000,0xFF0F => 0b00011111))),
            ime: Arc::new(Mutex::new(true)),
            
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!((ime,interrupt_request_register,rst_addr),(true,0b00011111,None));
    }
    #[test]
    fn test_execute_none_no_request(){
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_controller = InterruptController{
            memory: Arc::new(Mutex::new(memory!(0xFFFF => 0b00011111,0xFF0F => 0b00000000))),
            ime: Arc::new(Mutex::new(true)),
            
        };
        let rst_addr = interrupt_controller.execute();
        let ime: bool = *interrupt_controller.ime.lock().unwrap();
        let interrupt_request_register = interrupt_controller.memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!((ime,interrupt_request_register,rst_addr),(true,0b00000000,None));
    }
    # [test]
    fn test_request_handler_timer_initially_clear(){
        let memory = Arc::new(Mutex::new(memory!(0xFF0F => 0b00000000)));
        let memory_interrupt = Arc::clone(&memory);
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_handler = thread::spawn(move || {
            InterruptController::request_handler(memory_interrupt,rx);
        });
        tx.send(Interrupt::TIMER).unwrap();
        tx.send(InterruptType::EXIT).unwrap();
        interrupt_handler.join().unwrap();
        let interrupt_request_register = memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000100);
    }
    # [test]
    fn test_request_handler_timer_initially_set(){
        let memory = Arc::new(Mutex::new(memory!(0xFF0F => 0b00000100)));
        let memory_interrupt = Arc::clone(&memory);
        let (tx,rx):(mpsc::Sender<InterruptType>,mpsc::Receiver<InterruptType>) = mpsc::channel();
        let interrupt_handler = thread::spawn(move || {
            InterruptController::request_handler(memory_interrupt,rx);
        });
        tx.send(Interrupt::TIMER).unwrap();
        tx.send(InterruptType::EXIT).unwrap();
        interrupt_handler.join().unwrap();
        let interrupt_request_register = memory.lock().unwrap().get_addr(Interrupt::IF_ADDR);
        assert_eq!(interrupt_request_register,0b00000100);
    }
}