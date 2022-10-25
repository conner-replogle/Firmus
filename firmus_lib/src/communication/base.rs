use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub enum BaseResponse{
    Ok,
    Heartbeat,
    Ping,
    Pong,
    Error(u16,String)
}