mod firmus_daemon;
mod process;
mod dash_connection;
use std::{env, task::Poll};
use clap::Parser;
use firmus_lib::communication::{ConnectionType, instructor};
use ipmpsc::{Sender, SharedRingBuffer, Receiver};
use firmus_daemon::FrimusDaemon;

use tokio::{process::Command, io::{AsyncReadExt, BufReader, AsyncBufReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};



#[derive(thiserror::Error,Debug)]
enum FirmusError{
    #[error("TcpListener fucked up with this error {0}")]
    Disconnect(#[from] std::io::Error),

}

#[tokio::main]
async fn main() -> Result<(),FirmusError>{
    let port = env::var("FIRMUS_PORT").unwrap_or("8080".to_string());
    let daemon = FrimusDaemon::new();
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    tokio::spawn(async move {
        while let Ok((stream,address)) = listener.accept().await{
            handle_connection(stream);                
        }
    });

    
    Ok(())
}

async fn handle_connection(stream: TcpStream)-> Result<(),FirmusError>{
    let mut reader = std::io::BufReader::new(stream.into_std().unwrap());
    let connection_type: ConnectionType = serde_json::from_reader(&mut reader).unwrap();
    match connection_type{
        firmus_lib::communication::ConnectionType::Program => {
            
        },
        firmus_lib::communication::ConnectionType::Instructor => {
            let command: instructor::Command = serde_json::from_reader(&mut reader).unwrap();
            handle_instructor_command(command);

        },
    }
    Ok(())
}
