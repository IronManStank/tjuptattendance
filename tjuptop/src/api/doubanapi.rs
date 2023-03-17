//! 豆瓣api

use serde::{Deserialize, Serialize};
use util::{model::doubandata::DouBanData, CLIENT};

use crate::error::Error;

/// 豆瓣 API
///
/// 默认获取的海报链接图片格式为 `Jpeg` 与 JUTPT 提供的格式相同
#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct DouBanApi;

impl DouBanApi {
    /// 获取豆瓣API的数据，不包括 img_len
    pub async fn get_answer(title: &str) -> Result<Vec<DouBanData>, Error> {
        Ok(CLIENT
            .get("https://movie.douban.com/j/subject_suggest")
            .query(&[("q", title)])
            .send()
            .await?
            .json()
            .await?)
    }

    /// 尝试将无 `img_len` 的 `Answer` 转化为有 `img_len` 的 Answer。并且，保持原有顺序
    pub async fn get_more_info_try<T>(lst: T) -> Vec<DouBanData>
    where
        T: IntoIterator<Item = DouBanData>,
    {
        let mut answer = Vec::with_capacity(6);
        for i in lst.into_iter() {
            answer.push(i.get_len_try().await);
        }
        answer
    }
}

impl std::fmt::Display for DouBanApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DouBanAPI")
    }
}
