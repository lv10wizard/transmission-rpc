use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Id, Result};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();
    let url = env::var("TURL")?;
    let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    } else {
        TransClient::new(url.parse()?)
    };

    let tag = 123.into();
    let response = client.queue_move_bottom_tagged([Id::Id(1)], tag).await?;
    match response.is_ok() {
        true => {
            println!("Ok! (tag: {:?})", response.tag);
            assert_eq!(response.tag, Some(tag));
        },
        false => println!("Err: {}", response.result),
    }

    let response = client.queue_move_top(vec![Id::Id(1)]).await?;
    if response.is_ok() {
        println!("Ok!");
    } else {
        println!("Err: {}", response.result);
    }
    Ok(())
}
