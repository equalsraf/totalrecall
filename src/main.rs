extern crate time;

use std::process::{Command, exit, ExitStatus};
use std::env;
use time::PreciseTime;
use std::thread::sleep;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt ;

#[cfg(unix)]
fn signal(status: &ExitStatus) -> Option<i32> { status.signal() }
#[cfg(not(unix))]
fn signal(_: &ExitStatus) -> Option<i32> { None }

fn main() {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() < 2 {
        println!("Usage: totalrecall <cmd ...>");
        exit(-1);
    }

    loop {
        let start = PreciseTime::now();
        match Command::new(&arguments[1])
                .args(&arguments[2..])
                .status() {
            Err(err) => {
                println!("Error running {:?}: {}", &arguments, err);
                exit(-1);
            },
            Ok(status) if status.success() => exit(0),
            Ok(status) => {
                if let Some(signum) = signal(&status) {
                    println!("totalrecall: {} exited with signal({}), exiting",
                            arguments[1], signum);
                    exit(-1);
                }

                if let Some(code) = status.code() {
                    if let Ok(elapsed) = start.to(PreciseTime::now()).to_std() {
		        if elapsed.as_secs() < 2 {
                            sleep(Duration::from_secs(1))
		        }
                        println!("totalrecall: After {}s {} exited with code({}), restarting",
                                elapsed.as_secs(),
                                arguments[1], code);
                    } else {
                        println!("totalrecall: {} exited with code({}), restarting",
                                arguments[1], code);
                    }
                } else {
                    println!("totalrecall: {} terminated", arguments[1]);
                    exit(-1);
                }
            }
        };
    }
}
