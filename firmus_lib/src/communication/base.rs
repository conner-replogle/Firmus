use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize)]
pub enum BaseResponse{
    Ok,
    Heartbeat,
    Ping,
    Pong,
}