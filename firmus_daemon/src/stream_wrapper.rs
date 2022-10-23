use std::{net::TcpStream, io::Write};

use firmus_lib::communication::base::{self, BaseResponse};



pub struct Stream{
    pub inner: TcpStream
}
impl Stream{
    pub fn from(stream: TcpStream) -> Self{
        Self{
            inner: stream
        }
    }

    pub fn read<T: for<'de> serde::de::Deserialize<'de>>(&mut self) -> Result<T,bincode::Error>{
        bincode::deserialize_from(&mut self.inner)
    }
    pub fn write<T:serde::Serialize>(&mut self,value: T) -> Result<(),bincode::Error>{
        let bytes = bincode::serialize(&value)?;
        self.inner.write(&bytes);
        return Ok(());
    }
    pub fn send_base(&mut self,msg:BaseResponse)-> Result<(),bincode::Error>{
        let bytes = bincode::serialize(&msg)?;
        self.inner.write(&bytes);
        return Ok(());
    }
}