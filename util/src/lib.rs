//! 为 tjuptop(client) & server 通用部分
//!
//! ## 包括
//! 1. 通讯数据(Answer)
//! 2. 对比 Poster & Answer

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod error;
pub mod model;

use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, ClientBuilder};

lazy_static! {
    /// 默认的 User-Agent
    static ref HEADERS: HeaderMap = {
        let mut map = HeaderMap::default();
        map.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/100.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        map
    };

    /// 简单的 Client
    pub static ref CLIENT: Client = get_client();
}

/// 获取一个简单client
pub fn get_client() -> Client {
    ClientBuilder::new()
        .default_headers(HEADERS.clone())
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .unwrap_or_default()
}
