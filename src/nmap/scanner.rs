use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Output};

#[derive(Debug, Clone)]
pub struct Ip(pub String);

impl Ip {
    fn get_main_part(&self) -> &str {
        let ip_parts: Vec<&str> = self.0.split('.').collect();
        if ip_parts.len() != 4 {
            return "";
        }
        let str_length = ip_parts[0].len() + ip_parts[1].len() + ip_parts[2].len() + 2;
        &self.0[..str_length]
    }
}

pub fn get_ips(router_ip: Ip) -> Result<Vec<Ip>, io::Error> {
    match run_nmap_script(router_ip) {
        Ok(_) => println!("Nmap scann success"),
        Err(message) => { Err(io::Error::new(io::ErrorKind::NotFound, message)) }?,
    };

    match parse_nmap_result() {
        Ok(list_ip) => Ok(list_ip),
        Err(message) => { Err(io::Error::new(io::ErrorKind::NotFound, message)) }?,
    }
}

fn run_nmap_script(router_ip: Ip) -> Result<Output, io::Error> {
    Command::new("sh")
        .args([
            "scripts/nmap-scan.sh",
            match router_ip.get_main_part() {
                "" => return Err(io::Error::new(io::ErrorKind::NotFound, "Must be set IP.")),
                s => s,
            },
        ])
        .output()
}

fn parse_nmap_result() -> Result<Vec<Ip>, io::Error> {
    let file = match File::open("assets/nmap-scan-result.txt") {
        Ok(f) => f,
        Err(message) => return Err(io::Error::new(io::ErrorKind::NotFound, message)),
    };
    let reader = BufReader::new(file);

    let mut list_ip: Vec<Ip> = Vec::new();

    for line in reader.lines() {
        let text = match line {
            Ok(t) => t,
            Err(message) => return Err(io::Error::new(io::ErrorKind::NotFound, message)),
        };

        let ip = match text.find("Nmap scan report for") {
            Some(_) => extract_ip(text.trim()),
            None => continue,
        };

        match ip {
            Some(i) => list_ip.push(Ip(i)),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "File nmap-scan-result.txt contains invalid strings.",
                ));
            }
        };
    }

    Ok(list_ip)
}

fn extract_ip(input: &str) -> Option<String> {
    let start = input.rfind('(')? + 1;
    let end = input.rfind(')')?;

   if start >= end {
        return None;
    }

    let ip = &input[start..end];
    if ip.split('.').count() != 4 {
        return None;
    }

    Some(ip.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Ip("test".to_string()), "")]
    #[case(Ip("0.0.0.0".to_string()), "0.0.0")]
    #[case(Ip("192.168.50.19".to_string()), "192.168.50")]
    #[case(Ip("192.168.50".to_string()), "")]
    fn getting_main_part(#[case] ip: Ip, #[case] expected: &str) {
        assert_eq!(ip.get_main_part(), expected);
    }

    #[rstest]
    #[case("Nmap scan report for SomeComputer (192.168.50.228)", Some("192.168.50.228".to_string()))]
    #[case("Nmap scan report for SomeComputer (test)", None)]
    #[case("Nmap scan report for SomeComputer (192.168.50.228", None)]
    #[case("Nmap scan report for SomeComputer 192.168.50.228)", None)]
    #[case("Nmap scan report for SomeComputer 192.168.50.228)", None)]
    #[case("Nmap scan report for SomeComputer", None)]
    fn extracting_ip(#[case] ip: &str, #[case] expected: Option<String>) {
        assert_eq!(extract_ip(ip), expected);
    }
}
