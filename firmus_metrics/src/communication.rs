use std::{env, sync::atomic::AtomicUsize};

use ipmpsc::{Sender, SharedRingBuffer};
use serde::Serialize;

pub static mut COMMS: Communication = Communication::new();



pub struct Communication {
    sender: Option<Sender>,
    pub initialized: bool
}
impl Communication {
    const fn new() -> Self{
        Self{sender: None, initialized: false }
    }
    pub fn initialize(&mut self) {
        if let None = self.sender{
            let mut args = env::args();
            let arg = args.find(|a|{
                return a.contains("firmus_shared_mem_path=");

            }).unwrap();
            let mut split = arg.split("=");
            split.next();
            let path = split.next().unwrap();
            println!("Shared Mem Path {}",path);
            self.sender = Some(Sender::new(SharedRingBuffer::open(path).unwrap()));
            self.initialized = true;
        }else{
            panic!("Can't intialize comms more than once");
        }

    }
    pub fn send(&self,message: &impl Serialize){
        if let Some(sender) = &self.sender{
            sender.send(&message).unwrap();
        }

    }
}
