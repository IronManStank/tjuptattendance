use std::hash::Hash;
use serde::{Deserialize, Serialize};

/// 真正使用的豆瓣数据，不是原始数据
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct DouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    pub(crate) img: String,
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

/// 豆瓣API解析豆瓣数据
#[derive(Debug, Serialize, Deserialize, Eq, Clone)]
pub struct RawDouBanData {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) sub_title: Option<String>,
    pub(crate) img: String,
}

impl RawDouBanData {
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
