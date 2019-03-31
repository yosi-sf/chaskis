use super::chaskis_cipher::{Murmur, NodeId};
use chrono::NaiveDateTime;
use std::cmp::{min, Ord, PartialOrd, Ordering, PartialEq}

#[derive(Clone, Debug, Eq, Ord, Insertable, PartialEq, PartialOrd)]
//#[leaderboard_name="zookeeper leaderboard"]

pub struct NewChaskisPoll<'a> {

    pub zookeeper_issue: i32,
    pub zookeeper_init: i32,
    pub zookeeper_initiating_status: i32,
    pub zookeeper_bot_tracking_status: i32,
    pub poll_status: &'a str,
    pub poll_created_at: NaiveDateTime,
    pub poll_closed: bool,
    pub poll_agents: &'a str,


}
struct MurmurState {
    spread: u16,
    murmur: Murmur,
}

impl PartialEq for MurmurState {
    fn eq(&self, other: &Self) -> bool{
        self.spread == other.spread && self.murmur == other.murmur
    }
}

impl PartialOrd for MurmurState {
    fn partial_cmp(&self, other: &Self) -> option<Ordering> {
        self.spread.partial_cmp(&other.spread)
    }
}