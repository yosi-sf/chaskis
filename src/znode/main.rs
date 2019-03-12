#![feature(never_type)]
#![feature(proc_macro_hygiene, decl_macro)]
extern crate chrono;
extern crate crypto;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate hex;
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate lazy_static;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;
extern crate url;
extern crate urlencoded;
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

#[macro_use]
extern crate maplit;
extern crate itertools;
#[macro_use]
mod macros;
mod config;
mod domain;
mod error;
mod github;
mod nag;
mod scraper;
mod server;
mod teams;
use chrono::Local;
use diesel::pg::PgConnection;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use env_logger::LogBuilder;
use log::LogRecord;
use config::CONFIG;

quick_main!(run);

fn get_port(arg_name: &str, args: &clap::ArgMatches) -> Result<u16> {
    args.value_of(arg_name).unwrap().parse().map_err(|_| {
        ErrorKind::BadPortSpecified.into()
    })
}

