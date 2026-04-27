//! Sharable version of `TransClient`.
//!
//! It lifted the requirement of `&mut self` on
//! all requests methods by using a lock on inner state. This may introduce some
//! overhead so choose as needed.

use std::{ops::Deref, sync::{Arc, RwLock}};

use reqwest::{Client, Response, StatusCode, Url, header::{CONTENT_TYPE, HeaderValue}};
use semver::Version;
use serde::de::DeserializeOwned;

use crate::{
    BodyString, MAX_RETRIES, TransError,
    json_rpc::JsonRpcResponse,
    types::{
        JSON_RPC_VERSION_2_0, BasicAuth, BlocklistUpdate, FreeSpace, GroupGet, GroupSetArgs, Id,
        Nothing, PortTest, PortTestArgs, Result, RpcRequest, RpcResponse, RpcResponseArgument,
        RpcVersion, SessionGet, SessionGetField, SessionSetArgs, SessionStats, Tag, Torrent,
        TorrentAction, TorrentAddArgs, TorrentAddedOrDuplicate, TorrentGetField, TorrentRenamePath,
        TorrentSetArgs, Torrents,
    },
};

#[derive(Clone)]
pub struct SharableTransClient {
    url: Url,
    auth: Option<BasicAuth>,
    session_id: Arc<RwLock<Option<String>>>,
    client: Client,
    /// Stores the `X-Transmission-Rpc-Version` HTTP header value from the server if provided in
    /// the `409 Conflict` response. `semver` is used to flag that requests should be transformed
    /// into a [JSON-RPC] request.
    ///
    /// [JSON-RPC]: <https://www.jsonrpc.org/specification>
    semver: Arc<RwLock<Option<Version>>>,
}

impl SharableTransClient {
    /// Returns HTTP(S) client with configured Basic Auth
    #[must_use]
    pub fn with_auth(url: Url, basic_auth: BasicAuth) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: Some(basic_auth),
            session_id: RwLock::new(None).into(),
            client: Client::new(),
            semver: RwLock::new(None).into(),
        }
    }

    /// Returns HTTP(S) client
    #[must_use]
    pub fn new(url: Url) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: None,
            session_id: RwLock::new(None).into(),
            client: Client::new(),
            semver: RwLock::new(None).into(),
        }
    }

    #[must_use]
    pub fn new_with_client(url: Url, client: Client) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: None,
            session_id: RwLock::new(None).into(),
            client,
            semver: RwLock::new(None).into(),
        }
    }

    pub fn set_auth(&mut self, basic_auth: BasicAuth) {
        self.auth = Some(basic_auth);
    }

    /// Prepares a request for provided server and auth
    fn rpc_request(&self) -> reqwest::RequestBuilder {
        let mut rq = self.client
            .post(self.url.clone())
            .header(CONTENT_TYPE, "application/json");
        if let Some(auth) = &self.auth {
            rq = rq.basic_auth(&auth.user, Some(&auth.password));
        }
        if let Some(session_id) = &self.session_id
            .read()
            .expect("unpoisoned session-id lock")
            .deref()
        {
            rq = rq.header("X-Transmission-Session-Id", session_id);
        }
        rq
    }

    /// Performs a session set call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, SessionSetArgs},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let args: SessionSetArgs = SessionSetArgs {
    ///         download_dir: Some(
    ///             "/torrent/download".to_string(),
    ///         ),
    ///         ..SessionSetArgs::default()
    ///     };
    ///     let response = client.session_set(args).await;
    ///     match &response {
    ///         Ok(resp) => {
    ///             println!("Yay!");
    ///             assert_eq!(resp.tag, None);
    ///         },
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_set(&self, args: SessionSetArgs) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_set(args, None)).await
    }

    /// Performs a session-set request that can be tracked by `tag`.
    pub async fn session_set_tagged(
        &self,
        args: SessionSetArgs,
        tag: Tag,
    ) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_set(args, Some(tag))).await
    }

    /// Performs a session get call.
    ///
    /// # Arguments
    ///
    /// * `args` - An optional collection of [`SessionGetFields`] to request. Specifying either
    /// `None` or an empty collection (eg. `Some(vec![])`) will request all possible session
    /// arguments. (Note that the latter behavior differs from manual rpc requests where an empty
    /// `fields` array will yield a `session-get` response with an empty arguments object).
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::SharableTransClient;
    /// use transmission_rpc::types::{
    ///     BasicAuth, Result, RpcResponse, SessionGet, SessionGetField, Tag,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let client;
    ///     if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         client = SharableTransClient::with_auth(url.parse()?, BasicAuth {
    ///             user, password,
    ///         });
    ///     } else {
    ///         client = SharableTransClient::new(url.parse()?);
    ///     }
    ///     let response: Result<RpcResponse<SessionGet>> = client.session_get(None).await;
    ///     println!("{response:#?}");
    ///     match &response {
    ///         Ok(resp) => {
    ///             assert_eq!(resp.tag, None);
    ///             println!("Yay!");
    ///         }
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///
    ///     let tag = Tag(123);
    ///     let args = vec![SessionGetField::RpcVersion];
    ///     let response: Result<RpcResponse<SessionGet>> = client
    ///         .session_get_tagged(Some(args), tag)
    ///         .await;
    ///     println!("{response:#?}");
    ///     match &response {
    ///         Ok(resp) => {
    ///             assert_eq!(resp.tag, Some(tag));
    ///             println!("Yay!");
    ///         }
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_get(&self, fields: Option<Vec<SessionGetField>>)
        -> Result<RpcResponse<SessionGet>>
    {
        self.call(RpcRequest::session_get(fields.map(Into::into), None)).await
    }

    /// Performs a session-get request that can be tracked by `tag`.
    pub async fn session_get_tagged(&self, fields: Option<Vec<SessionGetField>>, tag: Tag)
        -> Result<RpcResponse<SessionGet>>
    {
        self.call(RpcRequest::session_get(fields.map(Into::into), Some(tag))).await
    }

    /// Performs a session stats call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, SessionStats},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<SessionSet>> = client.session_stats().await;
    ///     match &response {
    ///         Ok(resp) => {
    ///             println!("Yay!");
    ///             assert_eq!(resp.tag, None);
    ///         },
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_stats(&self) -> Result<RpcResponse<SessionStats>> {
        self.call(RpcRequest::session_stats(None)).await
    }

    /// Performs a session-stats request that can be tracked by `tag`.
    pub async fn session_stats_tagged(&self, tag: Tag) -> Result<RpcResponse<SessionStats>> {
        self.call(RpcRequest::session_stats(Some(tag))).await
    }

    /// Performs a session close call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Nothing, Result, RpcResponse},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response = client.session_close().await;
    ///     match &response {
    ///         Ok(resp) => {
    ///             println!("Yay!");
    ///             assert_eq!(resp.tag, None);
    ///         },
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_close(&self) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_close(None)).await
    }

    /// Performs a session-close request that can be tracked by `tag`.
    pub async fn session_close_tagged(&self, tag: Tag) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_close(Some(tag))).await
    }

    /// Performs a blocklist update call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, BlocklistUpdate, Result, RpcResponse},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<SessionSet>> = client.blocklist_update().await;
    ///     match &response {
    ///         Ok(resp) => {
    ///             println!("Yay!");
    ///             assert_eq!(resp.tag, None);
    ///         },
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn blocklist_update(&self) -> Result<RpcResponse<BlocklistUpdate>> {
        self.call(RpcRequest::blocklist_update(None)).await
    }

    /// Performs a blocklist-update request that can be tracked by `tag`.
    pub async fn blocklist_update_tagged(&self, tag: Tag) -> Result<RpcResponse<BlocklistUpdate>> {
        self.call(RpcRequest::blocklist_update(Some(tag))).await
    }

    /// Performs a session stats call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, FreeSpace, Result, RpcResponse},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let dir = env::var("TDIR")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<FreeSpace>> = client.free_space(dir).await;
    ///     match &response {
    ///         Ok(resp) => {
    ///             println!("Yay!");
    ///             assert_eq!(resp.tag, None);
    ///         },
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn free_space(&self, path: String) -> Result<RpcResponse<FreeSpace>> {
        self.call(RpcRequest::free_space(path, None)).await
    }

    /// Performs a free-space request that can be tracked by `tag`.
    pub async fn free_space_tagged(
        &self,
        path: String,
        tag: Tag,
    ) -> Result<RpcResponse<FreeSpace>> {
        self.call(RpcRequest::free_space(path, Some(tag))).await
    }

    /// Performs a port test call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, PortTest, Result, RpcResponse},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<PortTest>> = client.port_test().await;
    ///     match &response {
    ///         Ok(resp) => {
    ///             println!("Yay!");
    ///             assert_eq!(resp.tag, None);
    ///         },
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn port_test(&self, args: PortTestArgs) -> Result<RpcResponse<PortTest>> {
        self.call(RpcRequest::port_test(args, None)).await
    }

    /// Performs a port-test request that can be tracked by `tag`.
    pub async fn port_test_tagged(&self, args: PortTestArgs, tag: Tag)
        -> Result<RpcResponse<PortTest>>
    {
        self.call(RpcRequest::port_test(args, Some(tag))).await
    }

    /// Move torrents with IDs specified in `ids` to the top of the download queue.
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```rust
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::types::{BasicAuth, Id, Result};
    /// use transmission_rpc::TransClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv()?;
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    ///     } else {
    ///         TransClient::new(url.parse()?)
    ///     };
    ///     let response = client.queue_move_top(vec![Id::Id(1)]).await?;
    ///     if response.is_ok() {
    ///         println!("Ok!");
    ///     } else {
    ///         println!("Err: {}", response.result);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn queue_move_top<I>(&self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_top(ids, None)).await
    }

    /// Performs a `queue-move-top` request that can be tracked by `tag`.
    pub async fn queue_move_top_tagged<I>(&self, ids: I, tag: Tag)
        -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_top(ids, Some(tag))).await
    }

    /// Move torrents with IDs specified in `ids` up in the download queue.
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```rust
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::types::{BasicAuth, Id, Result};
    /// use transmission_rpc::TransClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv()?;
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    ///     } else {
    ///         TransClient::new(url.parse()?)
    ///     };
    ///     let response = client.queue_move_up(vec![Id::Id(1)]).await?;
    ///     if response.is_ok() {
    ///         println!("Ok!");
    ///     } else {
    ///         println!("Err: {}", response.result);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn queue_move_up<I>(&self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_up(ids, None)).await
    }

    /// Performs a `queue-move-up` request that can be tracked by `tag`.
    pub async fn queue_move_up_tagged<I>(&self, ids: I, tag: Tag)
        -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_up(ids, Some(tag))).await
    }

    /// Move torrents with IDs specified in `ids` down in the download queue.
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```rust
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::types::{BasicAuth, Id, Result};
    /// use transmission_rpc::TransClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv()?;
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    ///     } else {
    ///         TransClient::new(url.parse()?)
    ///     };
    ///     let response = client.queue_move_down(vec![Id::Id(1)]).await?;
    ///     if response.is_ok() {
    ///         println!("Ok!");
    ///     } else {
    ///         println!("Err: {}", response.result);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn queue_move_down<I>(&self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_down(ids, None)).await
    }

    /// Performs a `queue-move-down` request that can be tracked by `tag`.
    pub async fn queue_move_down_tagged<I>(&self, ids: I, tag: Tag)
        -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_down(ids, Some(tag))).await
    }

    /// Move torrents with IDs specified in `ids` to the bottom of the download queue.
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```rust
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::types::{BasicAuth, Id, Result};
    /// use transmission_rpc::TransClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv()?;
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    ///     } else {
    ///         TransClient::new(url.parse()?)
    ///     };
    ///     let response = client.queue_move_bottom(vec![Id::Id(1)]).await?;
    ///     if response.is_ok() {
    ///         println!("Ok!");
    ///     } else {
    ///         println!("Err: {}", response.result);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn queue_move_bottom<I>(&self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_bottom(ids, None)).await
    }

    /// Performs a `queue-move-bottom` request that can be tracked by `tag`.
    pub async fn queue_move_bottom_tagged<I>(&self, ids: I, tag: Tag)
        -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_bottom(ids, Some(tag))).await
    }

    /// Performs a torrent get call
    /// fields - if None then ALL fields
    /// ids - if None then All items
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Result, RpcResponse, Tag, Torrent, TorrentGetField, Torrents},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///
    ///     let fields: Option<Vec<_>> = None;
    ///     let ids: Option<Vec<_>> = None;
    ///     let res: RpcResponse<Torrents<Torrent>> =
    ///         client.torrent_get(fields, ids).await?;
    ///     assert_eq!(res.tag, None);
    ///     let names: Vec<&String> = res
    ///         .arguments
    ///         .torrents
    ///         .iter()
    ///         .map(|it| it.name.as_ref().unwrap())
    ///         .collect();
    ///     println!("{:#?}", names);
    ///
    ///     let tag1 = Tag(1);
    ///     let res1: RpcResponse<Torrents<Torrent>> = client
    ///         .torrent_get_tagged(
    ///             Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
    ///             Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
    ///             tag,
    ///         )
    ///         .await?;
    ///     assert_eq!(res1.tag, Some(tag1));
    ///     let first_three: Vec<String> = res1
    ///         .arguments
    ///         .torrents
    ///         .iter()
    ///         .map(|it| {
    ///             format!(
    ///                 "{}. {}",
    ///                 &it.id.as_ref().unwrap(),
    ///                 &it.name.as_ref().unwrap()
    ///             )
    ///         })
    ///         .collect();
    ///     println!("{:#?}", first_three);
    ///
    ///     let tag2 = Tag(-1);
    ///     let res2: RpcResponse<Torrents<Torrent>> = client
    ///         .torrent_get_tagged(
    ///             Some(vec![
    ///                 TorrentGetField::Id,
    ///                 TorrentGetField::HashString,
    ///                 TorrentGetField::Name,
    ///             ]),
    ///             Some(vec![Id::Hash(String::from(
    ///                 "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
    ///             ))]),
    ///             tag2,
    ///         )
    ///         .await?;
    ///     assert_eq!(res2.tag, Some(tag2));
    ///     let info: Vec<String> = res2
    ///         .arguments
    ///         .torrents
    ///         .iter()
    ///         .map(|it| {
    ///             format!(
    ///                 "{:5}. {:^45} {}",
    ///                 &it.id.as_ref().unwrap(),
    ///                 &it.hash_string.as_ref().unwrap(),
    ///                 &it.name.as_ref().unwrap()
    ///             )
    ///         })
    ///         .collect();
    ///     println!("{:#?}", info);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_get<FIELDS, IDS>(
        &self,
        fields: Option<FIELDS>,
        ids: Option<IDS>,
    ) -> Result<RpcResponse<Torrents<Torrent>>>
    where
        FIELDS: IntoIterator<Item = TorrentGetField> + FromIterator<TorrentGetField>,
        IDS: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_get(fields, ids, None)).await
    }

    /// Performs a torrent-get request that can be tracked by `tag`.
    pub async fn torrent_get_tagged<FIELDS, IDS>(
        &self,
        fields: Option<FIELDS>,
        ids: Option<IDS>,
        tag: Tag,
    ) -> Result<RpcResponse<Torrents<Torrent>>>
    where
        FIELDS: IntoIterator<Item = TorrentGetField> + FromIterator<TorrentGetField>,
        IDS: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_get(fields, ids, Some(tag)))
            .await
    }

    /// Performs a torrent set call
    /// args - the fields to update
    /// ids - if None then All items
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Result, RpcResponse, Torrent, TorrentSetArgs, Torrents},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///
    ///     let url = env::var("TURL")?.parse()?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url, basic_auth);
    ///
    ///     let args = TorrentSetArgs::default()
    ///         .labels(vec![String::from("blue")]);
    ///     assert!(
    ///         client
    ///             .torrent_set(args, Some(vec![Id::Id(0)]))
    ///             .await?
    ///             .is_ok()
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_set<I>(
        &self,
        args: TorrentSetArgs,
        ids: Option<I>,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_set(args, ids, None)).await
    }

    /// Performs a torrent-set request that can be tracked by `tag`.
    pub async fn torrent_set_tagged<I>(
        &self,
        args: TorrentSetArgs,
        ids: Option<I>,
        tag: Tag,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_set(args, ids, Some(tag)))
            .await
    }

    /// Performs a torrent action call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Nothing, Result, RpcResponse, Tag, TorrentAction},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res1: RpcResponse<Nothing> = client
    ///         .torrent_action(TorrentAction::Start, vec![Id::Id(1)])
    ///         .await?;
    ///     assert_eq!(res1.tag, Some(Tag(1)));
    ///     println!("Start result: {:?}", &res1.is_ok());
    ///     let res2: RpcResponse<Nothing> = client
    ///         .torrent_action_tagged(TorrentAction::Stop, vec![Id::Id(1)], Tag(-1))
    ///         .await?;
    ///     assert_eq!(res2.tag, Some(Tag(-1)));
    ///     println!("Stop result: {:?}", &res2.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_action<I>(
        &self,
        action: TorrentAction,
        ids: I,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_action(action, ids, None))
            .await
    }

    /// Performs a torrent-action request that can be tracked by `tag`.
    pub async fn torrent_action_tagged<I>(
        &self,
        action: TorrentAction,
        ids: I,
        tag: Tag,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_action(action, ids, Some(tag)))
            .await
    }

    /// Performs a torrent remove call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Nothing, Result, RpcResponse},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<Nothing> =
    ///         client.torrent_remove(vec![Id::Id(1)], false).await?;
    ///     assert_eq!(res.tag, None);
    ///     println!("Remove result: {:?}", &res.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_remove<I>(
        &self,
        ids: I,
        delete_local_data: bool,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_remove(ids, delete_local_data, None))
            .await
    }

    /// Performs a torrent-remove request that can be tracked by `tag`.
    pub async fn torrent_remove_tagged<I>(
        &self,
        ids: I,
        delete_local_data: bool,
        tag: Tag,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_remove(
            ids,
            delete_local_data,
            Some(tag),
        ))
        .await
    }

    /// Performs a torrent set location call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Nothing, Result, RpcResponse},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<Nothing> = client
    ///         .torrent_set_location(
    ///             vec![Id::Id(1)],
    ///             String::from("/new/location"),
    ///             false,
    ///         )
    ///         .await?;
    ///     assert_eq!(res.tag, None);
    ///     println!("Set-location result: {:?}", &res.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_set_location<I>(
        &self,
        ids: I,
        location: String,
        move_from: bool,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_set_location(
            ids, location, move_from, None,
        ))
        .await
    }

    /// Performs a torrent-set-location request that can be tracked by `tag`.
    pub async fn torrent_set_location_tagged<I>(
        &self,
        ids: I,
        location: String,
        move_from: bool,
        tag: Tag,
    ) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_set_location(
            ids,
            location,
            move_from,
            Some(tag),
        ))
        .await
    }

    /// Performs a torrent rename path call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Result, RpcResponse, Tag, TorrentRenamePath},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<TorrentRenamePath> = client
    ///         .torrent_rename_path(
    ///             vec![Id::Id(1)],
    ///             String::from("Folder/OldFile.jpg"),
    ///             String::from("NewFile.jpg"),
    ///         )
    ///         .await?;
    ///     assert_eq!(res.tag, Some(Tag(100)));
    ///     println!("rename-path result: {:#?}", res);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_rename_path<I>(
        &self,
        ids: I,
        path: String,
        name: String,
    ) -> Result<RpcResponse<TorrentRenamePath>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_rename_path(ids, path, name, None))
            .await
    }

    /// Performs a torrent-rename-path request that can be tracked by `tag`.
    pub async fn torrent_rename_path_tagged<I>(
        &self,
        ids: I,
        path: String,
        name: String,
        tag: Tag,
    ) -> Result<RpcResponse<TorrentRenamePath>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::torrent_rename_path(ids, path, name, Some(tag)))
            .await
    }

    /// Performs a torrent add call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, TorrentAddArgs, TorrentAddedOrDuplicate},
    ///     SharableTransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let mut client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let add: TorrentAddArgs = TorrentAddArgs {
    ///         filename: Some(
    ///             "https://releases.ubuntu.com/22.04/ubuntu-22.04.3-desktop-amd64.iso.torrent"
    ///                 .to_string(),
    ///         ),
    ///         ..TorrentAddArgs::default()
    ///     };
    ///     let res: RpcResponse<TorrentAddedOrDuplicate> = client.torrent_add(add).await?;
    ///     assert_eq!(res.tag, None);
    ///     println!("Add result: {:?}", &res.is_ok());
    ///     println!("response: {:?}", &res);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Panics
    /// Either metainfo or torrent filename must be set or this call will panic.
    pub async fn torrent_add(
        &self,
        add: TorrentAddArgs,
    ) -> Result<RpcResponse<TorrentAddedOrDuplicate>> {
        if !add.is_valid() {
            return Err(TransError::TorrentAddInvalid.into());
        }
        self.call(RpcRequest::torrent_add(add, None)).await
    }

    /// Performs a `torrent-add` request that can be tracked by `tag`.
    pub async fn torrent_add_tagged(
        &self,
        add: TorrentAddArgs,
        tag: Tag,
    ) -> Result<RpcResponse<TorrentAddedOrDuplicate>> {
        if !add.is_valid() {
            return Err(TransError::TorrentAddInvalid.into());
        }
        self.call(RpcRequest::torrent_add(add, Some(tag))).await
    }

    /// Performs a group-get request.
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::types::{BasicAuth, Result};
    /// use transmission_rpc::TransClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv()?;
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    ///     } else {
    ///         TransClient::new(url.parse()?)
    ///     };
    /// 
    ///     let tag = 123.into();
    ///     let group = Some(vec!["my-group-name".to_owned()]);
    ///     let response = client.group_get_tagged(group, tag).await?;
    ///     println!("response: {response:#?}");
    ///     match response.is_ok() {
    ///         true => {
    ///             println!("Ok!");
    ///             assert_eq!(response.tag, Some(tag));
    ///         },
    ///         false => println!("Err: {}", response.result),
    ///     }
    /// 
    ///     let response = client.group_get(None).await?;
    ///     println!("response: {response:#?}");
    ///     if response.is_ok() {
    ///         println!("Ok!");
    ///     } else {
    ///         println!("Err: {}", response.result);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn group_get(&self, groups: Option<Vec<String>>)
        -> Result<RpcResponse<Vec<GroupGet>>>
    {
        self.call(RpcRequest::group_get(groups, None)).await
    }

    /// Performs a group-get request that can be tracked by `tag`.
    pub async fn group_get_tagged(&self, groups: Option<Vec<String>>, tag: Tag)
        -> Result<RpcResponse<Vec<GroupGet>>>
    {
        self.call(RpcRequest::group_get(groups, Some(tag))).await
    }

    /// Performs a group-set request.
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// use dotenvy::dotenv;
    /// use std::env;
    /// use transmission_rpc::types::{BasicAuth, GroupSetArgs, Result};
    /// use transmission_rpc::TransClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv()?;
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    ///     } else {
    ///         TransClient::new(url.parse()?)
    ///     };
    /// 
    ///     let tag = 123.into();
    ///     let args = GroupSetArgs::new("group-name".to_owned())
    ///         .speed_limit_down_enabled(true)
    ///         .speed_limit_down(1000);
    ///     let response = client.group_set_tagged(args, tag).await?;
    ///     println!("response: {response:#?}");
    ///     match response.is_ok() {
    ///         true => {
    ///             println!("Ok! (tag: {:?})", response.tag);
    ///             assert_eq!(response.tag, Some(tag));
    ///         },
    ///         false => println!("Err: {}", response.result),
    ///     }
    /// 
    ///     let mut args = GroupSetArgs::new("group-name".to_owned());
    ///     args.speed_limit_down_enabled = Some(true);
    ///     args.speed_limit_down = Some(1000);
    ///     let response = client.group_set(args).await?;
    ///     println!("response: {response:#?}");
    ///     if response.is_ok() {
    ///         println!("Ok!");
    ///     } else {
    ///         println!("Err: {}", response.result);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn group_set(&self, args: GroupSetArgs) -> Result<RpcResponse<Nothing>>
    {
        self.call(RpcRequest::group_set(args, None)).await
    }

    /// Performs a group-set request that can be tracked by `tag`.
    pub async fn group_set_tagged(&self, args: GroupSetArgs, tag: Tag)
        -> Result<RpcResponse<Nothing>>
    {
        self.call(RpcRequest::group_set(args, Some(tag))).await
    }

    /// Performs a JRPC call to the server
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    async fn call<RS>(&self, request: RpcRequest) -> Result<RpcResponse<RS>>
    where
        RS: RpcResponseArgument + DeserializeOwned + std::fmt::Debug,
    {
        let mut remaining_retries = MAX_RETRIES;
        loop {
            remaining_retries = remaining_retries
                .checked_sub(1)
                .ok_or(TransError::MaxRetriesReached)?;

            debug!("Loaded auth: {:?}", &self.auth);
            let mut rq = self.rpc_request();
            if let Some(semver) = self.semver.read().expect("unpoisoned semver lock").as_ref() {
                let request = request.clone().into_compat(semver)?;
                rq = rq.json(&request);
            }

            debug!(
                "Request body: {:?}",
                rq.try_clone()
                    .expect("Unable to get the request body")
                    .body_string()?
            );

            let rsp: reqwest::Response = rq.send().await?;

            debug!("Response: {:?}", &rsp);
            if matches!(rsp.status(), StatusCode::CONFLICT) {
                let session_id = rsp
                    .headers()
                    .get("X-Transmission-Session-Id")
                    .ok_or(TransError::NoSessionIdReceived)?
                    .to_str()?;
                *self.session_id.write().expect("lock being poisoned") =
                    Some(String::from(session_id));

                debug!("Got new session_id: {}.", session_id);

                self.set_server_rpc_semver(&rsp).await?;
                debug!("Retrying request...");
            } else {
                let rpc_response: RpcResponse<RS> =
                    match self.semver.read().expect("unpoisoned semver lock").as_ref() {
                        Some(semver) if semver >= &Version::new(6, 0, 0) => {
                            let resp = rsp.json::<JsonRpcResponse<RS>>().await?;
                            debug!("JSON-RPC response body: {:#?}", resp);
                            if resp.jsonrpc != JSON_RPC_VERSION_2_0 {
                                // This probably means that the request was handled by a new
                                // Transmission version with an upgrade JSON-RPC protocol.
                                return TransError::UnhandledJsonRpcVersion(resp.jsonrpc.clone())
                                    .into();
                            }
                            resp.into()
                        },
                        _ => {
                            let resp = rsp.json().await?;
                            debug!("Response body: {:#?}", resp);
                            resp
                        },
                    };

                return Ok(rpc_response);
            }
        }
    }

    /// Stores the transmission server's reported rpc-semver either from its Conflict 409 response
    /// header `X-Transmission-Session-Id` for `rpc-semver >= 6.0.0` or by performing an additional
    /// `session-get` request to get the server's `rpc-version` for `6.0.0 > rpc-semver >= 1.3.0`.
    async fn set_server_rpc_semver(&self, rsp: &Response) -> Result<()> {
        // We only need to set this a single time.
        if !matches!(rsp.status(), StatusCode::CONFLICT) ||
            self.semver.read().expect("unpoisoned semver lock").is_some()
        {
            return Ok(());
        }

        // "Starting from rpc-version-semver 6.0.0, Transmission returns the RPC version in an HTTP
        // header X-Transmission-Rpc-Version: {rpc_version_semver} in the CSRF HTTP 409 response.
        // This is so that clients supporting both JSON-RPC and the old bespoke API can determine
        // which scheme to use without making any extra requests. Example:
        // X-Transmission-Rpc-Version: 6.0.0"
        let semver_header = rsp
            .headers()
            .get("X-Transmission-Rpc-Version")
            .map(HeaderValue::to_str)
            .transpose()?
            .map(Version::parse)
            .transpose()?;
        let semver = match semver_header {
            Some(semver) => Some(semver),
            None => {
                // This transmission rpc server is < rpc-semver 6.0.0.
                if self.session_id
                    .read()
                    .expect("unpoisoned session-id lock")
                    .is_none()
                {
                    return TransError::NoSessionIdReceived.into();
                };
                // Try to get the rpc-version with a session-get request which requires a minimum
                // rpc-semver of 1.3.0.
                let req = self.rpc_request()
                    .json(&{
                        let session_get = RpcRequest::session_get(None, None);
                        session_get.into_compat(&Version::new(1, 3, 0))?
                    });
                debug!("Requesting session-get to store the server's rpc-version-semver");
                let resp: RpcResponse<SessionGet> = req.send()
                    .await?
                    .json()
                    .await?;
                match resp.is_ok() {
                    true => {
                        resp.arguments.rpc_version
                            .map(|v| RpcVersion(v).semver())
                            .transpose()?
                    },
                    false => {
                        return TransError::ResponseFailure(resp.result).into();
                    },
                }
            },
        };
        if semver.is_none() {
            return TransError::VersionTooLow(RpcVersion(4)).into();
        }
        debug!("Got rpc-version-semver: {}", semver.as_ref().unwrap());
        *self.semver.write().expect("unpoisoned semver lock") = semver;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use dotenvy::dotenv;

    use super::*;

    #[tokio::test]
    pub async fn test_malformed_url() -> Result<()> {
        dotenv().ok();
        // To prevent double init
        let _ = env_logger::try_init();
        let url = env::var("TURL")?;
        let client;
        if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
            client = SharableTransClient::with_auth(url.parse()?, BasicAuth { user, password });
        } else {
            client = SharableTransClient::new(url.parse()?);
        }
        info!("Client is ready!");
        let add: TorrentAddArgs = TorrentAddArgs {
            filename: Some(
                "https://releases.ubuntu.com/jammy/ubuntu-22.04.1-desktop-amd64.iso.torrentt"
                    .to_string(),
            ),
            ..TorrentAddArgs::default()
        };
        match client.torrent_add(add).await {
            Ok(res) => {
                println!("Add result: {:?}", &res.is_ok());
                println!("response: {:?}", &res);
                assert!(!&res.is_ok());
                assert_eq!(res.tag, None);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
            }
        }

        Ok(())
    }
}
