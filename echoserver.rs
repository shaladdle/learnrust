extern mod extra;

use std::os::args;
use std::cell::Cell;
use std::rt::io::{Reader, Writer, Listener, Acceptor};
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

fn start_echoing<A: Acceptor<S>, S: Reader + Writer + Send>(mut a: A) {
    loop {
        match a.accept() {
            Some(client) => {
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
    let l = TcpListener::bind(addr)
        .expect(format!("failed to listen on socket {}", saddr));

    let a = l.listen();
    start_echoing(a);
}
