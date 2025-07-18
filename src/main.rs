mod nmap;
use std::process;

use dotenv::dotenv;
use nmap::scanner;

use crate::nmap::scanner::Ip;

fn main() {
    dotenv().ok();

    let local_ip = Ip(std::env::var("LOCAL_IP").expect("LOCAL_IP must be set in .env."));

    let list_ip = match scanner::get_ips(local_ip) {
        Ok(l) => l,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    for ip in list_ip {
        println!("ip: {}", ip.0);
    }
}
