use std::env;
#[allow(dead_code)]
#[derive(Debug)]
struct Domain {
    name: String,
    favicon: String,
}

fn block(domain: Domain) {
    println!("blocking: {:?}", domain);
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
                    favicon: "google.com/favicon".to_string(),
                };
                // Logging
                println!("ACTION: {}", action);
                println!("DOMAIN: {}", name);
                // Block
                block(domain);
            } else {
                println!("Missing block argument");
            }
        }
        _ => println!("Missing proper arguments. ...example: block cnn.com"),
    }
}
