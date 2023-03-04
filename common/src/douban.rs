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
#[cfg(feature = "origin_impl")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

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

pub trait DouBanData: Sized + Debug + Hash + PartialEq + Eq + Clone {}

#[cfg(feature = "origin_impl")]
impl DouBanData for OriginDouBanData {}

/// 服务器储存的豆瓣数据
pub trait Answer: Sized + Debug + Hash + PartialEq + Eq + Clone {}

/// 原始数据也可以作为答案
#[cfg(feature = "origin_impl")]
impl Answer for OriginDouBanData {}

#[async_trait]
pub trait IntoAnswer: Sized + Debug + Hash + PartialEq + Eq + Clone {
    type InputData: DouBanData;
    type OutputData: Answer;
    type Error;

    async fn get_doubandata(self) -> Result<Self::OutputData, Self::Error>;
}

// OriginDouBanData 既可以作为 DouBanData 也可以作为 Answer
#[cfg(feature = "origin_impl")]
#[async_trait]
impl IntoAnswer for OriginDouBanData {
    type InputData = OriginDouBanData;
    type OutputData = OriginDouBanData;
    type Error = ();

    /// 直接返回自身，必定不会失败
    async fn get_doubandata(self) -> Result<Self::OutputData, Self::Error> {
        Ok(self)
    }
}
