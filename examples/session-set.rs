extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::TransClient;
use transmission_rpc::types::{
    AltSpeedDay, BasicAuth, Encryption, Nothing, Result, RpcResponse, SessionSetArgs, Tag,
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
    let args = SessionSetArgs {
        alt_speed_time_day: Some(AltSpeedDay::WEEKDAY),
        encryption: Some(Encryption::Preferred),
        ..Default::default()
    };
    let response: Result<RpcResponse<Nothing>> = client.session_set(args).await;
    println!("{response:#?}");
    match &response {
        Ok(resp) => {
            assert_eq!(resp.tag, None);
            println!("Yay!");
        }
        Err(_) => panic!("Oh no!"),
    }
    println!("Rpc response is ok: {}", response?.is_ok());

    let tag = Tag(123);
    let args = SessionSetArgs {
        alt_speed_time_day: Some({
            AltSpeedDay::MONDAY | AltSpeedDay::WEDNESDAY | AltSpeedDay::FRIDAY
        }),
        ..Default::default()
    };
    let response: Result<RpcResponse<Nothing>> = client.session_set_tagged(args, tag).await;
    println!("{response:#?}");
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
