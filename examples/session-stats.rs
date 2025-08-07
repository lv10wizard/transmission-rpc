extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::TransClient;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse, SessionStats, Tag};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let mut client;
    if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        client = TransClient::with_auth(url.parse()?, BasicAuth { user, password });
    } else {
        client = TransClient::new(url.parse()?);
    }
    let response: Result<RpcResponse<SessionStats>> = client.session_stats().await;
    match &response {
        Ok(resp) => {
            assert_eq!(resp.tag, None);
            println!("Yay!");
        }
        Err(_) => panic!("Oh no!"),
    }
    println!("Rpc response is ok: {}", response?.is_ok());

    let tag = Tag(321);
    let response: Result<RpcResponse<SessionStats>> = client.session_stats_tagged(tag).await;
    match &response {
        Ok(resp) => {
            assert_eq!(resp.tag, Some(tag));
            println!("Yay!");
        }
        Err(_) => panic!("Oh no!"),
    }
    println!("Rpc response is ok: {}", response?.is_ok());
    Ok(())
}
