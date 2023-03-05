//! # 豆瓣数据
//! 定义通用数据格式及实现获取答案
//!
//! # 通用数据格式
//! - 海报信息 包括：链接、日期、图片大小
//! - 选项信息 包括：标题、值
//! - 自建API数据结构 包括：图片大小 及豆瓣API数据
//! - 豆瓣API数据结构 包括：标题、链接、ID
//!
//! ---
//!
//! # 流程
//! ## 1. 获取基本信息
//! - 海报信息
//! - 选项信息
//!
//! ## 2. 获取选项信息方式
//! - 从自建API获取
//! - 从豆瓣API获取
//!
//! ## 3. 获取答案
//! - 通过对比图片大小实现
//! - 通过对比图片相似度实现
//!
//! ## 4. 反馈答案
//! 将获取的所有答案反馈给自建API

use ahash::AHashSet;
use async_trait::async_trait;
use chrono::prelude::*;
use std::hash::Hash;

#[async_trait]
pub trait GetAnswer {}

#[derive(Debug, Eq, Clone)]
pub struct Question {
    pub(crate) poster: Poster,
    pub(crate) opts: AHashSet<Opt>,
}

impl Question {
    pub fn poster(&self) -> &Poster {
        &self.poster
    }

    pub fn opts(&self) -> &AHashSet<Opt> {
        &self.opts
    }
}

impl Hash for Question {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.poster.hash(state);
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.poster == other.poster
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Poster {
    pub(crate) date: NaiveDate,
    pub(crate) url: String,
    pub(crate) img_len: u64,
}

impl Poster {
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn img_len(&self) -> u64 {
        self.img_len
    }
}

impl Hash for Poster {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl PartialEq for Poster {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Opt {
    pub(crate) title: String,
    pub(crate) value: String,
}

impl Opt {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Hash for Opt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}

impl PartialEq for Opt {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DouBanApi;

#[derive(Debug, Clone, Eq)]
pub struct MyApi {
    pub(crate) url: String,
    pub(crate) token: Option<String>,
}

impl MyApi {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

impl Hash for MyApi {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl PartialEq for MyApi {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
