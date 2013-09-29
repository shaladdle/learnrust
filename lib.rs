// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

use std::rt::io::{Acceptor, Reader, Stream, Writer};
use std::task::spawn_with;

// Reads from a stream and writes back
// to the stream exactly what was read.
fn echo<S: Stream>(mut client: S) {
    let mut buf = [0, ..1024];
    loop {
        match client.read(buf) {
            Some(n) => client.write(buf.slice_to(n)),
            None => break,
        }
    }
}

// Accept clients, spawning a routine for each to echo
// incoming data.
pub fn start_echoing<S: Stream + Send, A: Acceptor<S>>(mut a: A) {
    for client in a.incoming() {
        do spawn_with(client.unwrap()) |client| {
            echo(client)
        }
    }
}
