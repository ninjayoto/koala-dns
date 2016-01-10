// source files all in named files in src. This is just for Cargo.toml
#![feature(convert)]
#![feature(vec_push_all)]
#![feature(test)]
extern crate getopts;
extern crate mio;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate test;

pub mod server;

mod dns;
mod command_line;
mod server_mio;
mod udp_request;

mod buf;