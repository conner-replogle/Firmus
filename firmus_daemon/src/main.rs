
use clap::Parser;
use ipmpsc::{Sender, SharedRingBuffer, Receiver};
use tokio::{process::Command, io::{AsyncReadExt, BufReader, AsyncBufReadExt}};
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
#[tokio::main]
async fn main() {
    let parsed_args = Args::parse();
    let executable_path = parsed_args.executable_path;
    
    
    if parsed_args.ipc {
        let rx = Receiver::new(SharedRingBuffer::create("test_reciever",4092*5).unwrap());
        let mut child = Command::new(executable_path).arg("-firmus_shared_mem_path=test_reciever").spawn().unwrap();
        
        let stderr = child.stderr.take()
            .expect("child did not have a handle to stdout");
    
        let mut stderr_reader = BufReader::new(stderr).lines();
    
        tokio::spawn(async {
            let status = child.wait_with_output().await.unwrap();
            println!("child status was: {}", status.status);
        });
        tokio::spawn(async move {
            while let Some(line) = stderr_reader.next_line().await.unwrap() {
                println!("Stderr line: {}", line);
            }
        });
        loop{}
    
        
    }
}
