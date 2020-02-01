use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::env;

use std::process;

mod digitalocean;

extern crate ureq;

static STATEFILE: &str = "current_ip.txt";

fn get_ip(endpoint: &str) -> Result<String, std::io::Error> {
    ureq::get(endpoint)
        .call()
        .into_string()
}

fn write_ip(ip: &str) -> std::io::Result<()> {
   let mut f = File::create(STATEFILE)?;

   f.write_all(ip.as_bytes())?;

   Ok(())
}

fn read_ip() -> std::io::Result<String> {
    let mut f = File::open(STATEFILE)?;
    let mut ip = String::new();

    f.read_to_string(&mut ip)?;

    Ok(ip)
}

fn ip_needs_update(current_ip: &str) -> std::io::Result<bool> {
    if ! Path::new(STATEFILE).exists() {
        // needs to update because the statefile doesn't even exist
        return Ok(true);
    }

    // statefile exists, so let's read it in and compare
    let old_ip = read_ip()?;

    Ok(old_ip != current_ip)
}

fn main() -> Result<(), std::io::Error> {
    let endpoint = match env::var("IP_ENDPOINT") {
        Ok(endpoint) => endpoint,
        _ => panic!("No IP_ENDPOINT set in environment."),
    };
    let ip = get_ip(&endpoint)?;
    println!("Hello, world: {}", ip);

    // testing out some DO stuff
    let config = digitalocean::Config::new_from_environment();


    if ip_needs_update(&ip)? {
        eprintln!("Needs update. Writing IP to file...");
        write_ip(&ip)?;

        let record_id = 
            match digitalocean::find_record_id(&config) {
                Some(record_id) => record_id,
                None => panic!("got no record"),
            };

        println!("got id: {}", record_id);

        let resp = digitalocean::update_record(&config, record_id, &ip);

        if resp.ok() {
            eprintln!("Updated record!");
        } else {
            eprintln!("Failed to update record ({}): {}", resp.status(), resp.into_string().unwrap());
        }
    } else {
        eprintln!("Nothing to do. IP is the same.");
        process::exit(0);
    }


    Ok(())
}
