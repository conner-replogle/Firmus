use std::collections::HashMap;

use tokio::{net::TcpStream, io::BufReader};

use crate::FirmusError;


pub struct Parent{
    children: HashMap<String,TcpStream>,
    instructors: Vec<TcpStream>

}
impl Parent{
    pub fn new() -> Self{
        Self { children: HashMap::new(), instructors: Vec::new() }
    }
    pub fn add_children(&mut self,stream: TcpStream) -> Result<(),FirmusError>{
        
        tokio::spawn(async {
            let reader = BufReader::new(stream);
        

        });
        Ok(())
    }
}