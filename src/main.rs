use std::env;

use std::process;

mod digitalocean;
mod ip;

extern crate ureq;

static DEFAULT_STATEFILE: &str = "./current_ip.txt";

fn main() -> Result<(), std::io::Error> {
    let statefile = env::var("STATEFILE").unwrap_or_else(|_| DEFAULT_STATEFILE.to_string());

    let endpoint = match env::var("IP_ENDPOINT") {
        Ok(endpoint) => endpoint,
        _ => panic!("No IP_ENDPOINT set in environment."),
    };

    let ip_client = ip::Client::new(&endpoint, &statefile);

    let ip = ip_client.get_ip()?;

    println!("Current IP: {}", ip);

    // testing out some DO stuff
    let config = digitalocean::Config::new_from_environment();
    let client = digitalocean::Client::new(&config);

    eprintln!("Using hostname: {}", config.hostname);

    if ip_client.ip_needs_update(&ip)? {
        eprintln!("Needs update. Writing IP to file...");
        ip_client.write_ip(&ip)?;

        let record_id = 
            match client.find_record_id() {
                Some(record_id) => record_id,
                None => panic!("got no record"),
            };

        println!("got id: {}", record_id);

        let resp = client.update_record(record_id, &ip);

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
