
use std::process::{Command, exit, ExitStatus};
use std::env;
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt ;

#[cfg(unix)]
fn signal(status: &ExitStatus) -> Option<i32> { status.signal() }
#[cfg(not(unix))]
fn signal(status: &ExitStatus) -> Option<i32> { None }

fn main() {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() < 2 {
        println!("Usage: totalrecall <cmd ...>");
        exit(-1);
    }

    loop {
        let start = SystemTime::now();
        match Command::new(&arguments[1])
                .args(&arguments[2..])
                .status() {
            Err(err) => {
                println!("Error running {:?}: {}", &arguments, err);
                exit(-1);
            },
            Ok(status) if status.success() => exit(0),
            Ok(status) => {
                if let Ok(elapsed) = start.elapsed() {
                    println!("totalrecall: After {}s", elapsed.as_secs());
                }

                if let Some(signum) = signal(&status) {
                    println!("totalrecall: {} exited with signal({}), exiting",
                            arguments[1], signum);
                    exit(-1);
                }

                if let Some(code) = status.code() {
                    println!("totalrecall: {} exited with code({}), restarting",
                            arguments[1], code);
                } else {
                    println!("totalrecall: {} terminated", arguments[1]);
                    exit(-1);
                }
            }
        };
    }
}
