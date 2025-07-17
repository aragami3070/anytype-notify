use std::io;
use std::process::{Command, Output};

#[derive(Debug, Clone)]
pub struct Ip(pub String);

impl Ip {
	fn get_main_part(&self) -> &str {
		match self.0.rfind('.').map(|idx| &self.0[..idx]) {
			Some(s) => s,
			None => {
				""
			}
		}
	}
}


pub fn get_ips(router_ip: Ip) -> Result<Vec<Ip>, io::Error> {
    Ok(Vec::new())
}

