mod error;
pub use error::{AttError, Error};

mod cli;
pub use cli::Cli;

mod user;
pub use user::User;

mod attbot;
pub use attbot::AttBot;

mod util;
pub use util::{get_now, OFFECT};

pub mod tjurl {
    /// 登陆链接
    pub const LOGIN: &str = "https://tjupt.org/login.php";
    /// 签到页面
    pub const ATTENDANCE: &str = "https://tjupt.org/attendance.php";
    /// POST 的登陆 url
    pub const TAKELOGIN: &str = "https://tjupt.org/takelogin.php";
}
