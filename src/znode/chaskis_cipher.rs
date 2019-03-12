// Copyright Venire Labs Inc 2019 Karl Whitford & Mateo Orland Dual-licensed MIT and Apache 2.0 (see LICENSE files for details).

use std::vec::Vec;
use std::net::SocketAddr;
use bincode::{serialize_into, deserialize, Bounded, Result as BincodeResult, ErrorKind as BincodeErrorKind};
use tokio_core::net::UdpCodec;
use zookeeper::{Acl, CreateMode, WatchedEvent, ZooKeeper};
use zookeeper::KeeperState;
//use ZkChaskisCluster;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::thread;
use env_logger;

#[derive(Debug, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct chaskisNodeId(pub SocketAddr);

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct IncomingMessage {
    source: SocketAddr,
    content: MessageContent,
}

fixed_size_list!(ChaskisMurmurList: Murmur; 5; derive(Debug, PartialEq, Clone, Serialize, Deserialize));
fixed_size_list!(KnownChaskisList: chaskisNodeId; 5; derive(Debug, Eq, PartialEq, Clone, Serializable, Deserialize));

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub (crate) enum ChaskisMessageContent {
    Automaton(ControlRequest, ChaskisMurmurList)
}

