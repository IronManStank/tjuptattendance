//! # 豆瓣数据
//! 定义通用数据格式及实现获取答案
//!
//! # 通用数据格式
//! - 海报信息 包括：链接、日期、图片大小
//! - 选项信息 包括：标题、值
//! - 自建API数据结构 包括：图片大小 及豆瓣API数据
//! - 豆瓣API数据结构 包括：标题、链接、ID
//!
//! ---
//!
//! # 流程
//! ## 1. 获取基本信息
//! - 海报信息
//! - 选项信息
//!
//! ## 2. 获取选项信息方式
//! - 从自建API获取
//! - 从豆瓣API获取
//!
//! ## 3. 获取答案
//! - 通过对比图片大小实现
//! - 通过对比图片相似度实现
//!
//! ## 4. 反馈答案
//! 将获取的所有答案反馈给自建API

use crate::error::{DouBanDataError, Error};
use ahash::AHashSet;
use chrono::prelude::*;
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use tokio::task::JoinSet;

#[derive(Debug, Eq, Clone)]
pub struct Question {
    pub(crate) poster: Poster,
    pub(crate) opts: AHashSet<Opt>,
}

impl Question {
    pub fn new<T: IntoIterator<Item = Opt>>(poster: Poster, opts_iter: T) -> Self {
        Self {
            poster,
            opts: AHashSet::from_iter(opts_iter),
        }
    }

    #[inline]
    pub fn opts_titles(&self) -> Vec<&str> {
        Vec::from_iter(self.opts.iter().map(|opt| opt.title()))
    }

    #[allow(unused)]
    async fn get_answer<T: IntoIterator<Item = MyApi>>(
        &self,
        douban: bool,
        myapis: T,
        report: bool,
    ) -> Result<DouBanData, Error> {
        let mut set = JoinSet::default();
        let mut useless = AHashSet::default();

        for api in myapis {
            for opt in self.opts.clone().into_iter() {
                let api = api.clone();
                set.spawn(async move { api.get_data(opt.title()).await });
            }
        }

        if douban {
            for opt in self.opts.clone().into_iter() {
                set.spawn(async move { DouBanApi::get_data(opt.title()).await });
            }
        }

        while let Some(res) = set.join_next().await {
            match res {
                Ok(Ok(data)) => {
                    if self.poster.is_answer(&data) {
                        return Ok(data);
                    } else {
                        useless.insert(data);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("{e}");
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
        }

        // 可以在这里向API返回结果们
        if report {
            println!("已经尝试了 {} 个", useless.len());
        }

        Err(DouBanDataError::NotAnswer.into())
    }

    pub fn get_value(&self, title: &str) -> Option<&Opt> {
        self.opts.get(&Opt {
            title: title.into(),
            value: "".into(),
        })
    }

    pub fn poster(&self) -> &Poster {
        &self.poster
    }

    pub fn opts(&self) -> &AHashSet<Opt> {
        &self.opts
    }
}

impl Hash for Question {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.poster.hash(state);
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.poster == other.poster
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Poster {
    pub(crate) date: NaiveDate,
    pub(crate) url: String,
    pub(crate) img_len: u64,
}

impl Poster {
    pub fn is_answer(&self, data: &DouBanData) -> bool {
        self.img_len.abs_diff(data.img_len) == 6
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn img_len(&self) -> u64 {
        self.img_len
    }
}

impl Hash for Poster {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl PartialEq for Poster {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Opt {
    pub(crate) title: String,
    pub(crate) value: String,
}

impl Opt {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Hash for Opt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}

impl PartialEq for Opt {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DouBanApi;

impl DouBanApi {
    const DOUBAN_API_URL: &str = "https://movie.douban.com/j/subject_suggest";
    pub async fn get_data(title: &str) -> Result<DouBanData, Error> {
        let client = get_client();
        let res: Vec<DouBanData> = client
            .get(Self::DOUBAN_API_URL)
            .query(&[("q", title)])
            .send()
            .await?
            .json()
            .await?;
        match res.into_iter().nth(0) {
            None => Err(DouBanDataError::ApiTired.into()),
            Some(data) => {
                let img_len = client.get(&data.img_url).send().await?.content_length();
                match img_len {
                    None => Err(DouBanDataError::ImgLenNotFound.into()),
                    Some(img_len) => Ok(data.set_img_len(img_len)),
                }
            }
        }
    }

    // pub async fn get_data(opt_titles: Vec<String>) -> Result<Vec<DouBanData>, Error> {
    //     let mut set = JoinSet::new();
    //     let mut result = Vec::with_capacity(6);

    //     opt_titles.into_iter().for_each(|title| {
    //         set.spawn(Self::get_one(title));
    //     });

    //     while let Some(rres) = set.join_next().await {
    //         match rres {
    //             Ok(Ok(data)) => {
    //                 result.push(data);
    //             }
    //             Ok(Err(e)) => {
    //                 eprintln!("无法从豆瓣API获取豆瓣数据 {e}");
    //             }
    //             Err(e) => {
    //                 eprintln!("内部错误 {e}");
    //             }
    //         }
    //     }

    //     Ok(result)
    // }
}

#[derive(Debug, Clone, Eq)]
pub struct MyApi {
    pub(crate) url: String,
    pub(crate) token: Option<String>,
}

impl MyApi {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn token(&self) -> &str {
        match self.token {
            Some(ref s) => s,
            None => "None",
        }
    }

    #[allow(unused)]
    pub async fn get_data(&self, opt: &str) -> Result<DouBanData, Error> {
        let res: Vec<DouBanData> = get_client()
            .get(&self.url)
            .query(&[("token", self.token()), ("q", opt)])
            .send()
            .await?
            .json()
            .await?;
        match res.into_iter().nth(0) {
            None => Err(DouBanDataError::ApiTired.into()),
            Some(data) => Ok(data),
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

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct DouBanData {
    pub(crate) id: String,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) img_len: u64,
}

impl DouBanData {
    pub fn set_img_len(self, img_len: u64) -> Self {
        Self { img_len, ..self }
    }
}

impl Hash for DouBanData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for DouBanData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut h = HeaderMap::default();
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

pub(crate) fn get_client() -> Client {
    let client = ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(5))
        .default_headers(HEADERS.clone())
        .build()
        .unwrap_or_default();
    client
}

#[cfg(test)]
mod api_test {
    use super::*;
    use crate::time::get_now;

    #[tokio::test]
    async fn doubanapi_test() {
        let poster = Poster {
            date: get_now().date(),
            url: "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg"
                .into(),
            img_len: 17075 - 6,
        };
        let opt = Opt {
            title: "三体".into(),
            value: "".into(),
        };
        let q = Question::new(
            poster,
            [
                opt.clone(),
                Opt {
                    title: "嘻嘻".into(),
                    value: "".into(),
                },
            ],
        );

        let res = q.get_answer(true, [], false).await.unwrap();

        assert_eq!(
            res,
            DouBanData {
                id: "26647087".into(),
                img_url: "url".into(),
                title: "三体".into(),
                img_len: 345
            }
        );

        assert_eq!(&opt, q.get_value("三体").unwrap());
    }

    #[tokio::test]
    /// 测试豆瓣API能否正确获取豆瓣数据
    /// 如果频繁调用会失败，但是成功一次就可以
    async fn ask_douban_api_test() {
        let data = DouBanData {
            id: "26647087".into(),
            img_url: "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg"
                .into(),
            title: "三体".into(),
            img_len: 17075,
        };
        let apidata = DouBanApi::get_data(&data.title).await.unwrap();
        assert_eq!(apidata, data,);
    }
}
