pub mod instructor;
pub mod program;
///first communication between any endpoint and the firmus_daemon.

#[derive(serde::Serialize,serde::Deserialize)]
pub struct Hello{
    pub connection_type: ConnectionType,
    
}
#[derive(serde::Serialize,serde::Deserialize)]
pub enum ConnectionType {
    Program,
    Instructor
}
