//! ## 豆瓣数据
//! - 从豆瓣API获取
//! - 从自建API获取

use std::hash::Hash;

use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};

use crate::error::{DouBanDataError, Error};

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
    /// 获取图片大小
    pub async fn get_img_len(self, client: &Client) -> Result<Self, Error> {
        if self.img_len != 0 {
            Ok(self)
        } else {
            // let client = get_client();
            let Some(img_len) = client.get(&self.img_url).send().await?.content_length() else {
                return Err(DouBanDataError::ImgLenNotFound.into());
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

    pub async fn get_data(title: &str) -> Result<DouBanData, Error> {
        println!("{title}");
        todo!()
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

/// 从自定义API获取
pub async fn get_data_by_my_api(api: &Api, title: &str) -> Result<DouBanData, Error> {
    let client = get_client();
    let data_vec: Vec<DouBanData> = client
        .get(&api.url)
        .query(&[("q", Some(title)), ("t", api.token())])
        .send()
        .await?
        .json()
        .await?;

    // TODO 可以在此返回数据
    if api.report() && data_vec.get(0).map(|d| d.img_len != 0).unwrap_or(false) {
        // 返回API数据
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
        let data_origin = DouBanData {
            id: "26647087".into(),
            img_url: "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg"
                .into(),
            title: "三体".into(),
            img_len: 17075,
        };
        let data = get_data_by_my_api(&Api::new_doubanapi(), "三体")
            .await
            .unwrap();
        // DouBanData 实现了 Eq，所以不能直接比较，而是比较其 Debug 的字符
        assert_str_eq!(format!("{:#?}", data_origin), format!("{:#?}", data));

        // 测试获取图片大小
        let data_1 = DouBanData {
            id: "26647087".into(),
            img_url: "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg"
                .into(),
            title: "三体".into(),
            img_len: 0,
        };
        let clinet = get_client();
        let data_1 = data_1.get_img_len(&clinet).await.unwrap();
        assert_str_eq!(format!("{:#?}", data_origin), format!("{:#?}", data_1));

        // 测试Myapi
        let myapi = Api {
            url: "https://movie.douban.com/j/subject_suggest".into(),
            token: None,
            report: None,
        };
        let data = get_data_by_my_api(&myapi, "三体").await.unwrap();
        assert_eq!(data.id, data_origin.id);
    }
}
