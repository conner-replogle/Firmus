use std::process::{Command, self};

use clap::Parser;
use ipmpsc::{Sender, SharedRingBuffer, Receiver};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Executable Path to application to run
    #[arg(short, long)]
    executable_path: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = true)]
    ipc: bool,
}

fn main() {
    let parsed_args = Args::parse();
    let executable_path = parsed_args.executable_path;
    
    
    if parsed_args.ipc {
        let rx = Receiver::new(SharedRingBuffer::create("test_reciever",4092*5).unwrap());
        Command::new(executable_path).arg("test").arg("--").arg("firmus_shared_mem_path=test_reciever").spawn().unwrap();
        loop{
            match rx.recv::<String>(){
                Ok(msg) => {
                    println!("{msg}")
                },
                Err(err) => {
                    println!("error: {:?}", err)
                },
            }
        }
    }
}
