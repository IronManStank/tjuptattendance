use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct User {
    pub(crate) name: String,
    pub(crate) pwd: String,
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pwd(&self) -> &str {
        &self.pwd
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User[{}]", self.name)
    }
}
