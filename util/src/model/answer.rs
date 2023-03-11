//! 用于获取答案的数据结构定义

use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::{error::Error, CLIENT};

/// API 返回和接受的数据
///
/// ## 注意：
/// - 对比时仅检查 ID
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct Answer {
    pub(crate) id: String,
    pub(crate) title: String,
    #[serde(rename = "img")]
    pub(crate) img_url: String,
    #[serde(default)]
    pub(crate) img_len: u64,
}

impl Hash for Answer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Answer {
    /// 唯一标识，是字符串的数字
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

    /// 图片大小
    #[inline]
    pub fn img_len(&self) -> u64 {
        self.img_len
    }

    /// 是否具有 img_len
    #[inline]
    pub fn have_len(&self) -> bool {
        self.img_len > 0
    }

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
            .ok_or(Error::Answer(crate::error::AnswerError::ImgLenNotFund))?;
        self.img_len = img_len;
        Ok(())
    }

    /// 获取图片大小，并返回一个新的 Answer
    ///
    /// ## 注意：
    /// - 此方法未检查是否 `hava_len` 只会强制覆盖 `img_len`
    /// - 请调用前判断是否需要
    pub async fn get_len(self) -> Result<Self, Error> {
        Ok(Self {
            img_len: self.ask_img_len().await?,
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
            Self { img_len, ..self }
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
            self.img_len = img_len;
        }
    }

    /// 每次调用都会访问Web一次
    pub async fn ask_img_len(&self) -> Result<u64, Error> {
        Self::get_content_length(&self.img_url)
            .await?
            .ok_or(Error::Answer(crate::error::AnswerError::ImgLenNotFund))
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.title)
    }
}

/// test Answer
#[cfg(test)]
mod test_answer {
    use std::collections::HashSet;

    use super::*;
    use pretty_assertions::assert_eq;

    /// 测试获取图片大小
    #[tokio::test]
    async fn test_get_img_len() {
        let url = "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg";
        let mut santi = Answer {
            id: "26647087".into(),
            img_url: url.into(),
            title: "三体".into(),
            img_len: 0,
        };

        santi.set_len().await.unwrap();

        assert_eq!(santi.img_len, 17075);

        let img_len = Answer::get_content_length(url).await.unwrap();
        assert_eq!(img_len, Some(17075));
    }

    /// 测试 Answer Hash
    #[test]
    fn test_hash_answer() {
        let url = "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.webp";
        let mut map = HashSet::new();
        map.insert(Answer {
            id: "26647087".into(),
            img_url: url.into(),
            title: "三体".into(),
            img_len: 0,
        });
        map.insert(Answer {
            id: "26647086".into(),
            img_url: url.into(),
            title: "三体".into(),
            img_len: 0,
        });
        assert_eq!(map.len(), 2);
        map.insert(Answer {
            id: "26647087".into(),
            img_url: "url.into()".into(),
            title: "三体1".into(),
            img_len: 1,
        });
        assert_eq!(map.len(), 2);
    }
}
