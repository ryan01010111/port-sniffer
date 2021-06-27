use std::env;
use std::process;

mod arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = arguments::Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} Error: {}", program, err);
            process::exit(0);
        }
    });
    println!(
        "{}, {}, {}",
        arguments.flag, arguments.ip_addr, arguments.threads
    );
}
