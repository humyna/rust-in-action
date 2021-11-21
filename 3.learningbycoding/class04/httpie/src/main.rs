use anyhow::{anyhow, Result};
use reqwest::Url;
use clap::{AppSettings, Clap};
use std::str::FromStr;


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
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str =  parse_url))]
    url : String, 
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body : Vec<KvPair>,
}

/// 命令行中的key=value可以通过parse_kv_pair解析成KvPair
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait后，可以用str.parse()将方法字符串解析成KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用=进行split，会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse{}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为key，迭代器返回Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E), 然后用E 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取出第二个结果作为value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为KvPair实现了FromStr，所以我们可以直接使用s.parse()得到KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}


fn main() {
    let opts: Opts= Opts::parse();
    println!("{:?}", opts);
}
