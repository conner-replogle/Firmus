
use std::process::Command;


fn main() {
    let mut args = std::env::args();
    args.next();
    Command::new("firmus_daemon").args(args).spawn().unwrap();
}
