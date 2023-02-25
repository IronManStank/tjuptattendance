use chrono::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OFFECT: FixedOffset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
}

/// 获取东八区时间
pub fn get_now() -> NaiveDateTime {
    let now = DateTime::<FixedOffset>::from_utc(Utc::now().naive_utc(), *OFFECT);
    now.naive_local()
}
