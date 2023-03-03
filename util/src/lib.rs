use chrono::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    /// 东八区
    pub static ref OFFSET: FixedOffset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
}

/// 获取东八区当前时间
pub fn get_east_eight_now() -> NaiveDateTime {
    let now = DateTime::<FixedOffset>::from_utc(Utc::now().naive_utc(), *OFFSET);
    now.naive_local()
}
