use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct DouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    pub(crate) img: String,
    // 由于豆瓣API未提供此属性，反序列化时设为0
    #[serde(default)]
    pub(crate) img_len: u64,
}

impl DouBanData {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn sub_title(&self) -> Option<&str> {
        self.sub_title.as_deref()
    }

    pub fn img(&self) -> &str {
        &self.img
    }

    pub fn img_len(&self) -> u64 {
        self.img_len
    }

    /// 是否为豆瓣API返回的原始数据
    /// 如果是，则需要获取一次
    pub fn is_raw(&self) -> bool {
        self.img_len == 0
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
        write!(f, "Data[{}-{}-{}]", self.title, self.id, self.img_len)
    }
}
