use crate::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

#[async_trait]
pub trait Data {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn sub_title(&self) -> &str;
    async fn img_len(&self) -> Result<usize, Error>;
}

/// 直接可用的豆瓣数据，可以由 RawDouBanData 转换，或者可以从自建API直接获得
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct DouBanData {
    pub(crate) id: String,
    pub(crate) img_len: usize,
    pub(crate) title: String,
    pub(crate) sub_title: String,
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

impl Display for DouBanData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DouBan[{}-{}-{}]", self.title, self.id, self.img_len)
    }
}

#[async_trait]
impl Data for DouBanData {
    fn id(&self) -> &str {
        &self.id
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn sub_title(&self) -> &str {
        &self.sub_title
    }
    async fn img_len(&self) -> Result<usize, Error> {
        Ok(self.img_len)
    }
}

/// 豆瓣API直接返回的数据，不可直接使用，需要转换为 DouBanData
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct RawDouBanData {
    pub(crate) id: String,
    pub(crate) img: String,
    pub(crate) title: String,
    pub(crate) sub_title: String,
}

impl RawDouBanData {
    /// 转换为 DoubanData
    pub async fn get_data(self) -> Result<DouBanData, Error> {
        let img_len = self.get_img_len().await?;
        Ok(DouBanData {
            id: self.id,
            title: self.title,
            sub_title: self.sub_title,
            img_len,
        })
    }

    async fn get_img_len(&self) -> Result<usize, Error> {
        todo!()
    }
}

impl Hash for RawDouBanData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for RawDouBanData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for RawDouBanData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawDouBanData[{}-{}]", self.title, self.id)
    }
}

#[async_trait]
impl Data for RawDouBanData {
    fn id(&self) -> &str {
        &self.id
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn sub_title(&self) -> &str {
        &self.sub_title
    }

    /// 获取一次 img 链接的图片大小，每次调用就会请求一次。
    /// 推荐的方式是转换为DouBanData
    async fn img_len(&self) -> Result<usize, Error> {
        self.get_img_len().await
    }
}
