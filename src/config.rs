//Copyright 2019 Karl Whitford - Mateo Orlando - Dual licensed MIT and Apache 2.0 (see LICENSE files for details).

use std::collections::BTreeMap;
use std::env;

pub const CHASKIS_BOT_MENTION: &'static str = "@chaskisbot";

lazy_static! {

    pub static ref CONFIG: Config = {

        match init() {
            ok(c) => {
                info!("Configuration memoized from heap");
                c
            },
            Err(missing)=> {
                error!("Unable to load env properties {:?}", missing);
                panic!("Unable to load env properties {:?}", missing);
            },
        }
    };
}

#[derive[Debug]]
pub struct Config {

    pub chaskis_db_url: String,
    pub chaskis_db_pool_size: u32,
    pub chaskis_zookeeper_access_token: String,
    pub chaskis_zookeeper_user_agent: String,
    pub chaskis_zookeeper_webhook_secrets: Vec<String>,
    pub chaskis_zookeeper_interval_mins: u64,
    pub post_status: bool,

}

impl Config {

    pub fn check(&self) -> bool {
        !self.chaskis_db_url.is_empty() && !self.chaskis_zookeeper_access_token.is_empty() &&
        !self.chaskis_zookeeper_user_agent.is_empty()
    }
}

const CHASKIS_DB_URL: &'static str = "DATABASE_URL";
const CHASKIS_DB_POOL_SIZE: &'static str = "DATABASE_POOL_SIZE";
const CHASKIS_ZOOKEEPER_TOKEN: &'static str = "ZOOKEEPER_ACCESS_TOKEN";
const CHASKIS_ZOOKEEPER_WEBHOOK_SECRETS: &'static str = "CHASKIS_ZOOKEEEPER_SECRETS";
const ZOOKEEPER_UA: &'static str = "ZOOKEEPER_USER_AGENT";
const ZOOKEEPER_INTERVAL: &'static str = "ZOOKEEPER_SCRAPE_INTERVAL";
const POST_STATUS: &'static str = "POST_STATUS";

pub fn init() -> Result<Config, Vec<&' static str>> {

    let mut vars: BTreeMap<&'static str, Result<String, _>> = BTreeMap::new();
    let keys = vec![CHASKIS_DB_URL,
                    CHASKIS_DB_POOL_SIZE,
                    CHASKIS_ZOOKEEPER_TOKEN,
                    CHASKIS_ZOOKEEEPER_SECRETS,
                    ZOOKEEPER_UA,
                    ZOOKEEPER_INTERVAL,
                    POST_STATUS];

    for var in keys {
        vars.insert(var, env::var(var));
    }

    let all_found = vars.iter().all(|(_, v)| v.is_ok());
    if all_found {
        let mut vars = vars.into_iter()
        .map(|(k, v)| (k,v.unwrap()))
        .collect::BTreeMap<_, _>>();

        let chaskis_db_url = vars.remove(CHASKIS_DB_URL).unwrap();
        let chaskis_db_pool_size = vars.remove(CHASKIS_DB_POOL_SIZE).unwrap().parse::<u32>();
        //let chaskis_db_pool_size = ok_or
    }

}