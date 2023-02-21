use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq)]
pub struct User {
    pub(crate) name: String,
    pub(crate) pwd: String,
}

impl User {
    pub fn new(name: String, pwd: String) -> Self {
        Self { name, pwd }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pwd(&self) -> &str {
        &self.pwd
    }
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
        write!(f, "User[{}]", self.name())
    }
}
