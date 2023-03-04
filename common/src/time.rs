use chrono::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    /// 东八区 offset
    pub static ref OFFSET:FixedOffset = FixedOffset::east_opt(8*60*60).unwrap();
}

/// 东八区时间
pub fn get_now() -> NaiveDateTime {
    DateTime::<FixedOffset>::from_utc(Utc::now().naive_utc(), *OFFSET).naive_local()
}
