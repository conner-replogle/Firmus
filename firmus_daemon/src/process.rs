use std::net::TcpStream;

use firmus_lib::config::Config;
use tokio::{process::Command, task::JoinHandle, io::BufReader};
use tokio::io::{ AsyncBufReadExt};
use crate::{FirmusError, dash_connection::DashConnection};
enum ProcessError{

}

pub struct Process{
    connection: Option<TcpStream>,
    handle: Option<JoinHandle<()>>,
    config: Config,
}
impl Process{
    pub fn new(config: Config) -> Self{
        Self { connection: None, config,handle: None }
    }
    pub fn start(&self) -> Result<(),ProcessError>{
        let start_command_split = self.config.run_command.split(" ");
        let first_command = start_command_split.next().unwrap();//TODO handle this error
        let command = Command::new(first_command).args(start_command_split).current_dir(self.config.directory_path).spawn().unwrap();
        let dash_connection: Option<DashConnection> = if let Some(connection) = self.config.metrics{
            if let Ok(valid_connection) = DashConnection::new(connection.url){
                Some(valid_connection)
            }else{
                None
            }
        }else{
            None
        };
        self.handle = Some(tokio::spawn(async {
            let stdout = BufReader::new(command.stdout.take().unwrap()).lines();
            let stderr = BufReader::new(command.stderr.take().unwrap()).lines();
            tokio::spawn(async move {
                while let Some(line) = stdout.next_line().await.unwrap() {
                    if let Some(dash) = dash_connection{
                        dash.send(line);
                    }
                }
            });
            
        }));
        
        Ok(())
    }

    pub fn add_connection(&mut self){

    }
}