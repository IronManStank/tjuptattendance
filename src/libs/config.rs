//! 配置文件解析
use std::{
    fmt::Display,
    fs::{read_to_string, File},
    hash::Hash,
    io::Write,
    path::Path,
};

use ahash::AHashSet;
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use time::Time;
use toml;

// 签到时间点
// 未考虑网络延迟提前量
lazy_static! {
    /// 午夜
    static ref MIDNIGHT: Time = Time::from_hms(0, 0, 0).unwrap();
    static ref SIX_OCLOCK: Time = Time::from_hms(6, 0, 0).unwrap();
    // static ref EIGHT_OCLOCK: Time = Time::from_hms(8, 0, 0).unwrap();
    // static ref TWELVE_OCLOCK: Time = Time::from_hms(12, 0, 0).unwrap();
    // static ref EIGHTEEN_OCLOK: Time = Time::from_hms(18, 0, 0).unwrap();
    // static ref TWENTY_OCLOCK: Time = Time::from_hms(20, 0, 0).unwrap();
    // static ref TWENTY_TWO_OCLOCK: Time = Time::from_hms(22, 0, 0).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    users: AHashSet<UserConfig>,
    global: GlobalConfig,
}

impl ConfigFile {
    /// 创建一个默认的
    ///
    /// 包含了一个未开启的最简实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 从文件读取
    pub fn new_from<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = read_to_string(path).context(anyhow!(
            "无法读取配置文件: {}, 如果使用默认位置则可能需要先初始化 `--init`",
            path.display()
        ))?;
        let result = toml::from_str(&content)?;
        Ok(result)
    }

    /// 用户配置
    pub fn users(&self) -> &AHashSet<UserConfig> {
        &self.users
    }

    /// 尽量少用，每次都否Clone一次
    pub fn get_users(&self) -> Vec<UserConfig> {
        let mut users = vec![];
        for i in self.users.iter() {
            users.push(i.clone());
        }
        users
    }

    /// 写入文件
    pub fn write_to_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        if !path.is_file() {
            log::info!("创建配置文件并写入: {}", path.display());
        }
        let content = toml::to_string(self)?;
        let mut file = File::create(path).context(anyhow!(
            "无法创建配置文件文件: {}，可能需要先初始化",
            path.display()
        ))?;
        let _r = file.write(content.as_bytes())?;
        Ok(())
    }

    pub fn gloablconfig(&self) -> &GlobalConfig {
        &self.global
    }

    /// 增加用户
    pub fn addusers(&mut self, users: Vec<UserConfig>) {
        for i in users.into_iter() {
            let _ = self.users.insert(i);
        }
    }

    /// 删除用户
    pub fn rmusers(&mut self, users: Vec<&str>) {
        let users: AHashSet<_> = users
            .into_iter()
            .map(|u| UserConfig::new(true, u.into(), "".into(), None, None, None, None))
            .collect();

        for i in users.into_iter() {
            self.users.remove(&i);
        }
    }
}

impl Display for ConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let enable_num = self.users.iter().filter(|u| u.enable).count();
        write!(
            f,
            "ConfigFile[users: {}/{} {}]",
            enable_num,
            self.users.len(),
            self.global
        )
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        let mut users = AHashSet::new();
        let _r = users.insert(UserConfig::default());
        let global = GlobalConfig::default();
        Self { users, global }
    }
}

/// 用户配置信息
#[derive(Serialize, Deserialize, Eq, Debug, Clone)]
pub struct UserConfig {
    enable: bool,
    id: String,
    pwd: String,
    email: Option<String>,
    retry: Option<u8>,
    delay: Option<u32>,
    points_in_time: Option<Vec<Time>>,
}

impl PartialEq for UserConfig {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Hash for UserConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl UserConfig {
    /// 获取重试次数，如果小于1返回1
    pub fn retry(&self) -> u8 {
        match self.retry {
            Some(num) => {
                if num < 1 {
                    1
                } else {
                    num
                }
            }
            None => 3,
        }
    }

    /// 更新delay
    ///
    /// 配置文件操作时不能用
    pub fn delay_when_read(&mut self, global_conf: &GlobalConfig) {
        self.delay = Some(match self.delay {
            None => global_conf.network_delay(),
            Some(n) => n,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn enable(&self) -> bool {
        self.enable
    }
}

impl UserConfig {
    /// 新建一个
    ///
    /// 如果是从命令行读取的，那么肯定是开启的
    pub fn new(
        enable: bool,
        id: String,
        pwd: String,
        email: Option<String>,
        retry: Option<u8>,
        delay: Option<u32>,
        points_in_time: Option<Vec<Time>>,
    ) -> Self {
        Self {
            enable,
            id,
            pwd,
            email,
            retry,
            delay,
            points_in_time,
        }
    }
}

impl Display for UserConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User[{}-{}]", self.id, self.enable)
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            enable: false,
            id: "test".into(),
            pwd: "pwd".into(),
            email: None,
            retry: None,
            delay: None,
            points_in_time: Some(vec![*MIDNIGHT, *SIX_OCLOCK]),
        }
    }
}

/// 全局配置里的邮件配置
#[derive(Serialize, Deserialize, Debug)]
pub struct EmailConfig {
    enable: bool,
    user: String,
    pwd: String,
    sender: Option<String>,
    port: Option<u32>,
    host: Option<String>,
}

impl EmailConfig {
    /// 是否启用
    pub fn enable(&self) -> bool {
        self.enable
    }

    /// port 默认是 465
    pub fn port(&self) -> u32 {
        self.port.unwrap_or(465)
    }

    /// host 默认: smtp.qq.com
    pub fn host(&self) -> &str {
        match self.host {
            Some(ref s) => s.as_str(),
            None => "smtp.qq.com",
        }
    }
}

impl Display for EmailConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmailConf[{}]", self.enable)
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enable: false,
            user: "user".into(),
            pwd: "pwd".into(),
            sender: None,
            port: None,
            host: None,
        }
    }
}

/// 全局配置
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GlobalConfig {
    network_delay: u32,
    emailconf: EmailConfig,
}

impl GlobalConfig {
    /// 网络延迟
    pub fn network_delay(&self) -> u32 {
        if self.network_delay > 1000 {
            1000
        } else {
            self.network_delay
        }
    }

    /// 邮件配置
    pub fn emailconf(&self) -> &EmailConfig {
        &self.emailconf
    }
}

impl Display for GlobalConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GlobalConf[delay: {}ms email: {}]",
            self.network_delay, self.emailconf.enable
        )
    }
}

// impl Default for GlobalConfig {
//     fn default() -> Self {
//         Self {
//             // 默认 50ms
//             network_delay: 0,
//             emailconf: EmailConfig::default(),
//         }
//     }
// }
