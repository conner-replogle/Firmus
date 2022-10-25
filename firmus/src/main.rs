
use std::{process::Command, time::Duration, thread, net::{TcpStream, SocketAddr}, str::FromStr, path::PathBuf};
use clap::{Parser, Subcommand};
use firmus_lib::{communication::{stream_wrapper::Stream, ConnectionType, base::BaseResponse, instructor}, config::Config};

#[derive(Parser, Debug)]
#[command(name = "Firmus")]
#[command(author = "Conner Replogle <connerlreplogle@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Manages Projects", long_about = None)]
struct Args {
    ///override the daemon port if not specified FIRMUS_PORT env var is used
    #[arg(short = 'p')]
    port: Option<String>,

    /// which command to run on the daemon
    #[command(subcommand)]
    command: Actions,
}

#[derive(Subcommand,Debug,Clone)]
enum Actions {
    /// starts a program with syntax firmus start -e {executable_path}
    Start {
        /// Path to a normal executable to run
        #[arg(long)]
        #[arg(short='e')]
        executable_path: String,
        /// option to open a connection to program logs after starting default to true
        #[arg(long)]
        #[arg(short='c')]
        #[arg(default_value_t = true)]
        open_connection: bool,
    },
    /// Lists all running programs
    List,
}
fn main() {
    let args = Args::parse();
    let port = args.port.unwrap_or(std::env::var("FIRMUS_PORT").unwrap_or("8888".to_string()));
    let addr: SocketAddr = SocketAddr::from_str(&format!("127.0.0.1:{port}")).unwrap();
    println!("Connecting on addr {addr}");
    if let Ok(connection) = TcpStream::connect(addr){
        let mut stream = Stream::from(connection);
        stream.write(ConnectionType::Instructor).unwrap();
        assert_eq!(stream.read::<BaseResponse>().unwrap(),BaseResponse::Ok);
        match args.command{
            Actions::Start { executable_path, open_connection } => {
                stream.write(instructor::Command::Start(Config{
                    run_command: executable_path,
                    ..Default::default()
    
            })).unwrap();
            },
            Actions::List => todo!(),
        }
        

        
    }
}