use clap::{Parser, Subcommand};

/// 一个用rust编写的简单文件传输工具
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// 是否开启调试模式
    #[arg(short, long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 发送文件到指定目标地址
    Send(SendArgs),

    /// 启动接收服务并等待文件
    Receive(ReceiveArgs),
}

#[derive(Parser)]
pub struct SendArgs {
    /// 要发送的文件路径
    #[arg(short, long)]
    pub file: String,

    /// 目标地址
    #[arg(short, long)]
    pub target: String,
}

#[derive(Parser)]
pub struct ReceiveArgs {
    /// 监听的端口号
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,

    /// 保存文件的目录
    #[arg(short, long, default_value = "./")]
    pub output_dir: String,
}
