use std::env;
use std::process;
use identic::helpers::config;

// This is the main function of the program.
fn main() {
    // Gets the command line arguments and parses them using the config module.
    let args: Vec<String> = env::args().collect();
    let conf = config::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // Calls the identic module (lib.rs) to run the program.
    if let Err(e) = identic::run(conf) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

