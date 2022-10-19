use std::{collections::HashMap, io::BufReader};
use firmus_lib::{communication::{Hello, ConnectionType, instructor}, config::Config};
use tokio::{net::{TcpStream, TcpListener}};

use crate::{FirmusError, process::Process};



struct PID(String);
pub struct FrimusDaemon{
    processes: HashMap<PID,Process>,
}
impl FrimusDaemon{
    pub fn new() -> Self{
        Self { processes: HashMap::new()}
    }
    ///Starts the process from a config does not add the tcp stream
    pub fn start_process(&mut self,config: Config){
        //start process then append it to self.processeses

    }
    pub fn process_connection(&mut self,stream: TcpStream){
        //get pid from stream then find the process in self.processes and add it to it

    }
}