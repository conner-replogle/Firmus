
use std::{process::Command, time::Duration, thread};

use tokio::task::JoinHandle;
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Executable Path to application to run
//     #[arg(short, long)]
//     executable_path: String,

//     /// Number of times to greet
//     #[arg(short, long, default_value_t = true)]
//     ipc: bool,
// }
#[tokio::main]
async fn main() {
    let a = count();
    loop{
        println!("H");
        thread::sleep(Duration::from_secs(1));
    }
    a.await;
}


async fn count() -> JoinHandle<()>{
    tokio::spawn(async {
        let mut num = 0;
        loop{
            println!(
                "number {num}"
            );
            num+=1
        }
    })
    
}
