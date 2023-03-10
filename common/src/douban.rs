//! ## 豆瓣数据
//! - 从豆瓣API获取
//! - 从自建API获取

use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

use crate::error::{DouBanDataError, Error};
use ahash::AHashSet;
use anyhow::anyhow;
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct DataStore {
    pub(crate) inner: AHashSet<DouBanData>,
}

impl DataStore {
    pub fn content(&self) -> &AHashSet<DouBanData> {
        &self.inner
    }

    async fn try_update_img(&mut self) {
        let mut set = AHashSet::default();
        let client = get_client();

        for data in self.inner.drain() {
            match data.get_img_len(&client).await {
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
                Ok(d) => {
                    set.insert(d);
                }
            }
        }
        self.inner = set;
    }

    pub async fn report_to_api(&mut self, api: &Api) -> Result<(), Error> {
        if api.report() && !self.inner.is_empty() {
            self.try_update_img().await;
            let client = get_client();
            let res = client
                .post(&api.url)
                .query(&[("q", api.token())])
                .json(&self.inner)
                .send()
                .await?;
            if res.status().is_success() {
                println!("成功向 {} 汇报 {} 个结果", api, self.inner.len());
            }
            Ok(())
        } else {
            eprintln!("{api} 未开启 report 或 无可汇报数据");
            Ok(())
        }
    }
}

impl std::ops::Deref for DataStore {
    type Target = AHashSet<DouBanData>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct DouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    /// 海报链接
    #[serde(rename = "img")]
    pub(crate) img_url: String,
    #[serde(default)]
    pub(crate) img_len: u64,
}

impl DouBanData {
    /// 获取图片大小，如果失败则不改变
    pub async fn get_img_len(self, client: &Client) -> Result<Self, Error> {
        if self.img_len != 0 {
            Ok(self)
        } else {
            // let client = get_client();
            let Some(img_len) = client.get(&self.img_url).send().await?.content_length() else {
                // 如果失败就返回自身
                return Ok(self);
            };
            Ok(Self { img_len, ..self })
        }
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

impl std::fmt::Display for DouBanData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct Api {
    pub(crate) url: String,
    pub(crate) token: Option<String>,
    pub(crate) report: Option<bool>,
}

impl Api {
    const DOUBAN_API: &str = "https://movie.douban.com/j/subject_suggest";

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn new_doubanapi() -> Self {
        Self {
            url: Self::DOUBAN_API.into(),
            token: None,
            report: None,
        }
    }

    pub fn report(&self) -> bool {
        self.report.unwrap_or(false)
    }

    pub async fn get_data(
        &self,
        title: &str,
        data_store: Arc<Mutex<DataStore>>,
    ) -> Result<DouBanData, Error> {
        let client = get_client();

        let data_vec: Vec<DouBanData> = client
            .get(&self.url)
            .query(&[("q", Some(title)), ("t", self.token())])
            .send()
            .await?
            .json()
            .await?;

        // 返回API数据
        if self.report() && !data_vec.is_empty() {
            let Ok(mut lock) = data_store.lock() else {
                return Err(anyhow!("内部错误").into());
            };
            lock.inner
                .extend(data_vec.clone().into_iter().filter(|d| d.img_len == 0));
        }

        let Some(data) = data_vec.into_iter().next() else {
            return Err(DouBanDataError::ApiTired.into());
        };
        if data.img_len == 0 {
            Ok(data.get_img_len(&client).await?)
        } else {
            // 从自建API获取的数据
            Ok(data)
        }
    }
}

impl Hash for Api {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl PartialEq for Api {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
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

// 一个简单的客户端
pub fn get_client() -> Client {
    ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(5))
        .default_headers(HEADERS.clone())
        .build()
        .unwrap_or_default()
}

#[cfg(test)]
mod test_douban_api {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use tokio;

    #[tokio::test]
    async fn test_get_data() {
        let store = Arc::new(Mutex::new(DataStore::default()));
        let data_origin = DouBanData {
            id: "26647087".into(),
            img_url: "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg"
                .into(),
            title: "三体".into(),
            img_len: 17075,
        };
        let data_douban = Api::new_doubanapi()
            .get_data(&data_origin.title, store.clone())
            .await
            .unwrap();
        // DouBanData 实现了 Eq，所以不能直接比较，而是比较其 Debug 的字符
        assert_str_eq!(format!("{:#?}", data_origin), format!("{:#?}", data_douban));

        // 测试获取图片大小
        let data_no_len = DouBanData {
            id: "26647087".into(),
            img_url: "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg"
                .into(),
            title: "三体".into(),
            img_len: 0,
        };
        let clinet = get_client();
        let data_with_len = data_no_len.get_img_len(&clinet).await.unwrap();
        assert_str_eq!(
            format!("{:#?}", data_origin),
            format!("{:#?}", data_with_len)
        );

        // 测试Myapi
        let myapi = Api {
            url: "https://movie.douban.com/j/subject_suggest".into(),
            token: None,
            report: Some(true),
        };
        let data = myapi.get_data("三体", store.clone()).await.unwrap();
        assert_eq!(data.id, data_origin.id);
        {
            let mut lock = store.lock().unwrap();
            lock.try_update_img().await;
            assert!(lock.content().contains(&data_origin));

            // TODO 测试反馈功能
        }
    }
}
