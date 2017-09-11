use std::str;
use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use nix::libc::{SIGPIPE, SIG_IGN};
use nix::libc::signal;

pub fn run_client(addr: String, msg: String) -> io::Result<()> {
    unsafe { signal(SIGPIPE, SIG_IGN); }

    let mut stream = try!(TcpStream::connect(addr));
    stream.set_read_timeout(None).expect("set_read_timeout call failed");
    stream.set_write_timeout(None).expect("set_write_timeout call failed");

    let s = try!(stream.write(msg.as_bytes()));
    println!("Write {} bytes to {}", s, try!(stream.peer_addr()));

    let mut bf = [0u8; 1024];
    let n = try!(stream.read(&mut bf));
    if n == 0 {
        println!("{:?} closed.", try!(stream.peer_addr()));
    } else {
        println!("Read {} bytes from {:?}", s, try!(stream.peer_addr()));
        let s = match str::from_utf8(&bf) {
            Ok(v) => v,
            Err(_) => "Invalid UTF-8 sequence",
        };
        println!("{}", s);
    }
    Ok(())
}
