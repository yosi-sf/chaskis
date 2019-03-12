#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate clap;

#[macro_use] extern crate log;
extern crate env_logger;

extern crate futures;
extern crate rand;
extern crate tokio_core;
extern crate tokio_timer;

use std::net::{SocketAddr, IpAddr};
use std::vec::Vec;

#[macro_use] mod fixed_list;
mod errors;
use errors::*;
mod node;

quick_main!(run);

fn get_port(arg_name: &str, args: &clap::ArgMatches) -> Result<u16> {
    args.value_of(arg_name).unwrap().parse().map_err(|_| {
        ErrorKind::BadPortSpecified.into()
    })
}

