// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

use std::io::{Acceptor, Reader, Stream, Writer, Listener, TcpListener, IoResult, IoError, EndOfFile};

// Reads from a stream and writes back
// to the stream exactly what was read.
fn echo<S: Stream>(mut client: S) -> IoResult<()> {
    let mut buf = [0, ..1024];
    loop {
        match client.read(buf) {
            Ok(n) => try!(client.write(buf.slice_to(n))),
            Err(e) => return verify_eof(e),
        }
    }
}

fn verify_eof(err: IoError) -> IoResult<()> {
    match err.kind {
        EndOfFile => {
            println!("Client closed connection.");
            Ok(())
        },
        _ => Err(err),
    }
}


pub fn run(addr: &str, port: u16) {
    match TcpListener::bind(addr, port) {
        Ok(l) => start_echoing(l.listen()),
        Err(e) => println!("{}", e),
    }
}

// Accept clients, spawning a routine for each to echo
// incoming data.
pub fn start_echoing<S: Stream + Send, A: Acceptor<S>>(mut a: A) {
    for client in a.incoming() {
        spawn(proc() {
            match echo(client) {
                Err(e) => println!("Error during echoing: {}", e),
                _ => (),
            }
        });
    }
}
