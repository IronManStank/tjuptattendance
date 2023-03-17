//! 自定义 API

use std::hash::Hash;

use crate::error::{ApiError, Error};
use serde::{Deserialize, Serialize};
use util::{model::doubandata::DouBanData, CLIENT};

/// 自定义API
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct MyApi {
    pub(crate) url: String,
    pub(crate) token: Option<String>,
}

impl MyApi {
    /// 获取一个豆瓣数据
    pub async fn get_answer(&self, title: &str) -> Result<DouBanData, Error> {
        let an_lst: Vec<DouBanData> = CLIENT
            .get(&self.url)
            .query(&[("q", Some(title)), ("t", self.token.as_deref())])
            .send()
            .await?
            .json()
            .await?;

        match an_lst.into_iter().next() {
            None => Err(ApiError::Tired.into()),
            Some(an) => Ok(an),
        }
    }
}

impl Hash for MyApi {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl PartialEq for MyApi {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl std::fmt::Display for MyApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.url)
    }
}
