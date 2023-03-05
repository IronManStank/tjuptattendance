//! ## 豆瓣数据
//! `DouBanData` **不一定** 作为 server 中最终储存的结构
//!
//! ### 分为两种情况
//! 1. 直接使用豆瓣API获取
//! 2. 通过自建 server 获取答案
//!
//! 第一种情况则直接使用，第二种则需要更有效率的格式。
//!
//! 即：**附加** 除了原始信息以外的经过预处理的信息

use async_trait::async_trait;
use lazy_static::lazy_static;
#[cfg(feature = "origin_impl")]
use reqwest::ClientBuilder;
#[cfg(feature = "origin_impl")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

/// 可以直接使用的数据
pub trait Answer: Debug + Hash + PartialEq + Eq + Clone + Send + Sized {}

// 此数据应该与豆瓣API同步更新 (虽然不大可能)
/// 豆瓣API提供的原始数据
#[cfg(feature = "origin_impl")]
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct OriginDouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
}

#[cfg(feature = "origin_impl")]
impl OriginDouBanData {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn sub_title(&self) -> Option<&str> {
        self.sub_title.as_deref()
    }

    pub fn img_url(&self) -> &str {
        &self.img_url
    }
}

#[cfg(feature = "origin_impl")]
impl Hash for OriginDouBanData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(feature = "origin_impl")]
impl PartialEq for OriginDouBanData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(feature = "origin_impl")]
impl std::fmt::Display for OriginDouBanData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data[{}-{}]", self.title, self.id)
    }
}

/// 原始数据也可以作为答案
#[cfg(feature = "origin_impl")]
impl Answer for OriginDouBanData {}

#[cfg(feature = "origin_impl")]
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct SeverData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
    pub(crate) img_len: u64,
}

#[cfg(feature = "origin_impl")]
impl SeverData {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn sub_title(&self) -> Option<&str> {
        self.sub_title.as_deref()
    }

    pub fn img_url(&self) -> &str {
        &self.img_url
    }

    pub fn img_len(&self) -> u64 {
        self.img_len
    }
}

#[cfg(feature = "origin_impl")]
impl Hash for SeverData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(feature = "origin_impl")]
impl PartialEq for SeverData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(feature = "origin_impl")]
impl std::fmt::Display for SeverData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data[{}-{}]", self.title, self.id)
    }
}

#[cfg(feature = "origin_impl")]
impl Answer for SeverData {}

/// 将 API 数据，转换为 Answer
#[async_trait]
pub trait IntoAnswer<T>
where
    T: Answer,
{
    type Error: Send;
    /// 将 API 数据，转换为可直接使用的 Answer
    async fn to_answer(self) -> Result<T, Self::Error>;
}

#[cfg(feature = "origin_impl")]
#[async_trait]
impl IntoAnswer<OriginDouBanData> for OriginDouBanData {
    type Error = ();
    async fn to_answer(self) -> Result<OriginDouBanData, Self::Error> {
        Ok(self)
    }
}

#[cfg(feature = "origin_impl")]
lazy_static! {
    static ref HEADERS: reqwest::header::HeaderMap = {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/100.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        h
    };
}

#[cfg(feature = "origin_impl")]
#[async_trait]
impl IntoAnswer<SeverData> for OriginDouBanData {
    type Error = crate::error::Error;
    async fn to_answer(self) -> Result<SeverData, Self::Error> {
        let client = ClientBuilder::new()
            .default_headers(HEADERS.clone())
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::limited(3))
            .build()
            .unwrap_or_default();

        let img_len = client
            .get(self.img_url())
            .send()
            .await?
            .content_length()
            .ok_or(crate::error::OrimplError::ImgLenNotFound)?;

        Ok(SeverData {
            id: self.id,
            title: self.title,
            sub_title: self.sub_title,
            img_url: self.img_url,
            img_len,
        })
    }
}

#[cfg(feature = "origin_impl")]
#[async_trait]
impl IntoAnswer<SeverData> for SeverData {
    type Error = ();
    async fn to_answer(self) -> Result<SeverData, Self::Error> {
        Ok(self)
    }
}
