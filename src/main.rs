use std::collections::HashMap;
use std::ffi::c_long;
use std::str::FromStr;
use clap::{arg, Parser};
use anyhow::{Result, anyhow};
use reqwest::{header, Client, Response, Url};


// 定义httpie的cli的主入口
// 

#[derive(Parser, Debug)]
#[clap(version = "1.0", author="sato")]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,

}


#[derive(Parser, Debug)]
struct Get {
    #[clap(value_parser=parse_url)] // 校验URL
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    #[clap(value_parser=parse_url)]
    url: String,
    #[clap(value_parser=parse_kv_pair)]
    boby: Vec<KvPair>,

}


#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),

}

fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    Ok(url.into())
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse! {}", s));
        Ok(
            Self{
                k: (split.next().ok_or_else(err)?).to_string(),
                v: (split.next().ok_or_else(err)?).to_string(),
            }
        )
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}


async fn get(client: Client, args: &Get) -> Result<()>{
    let resp = client.get(&args.url).send().await?;
    // println!("{:?}", resp.text().await?);
    print_resp(resp).await?;
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.boby.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{:?}", resp.text().await?);
    print_resp(resp).await?;

    Ok(())
}

fn print_status(resp: &Response) {
    println!("{:?}\n", format!("{:?} {}", resp.version(), resp.status()))
}



async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    let opts: Opts = Opts::parse();
    let client: Client = Client::new();
    let result = match opts.sub_cmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
