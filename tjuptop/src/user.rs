//! TJUPT 用户

use std::hash::Hash;

use anyhow::anyhow;
use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// TJUPT User
///
/// `name` 相同的 `User` 将会被视为同一个
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) name: String,
    pub(crate) pwd: String,
    #[serde(skip)]
    pub(crate) cookie: CookieStoreMutex,
}

impl User {
    /// 保存cookies
    pub fn save_cookie(&self) -> Result<(), Error> {
        let lock = self.cookie.lock().map_err(|e| anyhow!("内部错误 {e}"))?;
        drop(lock);
        // lock.save_json(writer)
        todo!()
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
        write!(f, "{}", &self.name)
    }
}
