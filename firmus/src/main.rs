
use std::process::Command;
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
    let mut args = std::env::args();
    args.next();
    Command::new("firmus_daemon").args(args).spawn().unwrap();
}
