use std::io;
use std::process::{Command, Output};

#[derive(Debug, Clone)]
pub struct Ip(pub String);

impl Ip {
    fn get_main_part(&self) -> &str {
        match self.0.rfind('.').map(|idx| &self.0[..idx]) {
            Some(s) => s,
            None => "",
        }
    }
}

pub fn get_ips(router_ip: Ip) -> Result<Vec<Ip>, io::Error> {
    match run_nmap_script(router_ip) {
        Ok(_) => println!("Nmap scann success"),
        Err(message) => { Err(io::Error::new(io::ErrorKind::NotFound, message)) }?,
    };
    Ok(Vec::new())
}

fn run_nmap_script(router_ip: Ip) -> Result<Output, io::Error> {
    Command::new("sh")
        .args(&[
            "scripts/nmap-scan.sh",
            match router_ip.get_main_part() {
                "" => return Err(io::Error::new(io::ErrorKind::NotFound, "")),
                s => s,
            },
        ])
        .output()
}
