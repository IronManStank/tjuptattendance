//! 用于获取答案的数据结构定义
//!
//! 图片 bytes 用以进行相似度识别
//! 附加信息用以快速识别
//!
//! ## TJUPT签到的题目：
//! - 图片的大小
//! - 图片格式
//! - 图片 bytes
//!
//! ## TJUPT签到的选项
//! - 名字（title）
//! - 值（value）
//!
//! ## 豆瓣数据（影视剧信息） - API与用户或API之间通信的数据结构
//! - 影视剧 ID
//! - 影视剧 标题（title）
//! - 图片链接
//! - 图片格式
//! - 图片 bytes
//! - 附加信息（快速识别-可由客户端自行获取或自建API直接提供）
//!
//! ## 附加信息（快速判断）
//! - 图片大小（需统一格式，目前豆瓣API默认值与TJUPT均为 `Jpeg` 格式，在浏览器中默认为 `WebP` 格式，应该可由 `Headers` 指定）

use crate::{
    error::{DouBanDataError, Error},
    CLIENT,
};
use serde::{Deserialize, Serialize};
use std::{hash::Hash, path::Path};

/// 图片格式
#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ImgFormat {
    /// 默认 Jpeg 仅支持 Jpeg
    #[default]
    Jpeg,
    /// 其他任意格式
    Other,
}

impl ImgFormat {
    /// 检查链接指向的图片格式
    pub(crate) fn check_url(url: &str) -> Result<Self, Error> {
        let url = reqwest::Url::parse(url).map_err(|_e| DouBanDataError::ImgFormatError)?;
        let is_jpeg = url
            .path_segments()
            .and_then(|p| p.rev().next())
            .and_then(|l| Path::new(l).extension())
            .and_then(|p| p.to_str())
            .map(|t| {
                t.contains("jpeg") || t.contains("jpg") || t.contains("Jpeg") || t.contains("Jpg")
            })
            .unwrap_or(false);
        if is_jpeg {
            Ok(Self::Jpeg)
        } else {
            Ok(Self::Other)
        }
    }
}

/// 额外信息，用以快速判断是否为答案
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct AdditionalInfo {
    /// 图片大小 图片类型均为 `Jpeg`
    /// 如果未设置则使用 0
    #[serde(default)]
    pub(crate) img_len: u64,
}

impl AdditionalInfo {
    /// new
    pub fn new(img_len: u64) -> Self {
        Self { img_len }
    }

    /// 是否设置了 additional info
    pub fn is_set(&self) -> bool {
        self.img_len > 0
    }
}

/// API 返回和接受的数据
///
/// ## 注意：
/// - 对比时仅检查 ID
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct DouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
    #[serde(default)]
    pub(crate) additional_info: Option<AdditionalInfo>,
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

impl DouBanData {
    /// 唯一标识，是数字的字符串
    /// eg. `"26647087"`
    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 标题
    #[inline]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// 海报链接
    #[inline]
    pub fn img_url(&self) -> &str {
        &self.img_url
    }

    /// 检查豆瓣链接格式
    pub fn check_img_format(&self) -> Result<bool, Error> {
        let check_res = ImgFormat::check_url(&self.img_url)?;
        Ok(matches!(check_res, ImgFormat::Jpeg))
    }

    /// 图片大小
    ///
    /// 如果为 0 则未设置
    #[inline]
    pub fn img_len(&self) -> u64 {
        self.additional_info
            .map(|info| info.img_len)
            .unwrap_or_default()
    }

    /// 是否具有 额外信息
    #[inline]
    pub fn have_additional_info(&self) -> bool {
        // 如果是豆瓣API
        self.additional_info
            .map(|info| info.is_set())
            .unwrap_or(false)
    }

    /// 获取 内容大小
    ///
    /// 如果未设置 Content-Length 则为 `None`
    async fn get_content_length(url: &str) -> Result<Option<u64>, Error> {
        Ok(CLIENT.get(url).send().await?.content_length())
    }

    /// 获取图片大小
    ///
    /// ## 注意：
    /// - 此方法未检查是否 `hava_len` 只会强制覆盖 `img_len`
    /// - 请调用前判断是否需要
    pub async fn set_len(&mut self) -> Result<(), Error> {
        let img_len = Self::get_content_length(&self.img_url)
            .await?
            .ok_or(Error::Data(crate::error::DouBanDataError::ImgLenNotFund))?;
        self.additional_info = Some(AdditionalInfo { img_len });
        Ok(())
    }

    /// 获取图片大小，并返回一个新的 Answer
    ///
    /// ## 注意：
    /// - 此方法未检查是否 `hava_len` 只会强制覆盖 `img_len`
    /// - 请调用前判断是否需要
    pub async fn get_len(self) -> Result<Self, Error> {
        Ok(Self {
            additional_info: Some(AdditionalInfo {
                img_len: self.ask_img_len().await?,
            }),
            ..self
        })
    }

    /// 获取图片大小，并返回一个新的 Answer
    ///
    /// ## 注意：
    /// - 此方法未检查是否 `hava_len` 只会强制覆盖 `img_len`
    /// - 请调用前判断是否需要
    pub async fn get_len_try(self) -> Self {
        if let Ok(Some(img_len)) = Self::get_content_length(&self.img_url).await {
            Self {
                additional_info: Some(AdditionalInfo { img_len }),
                ..self
            }
        } else {
            self
        }
    }

    /// 尝试获取图片大小
    ///
    /// ## 注意：
    /// - 此方法未检查是否 `hava_len` 只会强制覆盖 `img_len`
    /// - 请调用前判断是否需要
    pub async fn set_len_try(&mut self) {
        if let Ok(Some(img_len)) = Self::get_content_length(&self.img_url).await {
            self.additional_info = Some(AdditionalInfo { img_len });
        }
    }

    /// 谨慎使用：每次调用都会访问 Web 一次
    pub async fn ask_img_len(&self) -> Result<u64, Error> {
        Self::get_content_length(&self.img_url)
            .await?
            .ok_or(Error::Data(crate::error::DouBanDataError::ImgLenNotFund))
    }
}

impl std::fmt::Display for DouBanData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.title)
    }
}

/// 测试豆瓣数据
#[cfg(test)]
mod test_answer {
    use std::collections::HashSet;

    use super::*;
    use pretty_assertions::assert_eq;

    /// 测试获取图片大小
    #[tokio::test]
    async fn test_get_img_len() {
        let url = "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg";
        let mut santi = DouBanData {
            id: "26647087".into(),
            img_url: url.into(),
            title: "三体".into(),
            additional_info: None,
        };

        santi.set_len().await.unwrap();

        assert_eq!(santi.additional_info.unwrap().img_len, 17075);

        let img_len = DouBanData::get_content_length(url).await.unwrap();
        assert_eq!(img_len, Some(17075));
    }

    /// 测试 Answer Hash
    #[test]
    fn test_hash_answer() {
        let url = "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.webp";
        let mut map = HashSet::new();
        map.insert(DouBanData {
            id: "26647087".into(),
            img_url: url.into(),
            title: "三体".into(),
            additional_info: None,
        });
        map.insert(DouBanData {
            id: "26647086".into(),
            img_url: url.into(),
            title: "三体".into(),
            additional_info: None,
        });
        assert_eq!(map.len(), 2);
        map.insert(DouBanData {
            id: "26647087".into(),
            img_url: "url.into()".into(),
            title: "三体1".into(),
            additional_info: Some(AdditionalInfo { img_len: 1 }),
        });
        assert_eq!(map.len(), 2);
    }

    /// 测试：通过链接判断是否为 `Jpeg`
    #[test]
    fn test_imgformat_by_url() {
        let url = "https://dioubad.com/img/213.jpg";
        assert_eq!(ImgFormat::Jpeg, ImgFormat::check_url(url).unwrap());

        let url = "https://dioubad.com/img/213.jpeg";
        assert_eq!(ImgFormat::Jpeg, ImgFormat::check_url(url).unwrap());

        let url = "https://dioubad.com/img/213.Jpeg";
        assert_eq!(ImgFormat::Jpeg, ImgFormat::check_url(url).unwrap());

        let url = "https://dioubad.com/img/213.Jpg";
        assert_eq!(ImgFormat::Jpeg, ImgFormat::check_url(url).unwrap());

        let data = DouBanData {
            id: "26647087".into(),
            img_url: "https://dioubad.com/img/213.webp".into(),
            title: "三体".into(),
            additional_info: None,
        };
        assert!(data.check_img_format().unwrap() == false);
    }
}
