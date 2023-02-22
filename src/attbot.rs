use std::sync::Arc;

use anyhow::anyhow;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

use crate::{Error, User};

#[derive(Debug)]
pub struct AttBot {
    pub(crate) user: User,
    pub(crate) clinet: Client,
    pub(crate) cookie: Arc<CookieStoreMutex>,
}

impl AttBot {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn clinet(&self) -> &Client {
        &self.clinet
    }

    pub fn clear_cookie(&self) -> Result<(), Error> {
        let mut lock = self.cookie.lock().map_err(|e| anyhow!("内部错误 {e}"))?;
        lock.clear();
        Ok(())
    }
}
