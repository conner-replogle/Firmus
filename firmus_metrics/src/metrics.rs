use std::sync::{Mutex, Arc};

use serde::Serialize;

use crate::communication::{self, Communication, COMMS};

// pub struct Counter{
//     value: Arc<Mutex<i64>>,
// }
// impl Counter{
//     pub const fn init(value: i64)-> Self{
//         Self { value: Arc::from(Mutex::from(value)) }
//     }
//     pub fn set(&self,value: i64){
//         let mut num = self.value.lock().unwrap();
//         *num = value;
//     }
//     pub fn inc(&self){
//         let mut num = self.value.lock().unwrap();
//         *num = *num+1;
//     }
//     pub fn dec(&self){
//         let mut num = self.value.lock().unwrap();
//         *num = *num-1;
//     }
//     pub fn report(&self){
        
//     }
// }

pub fn initialize(){
    unsafe{
        if !communication::COMMS.initialized{
            communication::COMMS.initialize();
        }
    }
}
// pub fn send(metric: metric){
//     unsafe{
//         if !communication::COMMS.initialized{
//             communication::COMMS.send(&metric)
//         }
//     }
// }

// struct MetricData{
//     pub display_type: 
// }
