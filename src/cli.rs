use clap::{ArgGroup, Parser};

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
TJUPT规则详情：https://tjupt.org/rules.php

查看 https://github.com/azureqaq/tjuptattendance 来获取更多信息，
如果你有任何问题或建议欢迎 Issue/PR")]
#[command(group(
    ArgGroup::new("setup")
        .required(false)
        .args(["install", "uninstall"])
        .conflicts_with_all(["profile", "temp"])
))]
#[command(group(
    ArgGroup::new("profile")
        .multiple(true)
        .required(false)
        .args(["force"])
))]
#[command(group(
    ArgGroup::new("temp")
        .multiple(true)
        .required(false)
        .args(["user"])
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

    #[arg(short, long, default_value_t = String::from("username"))]
    /// 临时使用，不会产生任何本地文件
    ///
    /// 不会保存 cookie 文件，不依赖配置文件，立即签到
    pub(crate) user: String,

    #[arg(long, short)]
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
}
