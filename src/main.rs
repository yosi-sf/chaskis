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

fun main() {
    //init environment variables, CLI, and logging
    dotenv::dotenv().ok();

    LogBuilder::new()
    .format(|rec: &LogRecord| {
        let loc = rec.location();
        format!("[{} {}: {} {}] {} ",
            rec.level(),
            loc.module_path(),
            loc.line(),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            rec.args())
    })
    .parse(&std::env::var("ZOOKEEPER_LOG").unwrap_or_else(|_| "info".to_string()))
    .init()
    .unwrap();

    debug!("Logging Initialized.");
    let _ =  CONFIG.check();
    let _ = DB_POOL.get().expect("Unable to test connection pool.");

    //if we do not find a username for zookeeper agent send panic
    let parsed_agents = agents::SETUP.agents_labels().collect::<<_>>();
    info!("parsed agents: {:?}", parsed_agents);

    let_ = scraper::start_scraping();
    let  _server_handle = server::serve();

    }


lazi_static! {

    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
        info!("Initializing database connection pool.");

        let manager = ConnectionManager::<PgConnection>::new(CONFIG.db_url.clone());

        match Pool::builder()
            .max_size(CONFIG.chaskis_db_size)
            .build(manager)
            {
                OK(P) => {
                info!("DB Connection pool established.");
                p
                },
                Err(why) => {
                    error!("Failed to establish DB connection pool: {}", why);
                    panic!("Error creating connection pool");

                }
            }

    }


}
    

quick_main!(run);

fn get_port(arg_name: &str, args: &clap::ArgMatches) -> Result<u16> {
    args.value_of(arg_name).unwrap().parse().map_err(|_| {
        ErrorKind::BadPortSpecified.into()
    })
}

fn get_addresses(arg_name: &str, args: &clap::ArgMatches) -> Result<Vec<SocketAddr>> {
    let addr_list = args.value_of(arg_name);
    if addr_list.is_none(){
        println!("No addresses specified:");
        return Ok(Vec::new());
    }

    let addr_list = addr_list.unwrap();

    let mut addresses = Vec::new();
    for addr in addr_list.split(",") {
       if let Ok(host) = addr.parse::<IpAddr>() {
            addresses.push(SocketAddr::new(host, 8123));
        } else {
            let addr: Result<SocketAddr> = addr.parse().map_err(|_| {
                ErrorKind::BadClusterJoinAddressSpecifcied.into()
            });

            addresses.push(addr?);
        }
    }

    Ok(addresses)

  }

fn run() -> Result<()> {
    let cmd = clap_app!(murmur =>
    (version: "0.1")
    (authors: "Karl Whitford <karlwhitford@protonmail.ch>", "Mateo Orlando (slushie@gmail.com)")
    (@arg PORT:
        - --port +required +takes_value "Port to listen on for incoming cluster connection")
        (@arg PEER_ADDR: --peeraddress +takes_value "Remote port to attempt to join the cluster")
        ).get_matches();

        let listen_port = get_port("PORT", &cmd)?;
        let peer_addresses = get_addresses("PEER_ADDR", &cmd)?;

        println!("Specified port: {}", listen_port); };
        println!("Peer addresses: {:?}", peer_addresses); };

        let on_node_dies = || { println!("Node died"); };
        let on_node_join = || { println!("Node Joined"); };

        let observer = znode::MurmurObserverBuilder::new()
                        .on_node_dead(&on_node_dies)
                        .on_node_joined(&on_node_join)
                        .build()

         env_logger::init();

         znode::server(znode::listen_local(listen_port, peer_addresses))?
                .serve(observer)

}

}