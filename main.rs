// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

extern mod extra;

use lib::start_echoing;
use std::os::args;
use std::rt::io::{Acceptor, Listener, Reader, Writer};
use std::rt::io::net::ip::SocketAddr;
use std::rt::io::net::tcp::TcpListener;

use extra::getopts::{getopts, reqopt};

mod lib;

static USAGE: &'static str = "Usage: ./echoserver --addr <ip_addr>:<port>";

fn main() {
    //----- Parse the required argument, "addr", of the form <ip_addr>:<port>
    let args = args();
    let opts = [reqopt("addr")];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };
    let saddr = matches.opt_str("addr").expect(USAGE);
    let addr : SocketAddr = FromStr::from_str(saddr).expect(USAGE);
    let l = TcpListener::bind(addr)
        .expect(format!("failed to listen on socket {}", saddr));

    //----- Main routine
    start_echoing(l.listen());
}
