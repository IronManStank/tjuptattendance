//! 123

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod api;
pub mod bot;
pub mod error;
pub mod user;

use chrono::prelude::*;

/// 等待至时间点
pub async fn wait_to_point(target: Option<NaiveDateTime>) {
    if let Some(dt) = target {
        let dur = (dt - util::get_now()).abs();
        if !dur.is_zero() {
            tokio::time::sleep(dur.to_std().unwrap_or_default()).await;
        }
    }
}

#[cfg(test)]
mod test_tjuptop_util {
    use super::*;

    /// 测试：等待到时间点
    #[tokio::test]
    async fn test_wait_to_point() {
        let target = util::get_now() + chrono::Duration::seconds(1);
        let ins = tokio::time::Instant::now();
        wait_to_point(Some(target)).await;
        let dur = ins.elapsed().as_secs_f32();
        dbg!(dur);
        assert!((dur - 1.0).abs() <= 0.1);

        let ins = tokio::time::Instant::now();
        wait_to_point(None).await;
        let dur = ins.elapsed();
        dbg!(dur);
        assert!(dur.as_secs_f32().abs() <= 0.01);
    }
}
