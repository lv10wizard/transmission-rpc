use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, GroupSetArgs, Result};
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
    let args = GroupSetArgs::new("group-name".to_owned())
        .speed_limit_down_enabled(true)
        .speed_limit_down(1000);
    let response = client.group_set_tagged(args, tag).await?;
    println!("response: {response:#?}");
    match response.is_ok() {
        true => {
            println!("Ok! (tag: {:?})", response.tag);
            assert_eq!(response.tag, Some(tag));
        },
        false => println!("Err: {}", response.result),
    }

    let mut args = GroupSetArgs::new("group-name".to_owned());
    args.speed_limit_down_enabled = Some(true);
    args.speed_limit_down = Some(1000);
    let response = client.group_set(args).await?;
    println!("response: {response:#?}");
    if response.is_ok() {
        println!("Ok!");
    } else {
        println!("Err: {}", response.result);
    }
    Ok(())
}
