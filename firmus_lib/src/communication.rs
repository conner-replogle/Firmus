
///first communication between any endpoint and the firmus_daemon.
pub struct Hello{
    connection_type: Type,
    
}

pub enum ConnectionType {
    Program,
    Firmus
}