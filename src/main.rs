// echoserver.rs
// 
// A simple echo server that writes back to clients
// their messages verbatim. Start by typing ./echoserver --addr <ip>:<port>
// 
// Authored by Adam Wright (adam.austin.wright@gmail.com)
// and Tim Kuehn (timothy.j.kuehn@gmail.com), 2013

extern crate getopts;

use lib::run;

use std::os::args;
use std::io::{IoResult, IoError, InvalidInput};

use getopts::{ArgumentMissing, Fail_, Matches, OptionDuplicated, OptGroup, OptionMissing, 
    UnexpectedArgument, UnrecognizedOption, getopts, reqopt};

mod lib;

/// The list of required arguments to the main function
fn opts() -> [OptGroup, ..2] {
    [
        reqopt("a", "addr", "The ip address to bind to", ""),
        reqopt("p", "port", "The port to bind to", "")
    ]
}

/// Parse and validate the existence of the arguments to the main function
fn parse_args() -> IoResult<(String, u16)> {
    match getopts(args().tail(), opts()) {
        Ok(matches) => Ok((addr(&matches), try!(str_to_u16(port(&matches).as_slice())))),
        Err(fail) => get_fail_condition(fail),
    }
}

/// Get the address string from the matched arguments
fn addr(matches: &Matches) -> String {
    matches.opt_str("addr").unwrap()
}

/// Get the port string from the matched arguments
fn port(matches: &Matches) -> String {
    matches.opt_str("port").unwrap()
}

/// Convert a string to a u16, if possible.
/// Returns Ok(u16), if successful; otherwise, an IoResult describing the failure to parse 
fn str_to_u16(s: &str) -> IoResult<u16> {
    from_str(s)
        .map(|p| Ok(p))
        .unwrap_or(bad_port())
}

/// Constructs an IoResult error describing an invalid argument for the port
fn bad_port<T>() -> IoResult<T> {
    Err(IoError {
        kind: InvalidInput,
        desc: "port provided cannot be parsed to u16",
        detail: None
    })
}

/// Constructs an IoResult error describing a failure mode encountered by getopts
fn get_fail_condition<T>(fail: Fail_) -> IoResult<T> {
    Err(IoError {
            kind: InvalidInput,
            desc: match fail {
                ArgumentMissing(_) => "argument missing",
                UnrecognizedOption(_) => "unrecognized option",
                OptionMissing(_) => "option missing",
                OptionDuplicated(_) => "option duplicated",
                UnexpectedArgument(_) => "unexpected arguments",
            },
            detail: Some(match fail {
                ArgumentMissing(s) => s,
                UnrecognizedOption(s) => s,
                OptionMissing(s) => s,
                OptionDuplicated(s) => s,
                UnexpectedArgument(s) => s,
            })
        })
}

fn main() {
    match parse_args() {
        Ok((addr, port)) => run(addr.as_slice(), port),
        Err(e) => println!("{}", e),
    }
}
