//! TJUPT 签到题目中的海报

use crate::error::{DouBanDataError, Error};

use super::doubandata::{AdditionalInfo, ImgFormat};
use anyhow::anyhow;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// TJUPT 签到题目海报
#[derive(Debug, Serialize, Deserialize)]
pub struct Poster {
    pub(crate) url: String,
    pub(crate) bytes: Option<Bytes>,
    #[serde(default)]
    pub(crate) additional_info: Option<AdditionalInfo>,
}

impl Poster {
    /// 是否可用于快速判断答案
    pub fn is_quick_check(&self) -> bool {
        self.additional_info
            .map(|info| info.is_set())
            .unwrap_or(false)
    }

    /// 图片大小
    pub fn img_len(&self) -> u64 {
        self.additional_info
            .map(|info| info.img_len)
            .unwrap_or_default()
    }

    /// 创建一个 Poster
    pub fn new(
        url: String,
        bytes: Option<Bytes>,
        additional_info: Option<AdditionalInfo>,
    ) -> Result<Self, Error> {
        if bytes.is_none() & additional_info.map(|info| !info.is_set()).unwrap_or(true) {
            Err(anyhow!("无可用信息的 Poster {}", url).into())
        } else if !ImgFormat::check_url(&url).map(|t| matches!(t, ImgFormat::Jpeg))? {
            Err(DouBanDataError::ImgFormatError.into())
        } else {
            Ok(Self {
                url,
                bytes,
                additional_info,
            })
        }
    }
}

impl<T: AsRef<str>> From<T> for Poster {
    fn from(value: T) -> Self {
        Poster {
            url: value.as_ref().into(),
            bytes: None,
            additional_info: None,
        }
    }
}

#[cfg(test)]
mod test_poster {
    use super::*;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_poster_new() {
        let poster = Poster::new(
            "https://douban.com/asd.jpg".into(),
            None,
            Some(AdditionalInfo { img_len: 123 }),
        );
        assert!(poster.is_ok());
        let poster = poster.unwrap();
        assert!(poster.is_quick_check());

        let poster = Poster::new("https://douban.com/asd.jpg".into(), None, None);
        assert!(matches!(poster, Err(Error::Other(_))));
    }
}
