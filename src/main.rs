use std::env;
use std::process;

fn main() {
    
    let args = env::args();

    let config = minigrep::create_config(args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}





