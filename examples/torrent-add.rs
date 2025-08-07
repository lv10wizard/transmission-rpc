extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::TransClient;
use transmission_rpc::types::{
    BasicAuth, Result, RpcResponse, Tag, TorrentAddArgs, TorrentAddedOrDuplicate,
};

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
    let add: TorrentAddArgs = TorrentAddArgs {
        filename: Some(
            "https://releases.ubuntu.com/20.04/ubuntu-20.04.2.0-desktop-amd64.iso.torrent"
                .to_string(),
        ),
        ..TorrentAddArgs::default()
    };
    let res: RpcResponse<TorrentAddedOrDuplicate> = client.torrent_add(add.clone()).await?;
    println!("Add result: {:?}", &res.is_ok());
    println!("response: {:?}", &res);
    assert_eq!(res.tag, None);

    let tag = Tag(9);
    let res: RpcResponse<TorrentAddedOrDuplicate> = client.torrent_add_tagged(add, tag).await?;
    println!("Add result: {:?}", &res.is_ok());
    println!("response: {:?}", &res);
    assert_eq!(res.tag, Some(tag));

    Ok(())
}
