extern mod extra;

use std::os::args;
use std::cell::Cell;
use std::rt::io::{Reader, Writer, Listener};
use std::rt::io::net::ip::SocketAddr;
use std::rt::io::net::tcp::{TcpListener};

use extra::getopts::{getopts, reqopt, opt_str, fail_str};

fn transfer<R : Reader, W : Writer>(input: @mut R, output: @mut W) {
    let mut buf = [0, ..1024];
    loop {
        match input.read(buf) {
            Some(n) => output.write(buf.slice_to(n)),
            None => break,
        }
    }
}

fn main() {
    let args = args();
    let opts = [reqopt("addr")];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(fail_str(f)),
    };
    let saddr = opt_str(&matches, "addr");
    let addr : SocketAddr = FromStr::from_str(saddr)
        .expect("invalid address: " + saddr);
    let mut l = TcpListener::bind(addr)
        .expect(format!("failed to listen on socket {}", saddr));

    loop {
        match l.accept() {
            Some(client) => {
                let client = Cell::new(client);
                do spawn {
                    let client = @mut client.take();
                    transfer(client, client)
                }
            },
            None => println("error in accept"),
        }
    }
}
