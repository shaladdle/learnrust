// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

extern mod extra;

use std::rt::io::{Acceptor, Reader, Stream, Writer};
use std::task::spawn_with;

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
pub fn start_echoing<S: Stream + Send, A: Acceptor<S>>(mut a: A) {
    loop {
        match a.accept() {
            Some(client) => {
                println!("Connecting to client {:?}", client);
                do spawn_with(client) |client| {
                    let client = @mut client;
                    transfer(client, client)
                }
            }
            None => println("error in accept"),
        }
    }
}
