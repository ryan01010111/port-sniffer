use std::net::IpAddr;
use std::str::FromStr;

const DEFAULT_THREAD_COUNT: u16 = 4;

pub struct Arguments {
  pub flag: String,
  pub ip_addr: IpAddr,
  pub threads: u16,
}

impl Arguments {
  pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
    if args.len() < 2 {
      return Err("No IP address provided");
    } else if args.len() > 4 {
      return Err("Too many arguments");
    }

    let arg1 = args[1].clone();
    if let Ok(ip_addr) = IpAddr::from_str(&arg1) {
      return Ok(Arguments {
        flag: String::new(),
        ip_addr,
        threads: DEFAULT_THREAD_COUNT,
      });
    } else {
      if (arg1.contains("-h") || arg1.contains("-help")) && args.len() == 2 {
        println!(
          "\nUsage: port_sniffer [OPTIONS] <IP address>
          \r\nOptions:
          \r  -t    number of threads to use (default: 4)"
        );
        return Err("help");
      } else if arg1.contains("-h") || arg1.contains("-help") {
        return Err("Too many arguments");
      } else if arg1.contains("-t") && args.len() == 4 {
        let ip_addr = match IpAddr::from_str(&args[3]) {
          Ok(s) => s,
          Err(_) => return Err("Invalid IP address; Must be IPv4 or IPv6"),
        };
        let threads = match args[2].parse::<u16>() {
          Ok(n) => n,
          Err(_) => return Err("Invalid input for number of threads"),
        };
        return Ok(Arguments {
          flag: arg1,
          ip_addr,
          threads,
        });
      } else {
        return Err("Invalid syntax");
      }
    }
  }
}
