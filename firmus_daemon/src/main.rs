mod parent;
use std::{env, task::Poll};
use clap::Parser;
use ipmpsc::{Sender, SharedRingBuffer, Receiver};
use parent::Parent;
use tokio::{process::Command, io::{AsyncReadExt, BufReader, AsyncBufReadExt, AsyncWriteExt}, net::TcpListener};



#[derive(thiserror::Error,Debug)]
enum FirmusError{
    #[error("TcpListener fucked up with this error {0}")]
    Disconnect(#[from] std::io::Error),

}

#[tokio::main]
async fn main() -> Result<(),FirmusError>{
    let port = env::var("FIRMUS_PORT").unwrap_or("8080".to_string());
    let mut parent = Parent::new();
    let listener = TcpListener::bind(format!("127.0.0.1{port}")).await?;

    tokio::spawn(async move {
        while let Ok((socket,address)) = listener.accept().await{
            match parent.add_children(socket){
                _ => {}
                Err(err) => {}
            }
            
        }
    });
    
    Ok(())
}
