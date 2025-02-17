pub mod slapshot;
pub mod db;

use std::sync::Arc;
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Username(Arc<str>);

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Username(Arc::from(value))
    }
}

impl Deref for Username {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(Arc<str>);

impl From<&str> for PlayerId {
    fn from(value: &str) -> Self {
        PlayerId(Arc::from(value))
    }
}

impl Deref for PlayerId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchId(Arc<str>);

impl From<&str> for MatchId {
    fn from(value: &str) -> Self {
        MatchId(Arc::from(value))
    }
}

impl Deref for MatchId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
