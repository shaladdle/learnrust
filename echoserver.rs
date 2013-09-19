extern mod extra;

use std::os::args;
use std::cell::Cell;
use std::rt::io::{Acceptor, Listener, Reader, Stream, Writer};
use std::rt::io::net::ip::SocketAddr;
use std::rt::io::net::tcp::{TcpListener, TcpStream};

use extra::getopts::{getopts, reqopt, opt_str, fail_str};

fn transfer(input: @mut Reader, output: @mut Writer) {
    let mut buf = [0, ..1024];
    loop {
        match input.read(buf) {
            Some(n) => output.write(buf.slice_to(n)),
            None => {
                println!("Closing client {:?}", input);
                break;
            }
        }
    }
}

fn start_echoing<S: Stream + Send>(mut a: ~Acceptor<S>) {
    loop {
        match a.accept() {
            Some(client) => {
                println!("Connecting to client {:?}", client);
                let client = Cell::new(client);
                do spawn {
                    let client = @mut client.take();
                    transfer(client as @mut Reader, client as @mut Writer)
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
    start_echoing(~a as ~Acceptor<TcpStream>);
}
