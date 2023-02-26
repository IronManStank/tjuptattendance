use std::path::Path;

use crate::{get_now, Error, User};
use ahash::AHashSet;
use anyhow::anyhow;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetWorkConfig {
    /// 网络延迟 (ms)
    pub(crate) delay: u64,
    /// 获取答案所需要的时间 (s)
    pub(crate) answer: u8,
    /// POST 答案所需要的时间 (s)
    pub(crate) post: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfig {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) host: String,
    pub(crate) port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub(crate) users: AHashSet<User>,
    pub(crate) top: AHashSet<NaiveTime>,
    pub(crate) network: NetWorkConfig,
    pub(crate) email: Option<EmailConfig>,
}

impl ConfigFile {
    /// 获取下一个签到时间点，如果今天没有签到时间点，则返回明天的第一个签到时间点。
    pub fn get_next_top_time_point(&self) -> Option<NaiveDateTime> {
        let now = get_now();
        // 排序
        let mut lst = Vec::from_iter(self.top.iter());
        lst.sort();
        let first = lst.get(0).copied();
        let next_point = lst.into_iter().find(|&&t| now.time() < t);
        if next_point.is_some() {
            next_point.map(|&t| NaiveDateTime::new(now.date(), t))
        } else {
            first.map(|&t| NaiveDateTime::new(now.date() + chrono::Duration::days(1), t))
        }
    }

    /// 生成一个配置文件模板，用于初始化配置文件。
    /// 模板中的时间点设置为 0:00 6:00 7:00 8:00 12:00 18:00 20:00 22:00
    pub fn generate_template() -> Self {
        Self {
            users: {
                let mut set = AHashSet::new();
                set.insert(User::new("username".to_string(), "password".to_string()));
                set
            },
            top: {
                let mut set = AHashSet::new();
                set.insert(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(6, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(7, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(8, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(12, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(18, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(20, 0, 0).unwrap());
                set.insert(NaiveTime::from_hms_opt(22, 0, 0).unwrap());
                set
            },
            network: NetWorkConfig {
                delay: 1000,
                answer: 10,
                post: 10,
            },
            email: Some(EmailConfig {
                username: "username@qq.com".to_string(),
                password: "password".to_string(),
                from: "from@qq.com".to_string(),
                to: "to@qq.com".to_string(),
                host: "smtp.host.com".to_string(),
                port: 465,
            }),
        }
    }

    /// 读取配置文件
    pub fn load_config_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content).map_err(|e| anyhow!("{e}"))?;
        Ok(config)
    }

    /// 保存配置文件
    pub fn save_config_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let content = toml::to_string_pretty(self).map_err(|e| anyhow!("{e}"))?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod config_test {
    use super::*;

    #[test]
    /// 测试读取仓库中的配置文件模板，以确保模板的正确性。
    fn reade_local_template_test() {
        let config_template_path = Path::new("config_template.toml");
        let _config = ConfigFile::load_config_file(config_template_path).unwrap();
    }

    #[test]
    /// 在本地生成默认的配置文件模板
    fn generate_local_template_test() {
        let config_template_path = Path::new("config_template_local.toml");
        let config = ConfigFile::generate_template();
        config.save_config_file(config_template_path).unwrap();
    }
}
