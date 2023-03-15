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
/// 如果是 `None` or 0 or 负数 则不等待
pub async fn wait_to_point(target: Option<NaiveDateTime>) {
    if let Some(dt) = target {
        let dur = dt - util::get_now();
        if !dur.abs().is_zero() {
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
        // 一个未来的时间点
        let after = util::get_now() + chrono::Duration::seconds(1);
        // 一个过去的时间点
        let before = after - chrono::Duration::seconds(10);

        // 等待到一个未来的时间点
        let ins = tokio::time::Instant::now();
        wait_to_point(Some(after)).await;
        let dur = ins.elapsed().as_secs_f32();
        dbg!(dur);
        assert!(dur < 1.03);

        // 等待一个过去的和不等待的时间点
        let ins = tokio::time::Instant::now();
        wait_to_point(None).await;
        wait_to_point(Some(before)).await;
        let dur = ins.elapsed().as_secs_f32();
        dbg!(dur);
        assert!(dur <= 0.03);

        // 负数转换
        let dur_neg = chrono::Duration::seconds(-20);
        let dur_std = dur_neg.to_std().unwrap_or_default();
        dbg!(dur_std);
        assert!(dur_std == std::time::Duration::ZERO);
    }
}
