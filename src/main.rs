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

use getopts::{getopts, reqopt, OptGroup, Matches, Fail_, ArgumentMissing, UnrecognizedOption, OptionMissing, OptionDuplicated, UnexpectedArgument};

mod lib;

fn opts() -> [OptGroup, ..2] {
    [
        reqopt("a", "addr", "The ip address to bind to", ""),
        reqopt("p", "port", "The port to bind to", "")
    ]
}

fn parse_args() -> IoResult<(String, u16)> {
    match getopts(args().tail(), opts()) {
        Ok(matches) => Ok((addr(&matches), try!(str_to_port(port(&matches).as_slice())))),
        Err(fail) => get_fail_condition(fail),
    }
}

fn addr(matches: &Matches) -> String {
    matches.opt_str("addr").unwrap()
}

fn port(matches: &Matches) -> String {
    matches.opt_str("port").unwrap()
}

fn str_to_port(port: &str) -> IoResult<u16> {
    from_str(port)
        .map(|p| Ok(p))
        .unwrap_or(bad_port())
}

fn bad_port<T>() -> IoResult<T> {
    Err(IoError {
        kind: InvalidInput,
        desc: "port provided cannot be parsed to u16",
        detail: None
    })
}

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

