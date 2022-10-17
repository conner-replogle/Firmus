use std::{time::Duration, thread};

use firmus_metrics::metrics::{self};
//static PLAYER_COUNT: Counter = Counter::init(20);
fn main() {
    metrics::initialize();
    //PLAYER_COUNT.inc()
    loop{
        print!("Helo");
        thread::sleep(Duration::from_secs(1));
    }
    
}
