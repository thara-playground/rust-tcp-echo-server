use std::env;
extern crate nix;

mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage : {} {{client|server}}", args[0]);
        std::process::exit(1);
    }

    let mode = env::args().nth(1).expect("Missing argument");
    match mode.as_ref() {
        "client" => {
            let addr = env::args().nth(2).expect("Missing argument");
            let msg = env::args().nth(3).expect("Missing argument");
            let _ = client::run_client(addr, msg);
        },
        "server" => {
            let addr = env::args().nth(2).expect("Missing argument");
            let _ = server::run_server(addr);
        },
        _ => {
            eprintln!("Unsupported mode : {}", mode);
            eprintln!("Usage : {} {{client|server}}", args[0]);
            std::process::exit(1);
        }
    }
}
