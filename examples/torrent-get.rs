extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::TransClient;
use transmission_rpc::types::{
    BasicAuth, Id, Result, RpcResponse, Tag, Torrent, TorrentGetField, Torrents,
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

    let fields: Option<Vec<_>> = None;
    let ids: Option<Vec<_>> = None;
    let res: RpcResponse<Torrents<Torrent>> = client.torrent_get(fields, ids).await?;
    let names: Vec<&String> = res
        .arguments
        .torrents
        .iter()
        .map(|it| it.name.as_ref().unwrap())
        .collect();
    println!("{:#?}", names);
    assert_eq!(res.tag, None);

    let tag1 = Tag(1);
    let res1: RpcResponse<Torrents<Torrent>> = client
        .torrent_get_tagged(
            Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
            Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
            tag1,
        )
        .await?;
    assert_eq!(res1.tag, Some(tag1));
    let first_three: Vec<String> = res1
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{}. {}",
                &it.id.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    println!("{:#?}", first_three);

    let tag2 = Tag(2);
    let res2: RpcResponse<Torrents<Torrent>> = client
        .torrent_get_tagged(
            Some(vec![
                TorrentGetField::Id,
                TorrentGetField::HashString,
                TorrentGetField::Name,
            ]),
            Some(vec![Id::Hash(String::from(
                "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
            ))]),
            tag2,
        )
        .await?;
    assert_eq!(res2.tag, Some(tag2));
    let info: Vec<String> = res2
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{:5}. {:^45} {}",
                &it.id.as_ref().unwrap(),
                &it.hash_string.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    println!("{:#?}", info);

    Ok(())
}
