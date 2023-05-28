use clap::{Parser, command, arg};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 定义一个名为 name 的字段，添加一些属性来设置参数的选项和信息
    #[arg(short, long)]
    name: String,

    /// 定义一个名为 count 的字段，添加一些属性来设置参数的选项和信息
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}