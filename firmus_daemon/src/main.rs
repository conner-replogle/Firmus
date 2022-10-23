mod process;
mod stream_wrapper;
use std::{env, task::Poll, net::{TcpListener, SocketAddr, TcpStream}, thread, sync::{Mutex, Arc, RwLock, PoisonError}, str::FromStr, collections::HashMap};
use arc_swap::ArcSwap;
use clap::Parser;
use firmus_lib::communication::{ConnectionType, instructor::{self, Command}};
use process::Process;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use stream_wrapper::Stream;

use std::sync::mpsc;

type ThreadSafeProcess = Arc<Mutex<HashMap<String,Process>>>;

#[derive(thiserror::Error,Debug)]
enum FirmusError{
    #[error("TcpListener fucked up with this error {0}")]
    Disconnect(#[from] std::io::Error),
    #[error("Failed to parse a message: {0}")]
    FailedParsingCommunication(#[from] bincode::Error),


}
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
struct Procceses(Vec<Process>);

impl Serialize for Procceses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // If you implement `Deref`, then you don't need to add `.0`
        let s = format!("{}", self.0.format(SERIALIZE_FORMAT));
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Procceses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        
    }
}

fn main() -> Result<(),FirmusError>{
    let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:8888").unwrap();
    let listener = TcpListener::bind(addr).unwrap();

    let mut thread_safe_processes: ThreadSafeProcess = Arc::from(Mutex::from(HashMap::new()));


    for stream in listener.incoming() {
        
        match stream {
            Err(_) => println!("listen error"),
            Ok(stream) => {
                let mut stream = Stream::from(stream);
                println!("connection from {} to {}",
                        stream.inner.peer_addr().unwrap(),
                        stream.inner.local_addr().unwrap());
                let connection_type: ConnectionType = stream.read()?;
                match connection_type{
                    ConnectionType::Program => {
                        let thread_safe_processes = thread_safe_processes.clone();
                        thread::spawn(move ||{
                            handle_program(stream,thread_safe_processes).unwrap();
                        });
                    },
                    ConnectionType::Instructor => {
                        let thread_safe_processes = thread_safe_processes.clone();
                        thread::spawn(move ||{
                            handle_instructor(stream,thread_safe_processes).unwrap();
                        });

                    },
                }
            }
        }
    }
    Ok(())
}
fn handle_program(mut stream: Stream,thread_safe_processes:ThreadSafeProcess) -> Result<(),FirmusError>{
    let mut processes = thread_safe_processes.lock().unwrap();
    let id: String = stream.read()?;
    if let Some(process) = processes.get_mut(&id){
        process.stream = Some(stream);

    }
    Ok(())
}
fn handle_instructor(mut stream: Stream,thread_safe_processes:ThreadSafeProcess) -> Result<(),FirmusError>{
    let command: Command = stream.read()?;
    match command{
        Command::Start(config) => {
            
        },
        Command::Stop(id) => {
            
        },
        Command::List =>{
            let processes = thread_safe_processes.lock().unwrap();
            //stream.write(processes.values());


        },
    }
    Ok(())
}