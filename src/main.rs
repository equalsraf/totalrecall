
use std::process::{Command, exit};
use std::env;
use std::time::SystemTime;

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
                if let Some(code) = status.code() {
                    println!("totalrecall: {} exited with code({}), restarting",
                             arguments[1], code);
                } else {
                    // TODO: handle signals properly e.g. SIGINT is sufficient reason
                    // to exit instead of restarting
                    println!("totalrecall: {} terminated, restarting",
                             arguments[1]);
                }
            }
        };
    }
}
