// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

extern mod extra;

use std::os::args;
use std::cell::Cell;
use std::rt::io::{Acceptor, Listener, Reader, Stream, Writer};
use std::rt::io::net::ip::SocketAddr;
use std::rt::io::net::tcp::TcpListener;

use extra::getopts::{getopts, reqopt};

// General transfer function that transmits data 
// from a generic reader to a generic writer. Uses @mut
// so that a Stream can be both read from and written to
fn transfer<R: Reader, W: Writer>(input: @mut R, output: @mut W) {
    let mut buf = [0, ..1024];
    loop {
        match input.read(buf) {
            Some(n) => output.write(buf.slice_to(n)),
            None => {
                println!("Finished transferring from {:?} to {:?}",
                    input, output);
                break;
            }
        }
    }
}

// Accept clients, spawning a routine for each to echo
// incoming data.
fn start_echoing<S: Stream + Send, A: Acceptor<S>>(mut a: A) {
    loop {
        match a.accept() {
            Some(client) => {
                println!("Connecting to client {:?}", client);
                let client = Cell::new(client);
                do spawn {
                    let client = @mut client.take();
                    transfer(client, client)
                }
            }
            None => println("error in accept"),
        }
    }
}

static USAGE: &'static str = "Usage: ./echoserver --addr <ip_addr>:<port>";

fn main() {
    //----- Parse the required argument, "addr", of the form <ip_addr>:<port>
    let args = args();
    let opts = [reqopt("addr")];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };
    let saddr = matches.opt_str("addr")
        .expect(USAGE);
    let addr : SocketAddr = FromStr::from_str(saddr)
        .expect(USAGE);
    let l = TcpListener::bind(addr)
        .expect(format!("failed to listen on socket {}", saddr));

    //----- Main routine
    let a = l.listen();
    start_echoing(a);
}
