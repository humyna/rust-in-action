use anyhow::{anyhow, Result};
use reqwest::{ header, Client, Response, Url};
use clap::{AppSettings, Clap};
use std::{collections::HashMap, str::FromStr};
use colored::*;
use mime::Mime;


//定义httpie的CLI主入口，它包含若干子命令
// 下面/// 注释是文档, clap会将其作为CLI的帮助
/// a native httpie implmentation with Rust.
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

// get子命令
/// feed get with the url and we will retrieve the response for you 
#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url : String,
}

/// 检查url是否合法
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

// post子命令。 需要输入一个URL，和若干个可选的key = value,用于提供json body
/// feed post with the url and optional key=value pairs. we will post the data s JSON, and retrieve the response for you 
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
#[derive(Debug, PartialEq)]
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

/// 处理get子命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

/// 处理post子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

/// 打印响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_context_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

// 打印服务器版本号和状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP headers
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{} {:?}", name.to_string().green(), value);
    }

    print!("\n");
}

/// 将服务器返回的context-type解析成Mime类型
fn get_context_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于”applcation/json“进行pretty print
        Some(v) if v == mime:: APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        //其他mime type直接输出
        _ => println!("{}", body),
    }
}
// 使用 #[tokio::main] 宏来自动添加处理异步的运行时
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts= Opts::parse();
    //println!("{:?}", opts);
    //为http客户端添加一些缺省头
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY","Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    //生成一个http客户端
    let  client = reqwest::Client::builder().default_headers(headers).build()?;   
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

// 仅在cargo test时才编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(parse_kv_pair("name=humyna").unwrap(),
            KvPair {
                k: "name".into(),
                v: "humyna".into()
            }
        );

        assert_eq!(parse_kv_pair("name=").unwrap(),
            KvPair {
                k: "name".into(),
                v: "".into()
            }
        );
    }


}