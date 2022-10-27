use std::{process::{Command, Child, Stdio}, thread, io};

use firmus_lib::{config::Config, communication::stream_wrapper::Stream};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProcessStatus {
    id: u32,
    ipc_online: bool,
    process_online: bool,
    pid: u32,
}

#[derive(thiserror::Error, Debug)]
pub enum ProcessError {
    #[error("Process failed to start: {0}")]
    FailedToStart(#[from] std::io::Error),
}

pub struct Process {
    pub config: Config,
    pub child: Option<Child>,
    pub stream: Option<Stream>,
}
impl Process {
    pub fn from(config: Config) -> Self {
        Self {
            stream: None,
            config,
            child: None
        }
    }
    pub fn start(&mut self) -> Result<u32,ProcessError> {
        let mut args = self.config.run_command.split(" ");
        let main = args.next().unwrap();
        let mut command = Command::new(main);
        command.args(args);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        let child = command.spawn()?;
        let pid = child.id();
        self.child = Some(child);

        Ok(pid)
    }
    pub fn into_status(&self) -> ProcessStatus {
        ProcessStatus {
            id: 0,
            ipc_online: self.stream.is_some(),
            process_online: self.child.is_some(),
            pid: self.child.as_ref().map_or(0, |a|{a.id()}),
        }
    }
}
