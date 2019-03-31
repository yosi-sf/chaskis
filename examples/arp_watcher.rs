extern crate chaskis;
//extern crate chaskis_datalink;

use std::env;
use std::io::{self, Write};
use std::net::{AddrParseError, IpAddr, Ipv4Addr};
use std::process;
use std::collections::HashSet;
use std::cmp::Eq;
use std::hash::Hash;
use std::vec::{Vec, Drain};

use pnet_datalink::{Channel, MacAddr, NetworkInterface, ParseMacAddrErr};

//use chaskis::packet::arp::{ArpHardwareTypes, ArpOperations};
//use chaskis::packet::arp::{ArpPacket, MutableArpPacket};
//use chaskis::packet::ethernet::EtherTypes;
//use chaskis::packet::ethernet::MutableEthernetPacket;
//use chaskis::packet::{MutablePacket, Packet};

use consts::{KeeperState, WatchedEventType};
use consts::WatchedEventType::{NodeCreated, NodeDataChanged, NodeDeleted, NodeChildrenChanged};
use proto::ReadFrom;
use zookeeper::RawResponse;
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;
use std::io;

use rand;

use super::chaskis_cipher::{NodeId};
use super::MurmurObserver;

