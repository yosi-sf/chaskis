use std::collections::HashSet;
use std::cmp::Eq;
use std::hash::Hash;
use std::vec::{Vec, Drain}

use rand;

use super::chaskis_cipher::{NodeId};
use super::MurmurObserver;

//pub(crate) fn<'a>