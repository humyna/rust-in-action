use anyhow::Result;
use reqwest::Url;
use clap::{AppSettings, Clap};

//定义httpie的CLI主入口，它包含若干子命令
// 下面/// 注释是文档, clap会将其作为CLI的帮助
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "humyna")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

//子命令分别对应不同的http方法，目前仅支持get\post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    //暂不支持其他http方法
}

#[derive(Clap, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url : String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Clap, Debug)]
struct Post {
    url : String, 
    body : Vec<String>,
}
fn main() {
    let opts: Opts= Opts::parse();
    println!("{:?}", opts);
}
