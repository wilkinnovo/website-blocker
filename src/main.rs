use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[allow(dead_code)]
#[derive(Debug)]
struct Domain {
    name: String,
}

fn get_hosts_path() -> &'static str {
    if cfg!(windows) {
        "C:/Windows/System32/drivers/etc/hosts"
    } else {
        "/etc/hosts"
    }
}

fn domain_in_hosts_file(domain: &Domain) -> bool {
    let hosts_path = get_hosts_path();
    let mut file = match File::open(hosts_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open hosts file.");
            return false;
        }
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        eprintln!("Failed to read hosts file.");
        return false;
    }
    contents.contains(&domain.name) || contents.contains(&format!("www.{}", domain.name))
}

fn block(domain: Domain) {
    if domain_in_hosts_file(&domain) {
        println!("{:?} is already blocked", domain.name);
    } else {
        let hosts_path = get_hosts_path();

        // Open the hosts file in append mode
        let mut file = match OpenOptions::new().append(true).open(hosts_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Failed to open hosts file for writing. You might need elevated access. Check your permissions level.");
                return;
            }
        };

        // Write the domain and www.domain to the hosts file
        if let Err(e) = writeln!(file, "127.0.0.1 {}", domain.name) {
            eprintln!("Error writing to hosts file: {}", e);
            return;
        }
        if let Err(e) = writeln!(file, "127.0.0.1 www.{}", domain.name) {
            eprintln!("Error writing to hosts file: {}", e);
            return;
        }

        println!("Blocked: {:?}", domain);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let action = &args[1];
            if action == "block" {
                let name = &args[2];
                let domain = Domain {
                    name: name.to_string(),
                };

                println!("Action: {}", action);
                println!("Domain: {}", name);

                // Block
                block(domain);
            } else {
                println!("Missing block argument.");
            }
        }
        _ => println!("Missing proper arguments. ...example: block cnn.com ."),
    }
}
