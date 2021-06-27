use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::sync::mpsc::{channel, Sender};
use std::thread;

mod arguments;

const MAX_PORT: u16 = 65535;

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, thread_count: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };

        if (MAX_PORT - port) <= thread_count {
            break;
        }

        port += thread_count;
    }
}

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

    let (tx, rx) = channel();
    let arguments::Arguments {
        ip_addr, threads, ..
    } = arguments;

    for i in 0..threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, ip_addr, threads);
        });
    }

    drop(tx);
    let mut open_ports = vec![];
    for port in rx {
        open_ports.push(port);
    }

    println!("");
    open_ports.sort();
    for p in open_ports {
        println!("open : {}", p);
    }
}
