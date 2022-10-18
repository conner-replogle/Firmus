use std::{collections::HashMap, io::BufReader};

use tokio::{net::{TcpStream, TcpListener}};

use crate::FirmusError;


pub struct Parent{
    children: HashMap<String,TcpStream>,
    instructors: Vec<TcpStream>,
    listener: Option<TcpListener>
}
impl Parent{
    pub fn new() -> Self{
        Self { children: HashMap::new(), instructors: Vec::new() ,listener: None }
    }
    pub async fn listen(mut self,ip: &str)-> Result<(),FirmusError>{
        self.listener = Some(TcpListener::bind(ip).await?);

        tokio::spawn(async move {
            while let Ok((socket,address)) = self.listener.as_ref().unwrap().accept().await{
                self.handle_connection(socket);
                
            }
        });
        Ok(())
    }
    pub async fn handle_connection(&mut self,stream: TcpStream) -> Result<(),FirmusError>{
        let reader = BufReader::new(stream.into_std().unwrap());
        let hello: 
        Ok(())
    }
}