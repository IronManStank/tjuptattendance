//! ## 共用模型及方法
//! 这里定义了一些 client 和 server 共用的基础模型和方法，
//! 以便使得 clent 和 server 有 “共同语言”。
//! 具体实现方式应该在 client 和 server 中适当位置按需实现


#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]

pub mod douban;
pub mod error;
pub mod time;
pub mod user;
