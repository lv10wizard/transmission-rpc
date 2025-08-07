extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::TransClient;
use transmission_rpc::types::{BasicAuth, Id, Nothing, Result, RpcResponse, Tag, TorrentAction};

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
    let res1: RpcResponse<Nothing> = client
        .torrent_action(TorrentAction::Start, vec![Id::Id(1)])
        .await?;
    println!("Start result: {:?}", &res1.is_ok());
    assert_eq!(res1.tag, None);
    let tag2 = Tag(2);
    let res2: RpcResponse<Nothing> = client
        .torrent_action_tagged(TorrentAction::Stop, vec![Id::Id(1)], tag2)
        .await?;
    println!("Stop result: {:?}", &res2.is_ok());
    assert_eq!(res2.tag, Some(tag2));

    Ok(())
}
