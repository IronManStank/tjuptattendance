//! ## 签到实现
//! 为了多种签到实现
//!
//! 1. 通过豆瓣API
//! 2. 通过自建API
//! 3. 方便以后升级、维护

use async_trait::async_trait;

use crate::Error;

#[async_trait]
pub trait Attendance {
    async fn login(&self) -> Result<(), Error>;
    async fn att_once_now(&self) -> Result<(), Error>;
}
