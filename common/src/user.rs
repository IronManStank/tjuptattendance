use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// TJUPT 用户，比较时只考虑 name 是否相同
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct User {
    pub(crate) name: String,
    pub(crate) pwd: String,
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User[{}]", self.name)
    }
}
