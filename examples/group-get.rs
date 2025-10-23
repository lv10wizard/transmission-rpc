use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result};
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
    let response = client.group_get_tagged(Some(vec!["my-group-name".to_owned()]), tag).await?;
    println!("response: {response:#?}");
    match response.is_ok() {
        true => {
            println!("Ok!");
            assert_eq!(response.tag, Some(tag));
        },
        false => println!("Err: {}", response.result),
    }

    let response = client.group_get(None).await?;
    println!("response: {response:#?}");
    if response.is_ok() {
        println!("Ok!");
    } else {
        println!("Err: {}", response.result);
    }
    Ok(())
}
