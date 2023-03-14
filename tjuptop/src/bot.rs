//! 主要逻辑

use crate::user::User;
use chrono::prelude::*;
use std::sync::Arc;

/// 签到任务
#[derive(Debug)]
pub struct Bot {
    pub(crate) user: Arc<User>,
    /// 目标时间点
    pub(crate) target: Option<NaiveDateTime>,
}

impl Bot {
    /// 用户
    pub fn user(&self) -> &User {
        &self.user
    }

    /// 签到时间点
    pub fn target(&self) -> Option<NaiveDateTime> {
        self.target
    }
}
