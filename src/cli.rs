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
}
