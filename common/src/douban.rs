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

use anyhow::anyhow;
use async_trait::async_trait;
use chrono::prelude::*;
use lazy_static::lazy_static;

use reqwest::{Client, ClientBuilder};

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

use crate::error::{DouBanDataError, Error};
use crate::time::get_now;

const IMG_LEN_DIFF: u64 = 6;
const DOUBAN_SUGGEST_URL: &str = "https://movie.douban.com/j/subject_suggest";

pub trait Poster: Debug + Hash + Eq + Clone + Send + Sized {
    /// 检查是否可用
    fn is_available(&self) -> bool;
}

/// 将 API 数据，转换为 Answer
#[async_trait]
pub trait IntoAnswer<T>: Debug + Hash + Eq + Clone + Send + Sized
where
    T: Answer,
{
    type Error: Send;
    /// 将 API 数据，转换为可直接使用的 Answer
    async fn to_answer(self) -> Result<T, Self::Error>;
}

/// 可以直接使用的数据
pub trait Answer: Debug + Hash + Eq + Clone + Send + Sized {
    type Poster: Poster;
    /// 检查是否为答案
    fn is_answer(&self, poster: Arc<Self::Poster>) -> bool;
}

#[async_trait]
pub trait Api<A: Answer, T: IntoAnswer<A>>: Sized + Eq + Hash + Send {
    type Output: IntoAnswer<A>;
    type Error;

    async fn get_answer(&self, title: String) -> Result<Self::Output, Self::Error>;
}

pub trait Question<P, A, D, M>: Sized + Eq + Hash + Send
where
    D: Answer,
    P: Poster,
    A: IntoAnswer<D>,
    M: Api<D, A>,
{
}

/// - url: GET `https://movie.douban.com/j/subject_suggest?q=三体`
/// - return JSON `[OriginDouBanData, ...]`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DouBanApi;

/// 自建API
/// - url: POST JSON
/// - retrun JSON `[SeverData, ...]`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MyUpStreamApi {
    pub(crate) url: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MixedApi {
    pub(crate) url: String,
}

impl MyUpStreamApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait]
impl Api<SeverData, OriginDouBanData> for MixedApi {
    type Output = SeverData;
    type Error = Error;

    async fn get_answer(&self, title: String) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl Api<SeverData, OriginDouBanData> for DouBanApi {
    type Output = SeverData;
    type Error = Error;
    async fn get_answer(&self, title: String) -> Result<Self::Output, Self::Error> {
        let client = get_data_client();
        let res: Vec<OriginDouBanData> = client
            .get(DOUBAN_SUGGEST_URL)
            .query(&[("q", &title)])
            .send()
            .await?
            .json()
            .await?;
        let r = res.into_iter().nth(0).ok_or(DouBanDataError::ApiTired)?;
        let r = r.to_answer().await?;
        Ok(r)
    }
}

#[async_trait]
impl Api<SeverData, SeverData> for MyUpStreamApi {
    type Output = SeverData;
    type Error = Error;

    async fn get_answer(&self, title: String) -> Result<Self::Output, Self::Error> {
        println!("{title}");
        todo!()
    }
}

#[derive(Debug, Eq, Clone)]
pub struct TjuPoster {
    pub(crate) date: NaiveDate,
    // url 包含日期与随机字符
    pub(crate) url: reqwest::Url,
    pub(crate) img_len: u64,
}

impl Hash for TjuPoster {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl PartialEq for TjuPoster {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl TjuPoster {
    // 必须在用户登陆后获取
    pub fn new(url: String, img_len: u64) -> Result<Self, Error> {
        let url = reqwest::Url::parse(&url).map_err(|e| anyhow!("无法解析海报链接 {e}"))?;
        // 通过解析 url 链接来获取poster可用日期
        let Some(d) = url.path_segments().map(|c| c.collect::<Vec<_>>()).and_then(|lst| lst.get(1).copied() ) else {
            return Err(Error::Other(anyhow!("无法解析海报链接")));
        };
        let date = NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .map_err(|e| anyhow!("无法获取海报日期 {e}"))?;

        // https://tjupt.org/assets/2023-03-05/asdqweQ.jpg
        Ok(Self { date, url, img_len })
    }
}

impl Poster for TjuPoster {
    fn is_available(&self) -> bool {
        // 检查poster是否过期
        get_now().date() == self.date
    }
}

// 此数据应该与豆瓣API同步更新 (虽然不大可能)
/// 豆瓣API提供的原始数据

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct OriginDouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
}

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

impl Hash for OriginDouBanData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for OriginDouBanData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for OriginDouBanData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data[{}-{}]", self.title, self.id)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct SeverData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
    pub(crate) img_len: u64,
}

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

impl Hash for SeverData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for SeverData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for SeverData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data[{}-{}-{}]", self.title, self.id, self.img_len)
    }
}

impl Answer for SeverData {
    type Poster = TjuPoster;
    /// 海报与答案相差6bytes
    fn is_answer(&self, poster: Arc<Self::Poster>) -> bool {
        poster.is_available() && self.img_len.abs_diff(poster.img_len) == IMG_LEN_DIFF
    }
}

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

#[async_trait]
impl IntoAnswer<SeverData> for SeverData {
    type Error = Error;
    /// 直接返回自身，不会失败
    async fn to_answer(self) -> Result<SeverData, Self::Error> {
        Ok(self)
    }
}

#[async_trait]
impl IntoAnswer<SeverData> for OriginDouBanData {
    type Error = Error;
    async fn to_answer(self) -> Result<SeverData, Self::Error> {
        let client = get_data_client();

        let img_len = client
            .get(self.img_url())
            .send()
            .await?
            .content_length()
            .ok_or(crate::error::DouBanDataError::ImgLenNotFound)?;

        Ok(SeverData {
            id: self.id,
            title: self.title,
            sub_title: self.sub_title,
            img_url: self.img_url,
            img_len,
        })
    }
}

pub(crate) fn get_data_client() -> Client {
    let client = ClientBuilder::new()
        .default_headers(HEADERS.clone())
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()
        .unwrap_or_default();
    client
}

#[cfg(test)]
mod doubandata_test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;
    use tokio_test::block_on;

    #[test]
    fn is_answer_test() {
        let sdata = block_on(async {
            let ori = OriginDouBanData {
                id: "id".into(),
                title: "title".into(),
                sub_title: None,
                img_url: "https://www.3moredays.com/assets/img/thumb.png".into(),
            };
            let sev = ori.to_answer().await.unwrap();
            sev
        });

        let poster = Arc::new(
            TjuPoster::new(
                format!(
                    "https://tjupt.org/assets/{}/asdqweQ.jpg",
                    get_now().date().format("%Y-%m-%d")
                ),
                396424 + IMG_LEN_DIFF,
            )
            .unwrap(),
        );

        assert_eq!(sdata.img_len(), 396424);
        assert!(sdata.is_answer(poster));
    }
}
