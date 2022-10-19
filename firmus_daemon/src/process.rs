use std::net::TcpStream;

use firmus_lib::config::Config;
use tokio::{process::Command, task::JoinHandle};

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
            let stdout = command.stdout.take().unwrap();
            let stderr = command.stderr.take().unwrap();
            
        }));
        
        Ok(())
    }

    pub fn add_connection(&mut self){

    }
}