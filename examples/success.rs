use std::env::args;
use std::thread::sleep;
use std::time::Duration;
use std::str::FromStr;

fn main() {
    if let Some(arg) = args().nth(1) {
        sleep(Duration::from_secs(u64::from_str(&arg).expect("Invalid argument time")));
    }
}
