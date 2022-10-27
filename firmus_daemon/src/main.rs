mod process;
mod dash_connection;
use arc_swap::ArcSwap;
use clap::Parser;
use dash_connection::DashConnection;
use firmus_lib::communication::{
    instructor::{self, Command},
    ConnectionType, stream_wrapper::Stream, base::BaseResponse,
};
use process::{Process, ProcessStatus};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    env,
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex, PoisonError, RwLock},
    task::Poll,
    thread, io::Read,
};


use std::sync::mpsc;

type ThreadSafeProcess = Arc<Mutex<HashMap<u32, Arc<Mutex<Process>>>>>;

#[derive(thiserror::Error, Debug)]
enum FirmusError {
    #[error("TcpListener fucked up with this error {0}")]
    Disconnect(#[from] std::io::Error),
    #[error("Failed to parse a message: {0}")]
    FailedParsingCommunication(#[from] bincode::Error),
}

fn main() -> Result<(), FirmusError> {
    let port = env::var("FIRMUS_PORT").unwrap_or("8888".to_string());
    let addr: SocketAddr = SocketAddr::from_str(&format!("127.0.0.1:{port}")).unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on addr {addr}");
    let mut dash_connection = DashConnection::new();
    let thread_safe_processes: ThreadSafeProcess = Arc::from(Mutex::from(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Err(_) => println!("listen error"),
            Ok(stream) => {
                let mut stream = Stream::from(stream);
                println!(
                    "connection from {} to {}",
                    stream.inner.peer_addr().unwrap(),
                    stream.inner.local_addr().unwrap()
                );
                let connection_type: ConnectionType = stream.read()?;
                match connection_type {
                    ConnectionType::Program => {
                        stream.send_base(BaseResponse::Ok).unwrap();
                        println!("Stream identified as program");
                        let thread_safe_processes = thread_safe_processes.clone();
                        thread::spawn(move || {
                            handle_program(stream, thread_safe_processes).unwrap();
                        });
                    }
                    ConnectionType::Instructor => {
                        stream.send_base(BaseResponse::Ok).unwrap();
                        println!("Stream identified as Instructor");
                        

                        let thread_safe_processes = thread_safe_processes.clone();
                        thread::spawn(move || {
                            handle_instructor(stream, thread_safe_processes).unwrap();
                        });
                    }
                }
            }
        }
    }
    Ok(())
}
fn handle_program(
    mut stream: Stream,
    thread_safe_processes: ThreadSafeProcess,
) -> Result<(), FirmusError> {
    let mut processes = thread_safe_processes.lock().unwrap();
    let id: u32 = stream.read()?;
    if let Some(process) = processes.get_mut(&id) {
        let mut process = process.lock().unwrap();
        process.stream = Some(stream);
    }
    Ok(())
}
fn handle_instructor(
    mut stream: Stream,
    thread_safe_processes: ThreadSafeProcess,
) -> Result<(), FirmusError> {
    let command: Command = stream.read()?;
    match command {
        Command::Start(config) => {
            start_program(stream, thread_safe_processes, config);
        }
        Command::Stop(id) => {}
        Command::List => {
            let processes = thread_safe_processes.lock().unwrap();
            stream.write(processes.values().zip(processes.keys()).map(|(v,k)|{(*k,v.lock().unwrap().into_status())}).collect::<Vec<(u32,ProcessStatus)>>()).unwrap();
        }
    }
    Ok(())
}

fn start_program(
    mut stream: Stream,
    thread_safe_processes: ThreadSafeProcess,
    config: firmus_lib::config::Config,
){
    let mut process = Process::from(config);
    println!("Running command {}",process.config.run_command);
    if let Ok(pid) = process.start(){
        println!("Process Started with pid:{pid}");
        let mut proccesses = thread_safe_processes.lock().unwrap();
        proccesses.insert(pid,Arc::from(Mutex::from(process)));
        let process = proccesses.get_mut(&pid).unwrap();
        let mut p = process.lock().unwrap();
        let child = p.child.as_mut().unwrap();
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        drop(child);drop(p);drop(process);drop(proccesses);
        thread::scope(|s|{
            s.spawn(||{
                loop{
                    let mut buf = Vec::new();
                    if stdout.read(&mut buf).unwrap() > 0{
                        println!("stdout:{}",String::from_utf8(buf).unwrap())
                    }
                    
                }
            });

        });
        thread::scope(|s|{
            s.spawn(||{
                loop{
                    let mut buf = Vec::new();
                    if stderr.read(&mut buf).unwrap() > 0{
                        println!("stderr:{}",String::from_utf8(buf).unwrap())
                    }
                    
                }
            });

        });
        loop{
            
        }



    }else{
        println!("failed to start program");
        stream.send_base(BaseResponse::Error(1, "Program failed to start".to_string())).unwrap();
    }
    
}
