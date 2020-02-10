use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct Client<'a> {
    endpoint: &'a str,
    statefile: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(endpoint: &'a str, statefile: &'a str) -> Self {
        Client {
            endpoint,
            statefile,
        }
    }

    pub fn get_ip(&self) -> Result<String, std::io::Error> {
        ureq::get(self.endpoint)
            .call()
            .into_string()
    }

    pub fn write_ip(&self, ip: &str) -> std::io::Result<()> {
        eprintln!("Writing statefile: {}", self.statefile);

        let mut f = File::create(self.statefile)?;

        f.write_all(ip.as_bytes())?;

        Ok(())
    }

    pub fn read_ip(&self) -> std::io::Result<String> {
        let mut f = File::open(self.statefile)?;
        let mut ip = String::new();

        f.read_to_string(&mut ip)?;

        Ok(ip)
    }

    pub fn ip_needs_update(&self, current_ip: &str) -> std::io::Result<bool> {
        if ! Path::new(self.statefile).exists() {
            eprintln!("No statefile found. Need to write it.");

            // needs to update because the statefile doesn't even exist
            return Ok(true);
        }

        eprintln!("Found statefile: {}", self.statefile);

        // statefile exists, so let's read it in and compare
        let old_ip = self.read_ip()?;

        Ok(old_ip != current_ip)
    }
}


