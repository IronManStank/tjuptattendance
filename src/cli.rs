use anyhow::anyhow;
use clap::{ArgGroup, Parser};

use crate::{Error, User};

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author-with-newline}{about-section}
{usage-heading} {usage}

{all-args}{after-help}";

#[derive(Debug, Parser)]
#[command(author, version, about, help_template = HELP_TEMPLATE, long_about,
    arg_required_else_help = false,
    before_long_help = "\
tjuptatt 仅作为学习交流之用，禁止一切违法违规用途！
TJUPT规则详见：https://tjupt.org/rules.php

查看 https://github.com/azureqaq/tjuptattendance 来获取更多信息，
如果你有任何问题或建议欢迎 Issue/PR")]
#[command(group(
    ArgGroup::new("setup")
        .required(false)
        .args(["install", "uninstall"])
        .conflicts_with_all(["profile", "temp", "general"])
))]
#[command(group(
    ArgGroup::new("profile")
        .multiple(true)
        .required(false)
        .args(["force", "top"])
        .conflicts_with_all(["temp"])
))]
#[command(group(
    ArgGroup::new("temp")
        .multiple(true)
        .required(false)
        .args(["user"])
))]
#[command(group(
    ArgGroup::new("general")
        .multiple(true)
        .required(false)
        .args(["retry"])
))]
/// 一个 TJUPT <https://tjupt.org/> 签到工具.
/// 支持：TOP10签到、邮件提醒
pub struct Cli {
    #[arg(long)]
    /// 创建必须的文件夹及配置文件
    ///
    /// 初次使用时，请编辑生成的默认配置
    pub(crate) install: bool,

    #[arg(long)]
    /// 删除所有安装时生成的文件(夹)
    ///
    /// 注意：此操作会删掉默认配置文件和状态文件所在的文件夹
    pub(crate) uninstall: bool,

    #[arg(short, long, value_name = "NAME=PWD")]
    #[arg(value_parser = get_user)]
    /// 临时使用，不会产生任何本地文件
    ///
    /// 不会保存 cookie 文件，不依赖配置文件，立即签到，
    /// 可通过 `--user name=pwd --user name2=pwd2` 来指定多个，
    /// 注意：此方式使用时，要求 name 中不含 '='
    pub(crate) user: Vec<User>,

    #[arg(long, short, value_name = "NUM")]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=20), default_value_t = 1)]
    /// 重试次数
    ///
    /// 包括：临时模式，配置文件模式
    retry: u8,

    #[arg(long)]
    /// 忽略本地已完成记录，强制签到
    ///
    /// 即：不尝试本地记录中已完成的签到。默认关闭
    pub(crate) force: bool,

    #[arg(long)]
    /// TOP10签到模式
    ///
    /// 此模式必须使用配置文件
    pub(crate) top: bool,
}

impl Cli {
    pub fn is_temp_use(&self) -> bool {
        !self.user.is_empty()
    }

    // 根据用户名去重
    pub fn get_users(&self) -> ahash::AHashSet<&User> {
        ahash::AHashSet::from_iter(self.user.iter())
    }
}

fn get_user(s: &str) -> Result<User, Error> {
    let Some((name, pwd)) = s.split_once('=') else {
        return Err(Error::Other(anyhow!("无法解析为 User，请按照格式输入")));
    };
    Ok(User {
        name: name.into(),
        pwd: pwd.into(),
    })
}
