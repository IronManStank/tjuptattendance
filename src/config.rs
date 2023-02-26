use crate::{get_now, User};
use ahash::AHashSet;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub(crate) users: AHashSet<User>,
    pub(crate) top: AHashSet<NaiveTime>,
}

impl ConfigFile {
    pub fn get_next_top_time_point(&self) -> Option<NaiveDateTime> {
        let now = get_now();
        // 排序
        let mut lst = Vec::from_iter(self.top.iter());
        lst.sort();
        let first = lst.get(0).copied();
        let next_point = lst.into_iter().find(|&&t| now.time() < t);
        if next_point.is_some() {
            next_point.map(|&t| NaiveDateTime::new(now.date(), t))
        } else {
            first.map(|&t| NaiveDateTime::new(now.date() + chrono::Duration::days(1), t))
        }
    }
}
