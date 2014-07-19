// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

use std::io::{Acceptor, EndOfFile, IoResult, IoError, Listener, Stream, TcpListener};

/// Reads from a stream and writes back
/// to the stream exactly what was read.
fn echo<S: Stream>(mut client: S) -> IoResult<()> {
    let mut buf = [0, ..1024];
    loop {
        match client.read(buf) {
            Ok(n) => try!(client.write(buf.slice_to(n))),
            Err(e) => return verify_eof(e),
        }
    }
}

/// Checks an IoError to see if it's EOF.
/// Returns Ok(()) if it is EOF and Err(err) otherwise.
fn verify_eof(err: IoError) -> IoResult<()> {
    match err.kind {
        EndOfFile => {
            println!("Client closed connection.");
            Ok(())
        },
        _ => Err(err),
    }
}

/// Accept clients, spawning a routine for each to echo
/// incoming data.
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

/// Starts up an echo server using the given address and port
pub fn run(addr: &str, port: u16) {
    match TcpListener::bind(addr, port) {
        Ok(l) => start_echoing(l.listen()),
        Err(e) => println!("{}", e),
    }
}

