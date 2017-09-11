use std::str;
use std::io;
use std::io::{Read, Write};
use std::net::TcpListener;
use nix::libc::{SIGPIPE, SIG_IGN};
use nix::libc::signal;

pub fn run_server(addr: String) -> io::Result<()> {
    unsafe { signal(SIGPIPE, SIG_IGN); }

    let listener = try!(TcpListener::bind(addr));
    println!("started on {:?}", listener);

    for stream in listener.incoming() {
        let mut stream = try!(stream);
        println!("{:?} connected.", try!(stream.peer_addr()));

        let mut bf = [0; 1024];

        let n = try!(stream.read(&mut bf));
        if n == 0 {
            println!("{:?} closed.", try!(stream.peer_addr()));
            return Ok(());
        } else {
            let s = match str::from_utf8(&bf) {
                Ok(v) => v,
                Err(_) => "Invalid UTF-8 sequence",
            };
            println!("Received '{}'", s);

            try!(stream.write(&bf[0..n]));
        }
    }
    Ok(())
}
