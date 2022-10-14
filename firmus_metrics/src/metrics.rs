use crate::communication::{self, Communication};


pub fn initialize(){
    unsafe{
        if !communication::COMMS.initialized{
            communication::COMMS.initialize();
        }
    }
}