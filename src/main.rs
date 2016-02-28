#![feature(test)]
extern crate getopts;
extern crate mio;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate test;
//extern crate koala_dns;

mod command_line;
mod server;
mod server_mio;
mod dns;
mod request;
mod buf;

use server::ServerOps;

fn main() {
	env_logger::init().unwrap_or_else(|err| println!("Failed to initialize logger. {:?}", err));

    let config = command_line::parse_args();
    let mut server = server::Server::new(config.port, config.server, config.timeout);
    server.start();
}
