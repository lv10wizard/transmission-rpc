//! Library to communicate with [transmission
//! rpc](https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md).
//!
//! **WARNING:**
//!
//! It is highly encouraged to use HTTPS since the Transmission authentication is using
//! [BasicAuth](https://wikipedia.org/wiki/Basic_access_authentication) which could be easily
//! intercepted.
//!
//! #### Transmission RPC Spec
//!
//! <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md>
//!
//! #### Supported Methods
//!
//! ##### Torrent Actions
//!
//! - [X] torrent-start
//! - [X] torrent-stop
//! - [X] torrent-start-now
//! - [X] torrent-verify
//! - [X] torrent-reannounce
//!
//! ##### Torrent Mutators
//!
//! - [X] torrent-set
//! - [X] torrent-get
//! - [X] torrent-add
//! - [X] torrent-remove
//! - [X] torrent-set-location
//! - [X] torrent-rename-path
//!
//! ##### Session Requests
//!
//! - [X] session-set
//! - [X] session-get
//! - [X] session-stats
//! - [X] blocklist-update
//! - [X] port-test
//! - [X] queue-move-top, queue-move-up, queue-move-down, queue-move-bottom
//! - [X] session-close
//! - [X] free-space
//! - [X] group-set
//! - [X] group-get
//!
//! ##### Feature Flags
//!
//! - `sync`: Enables a thread-safe version of `TransClient`.
//!
//! ### Examples
//!
//! To run examples: `cargo run --example EXAMPLE-NAME`, eg.
//!
//! ```bash
//! cargo run --example port-test
//! ```
//!
//! You can specify server url and, if needed, credentials by defining env vars:
//!
//! * `TURL` - Transmission daemon rpc url, eg. `localhost:9091/transmission/rpc`
//! * `TUSER` - Optional rpc username
//! * `TPWD` - Optional rpc password
//!
//! One way to define these:
//!
//! ```bash
//! TURL=localhost:9091/transmission/rpc TUSER=name TPWD=hunter2 \
//!      cargo run --example port-test
//! ```
//!
//! **NOTE:** These examples will connect to and perform requests to an actual
//! server! Run modifying rpc examples (like `torrent-remove`) with care!
//!
//!
//! The following examples are implemented:
//!
//! ```text
//! $ tree examples/
//! examples/
//! ├── blocklist-update.rs
//! ├── free-space.rs
//! ├── group-get.rs
//! ├── group-set.rs
//! ├── port-test.rs
//! ├── queue-move.rs
//! ├── session-close.rs
//! ├── session-get.rs
//! ├── session-stats.rs
//! ├── torrent-action.rs
//! ├── torrent-add.rs
//! ├── torrent-get.rs
//! ├── torrent-remove.rs
//! ├── torrent-rename-path.rs
//! └── torrent-set-location.rs
//! 
//! 1 directory, 15 files
//! ```
//!
//! -----
//!
//! Support the project: [![Donate button](https://www.paypalobjects.com/en_US/DK/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=H337RKJSC4YG4&source=url)
//!
//! <a href="https://www.buymeacoffee.com/j0rsa" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: 41px !important;width: 174px !important;box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;-webkit-box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;" ></a>
//!
//! [`HashMap`]: std::collections::HashMap
//! [`HashSet`]: std::collections::HashSet
//! [`OrderedFloat`]: https://docs.rs/ordered-float/latest/ordered_float/struct.OrderedFloat.html

#[macro_use]
extern crate log;

use reqwest::{Client, StatusCode, Url, header::{CONTENT_TYPE, HeaderValue}};
use semver::Version;
use serde::de::DeserializeOwned;

#[cfg(feature = "sync")]
pub use sync::SharableTransClient;
use json_rpc::JsonRpcResponse;
use types::{
    JSON_RPC_VERSION_2_0, BasicAuth, BlocklistUpdate, FreeSpace, GroupGet, GroupSetArgs, Id,
    Nothing, PortTest, Result, RpcRequest, RpcResponse, RpcResponseArgument, SessionGet,
    SessionGetField, SessionSetArgs, SessionStats, Tag, Torrent, TorrentAction, TorrentAddArgs,
    TorrentAddedOrDuplicate, TorrentGetField, TorrentRenamePath, TorrentSetArgs, Torrents,
};

#[cfg(feature = "sync")]
mod sync;

mod json_rpc;
pub mod types;

const MAX_RETRIES: usize = 5;

#[derive(Clone, Debug)]
enum TransError {
    MaxRetriesReached,
    NoSessionIdReceived,
    UnhandledJsonRpcVersion(String),
}

impl std::fmt::Display for TransError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransError::MaxRetriesReached => write!(f, "Max retries reached!"),
            TransError::NoSessionIdReceived => write!(f, "No session id received!"),
            TransError::UnhandledJsonRpcVersion(v) => write!(f, "Unhandled JSON-RPC version: {v}"),
        }
    }
}

impl std::error::Error for TransError {}

pub struct TransClient {
    url: Url,
    auth: Option<BasicAuth>,
    session_id: Option<String>,
    // TODO: refactor to a wrapper Client that handles both reqwest & jsonrpc clients
    client: Client,
    /// Stores the `X-Transmission-Rpc-Version` HTTP header value from the server if provided in
    /// the `409 Conflict` response. `semver` is used to flag that requests should be transformed
    /// into a [JSON-RPC] request.
    ///
    /// [JSON-RPC]: <https://www.jsonrpc.org/specification>
    semver: Option<Version>,
}

impl TransClient {
    /// Returns HTTP(S) client with configured Basic Auth
    #[must_use]
    pub fn with_auth(url: Url, basic_auth: BasicAuth) -> TransClient {
        TransClient {
            url,
            auth: Some(basic_auth),
            session_id: None,
            client: Client::new(),
            semver: None,
        }
    }

    /// Returns HTTP(S) client
    #[must_use]
    pub fn new(url: Url) -> TransClient {
        TransClient {
            url,
            auth: None,
            session_id: None,
            client: Client::new(),
            semver: None,
        }
    }

    #[must_use]
    pub fn new_with_client(url: Url, client: Client) -> TransClient {
        TransClient {
            url,
            auth: None,
            session_id: None,
            client,
            semver: None,
        }
    }

    pub fn set_auth(&mut self, basic_auth: BasicAuth) {
        self.auth = Some(basic_auth);
    }

    /// Prepares a request for provided server and auth
    fn rpc_request(&self) -> reqwest::RequestBuilder {
        if let Some(auth) = &self.auth {
            self.client
                .post(self.url.clone())
                .basic_auth(&auth.user, Some(&auth.password))
        } else {
            self.client.post(self.url.clone())
        }
        .header(CONTENT_TYPE, "application/json")
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
    /// use transmission_rpc::types::{BasicAuth, Result, SessionSetArgs};
    /// use transmission_rpc::TransClient;
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
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
    pub async fn session_set(&mut self, args: SessionSetArgs) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_set(args, None)).await
    }

    /// Performs a session-set request that can be tracked by `tag`.
    pub async fn session_set_tagged(
        &mut self,
        args: SessionSetArgs,
        tag: Tag,
    ) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_set(args, Some(tag))).await
    }

    /// Performs a session get call.
    ///
    /// # Arguments
    ///
    /// * `args` - An optional collection of [`SessionGetField`]s to request. Specifying either
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
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{
    ///     BasicAuth, Result, RpcResponse, SessionGet, SessionGetField, Tag,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let mut client;
    ///     if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
    ///         client = TransClient::with_auth(url.parse()?, BasicAuth { user, password });
    ///     } else {
    ///         client = TransClient::new(url.parse()?);
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
    pub async fn session_get(&mut self, fields: Option<Vec<SessionGetField>>)
        -> Result<RpcResponse<SessionGet>>
    {
        self.call(RpcRequest::session_get(fields.map(Into::into), None)).await
    }

    /// Performs a session-get request that can be tracked by `tag`.
    pub async fn session_get_tagged(&mut self, fields: Option<Vec<SessionGetField>>, tag: Tag)
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<SessionStats>> = client.session_stats().await;
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
    pub async fn session_stats(&mut self) -> Result<RpcResponse<SessionStats>> {
        self.call(RpcRequest::session_stats(None)).await
    }

    /// Performs a session-stats request that can be tracked by `tag`.
    pub async fn session_stats_tagged(&mut self, tag: Tag) -> Result<RpcResponse<SessionStats>> {
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
    /// use transmission_rpc::types::{BasicAuth, Result};
    /// use transmission_rpc::TransClient;
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
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
    pub async fn session_close(&mut self) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::session_close(None)).await
    }

    /// Performs a session-close request that can be tracked by `tag`.
    pub async fn session_close_tagged(&mut self, tag: Tag) -> Result<RpcResponse<Nothing>> {
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<BlocklistUpdate>> =
    ///         client.blocklist_update().await;
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
    pub async fn blocklist_update(&mut self) -> Result<RpcResponse<BlocklistUpdate>> {
        self.call(RpcRequest::blocklist_update(None)).await
    }

    /// Performs a blocklist-update request that can be tracked by `tag`.
    pub async fn blocklist_update_tagged(
        &mut self,
        tag: Tag,
    ) -> Result<RpcResponse<BlocklistUpdate>> {
        self.call(RpcRequest::blocklist_update(Some(tag))).await
    }

    /// Performs a free space call
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
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
    pub async fn free_space(&mut self, path: String) -> Result<RpcResponse<FreeSpace>> {
        self.call(RpcRequest::free_space(path, None)).await
    }

    /// Performs a free-space request that can be tracked by `tag`.
    pub async fn free_space_tagged(
        &mut self,
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
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
    pub async fn port_test(&mut self) -> Result<RpcResponse<PortTest>> {
        self.call(RpcRequest::port_test(None)).await
    }

    /// Performs a port-test request that can be tracked by `tag`.
    pub async fn port_test_tagged(&mut self, tag: Tag) -> Result<RpcResponse<PortTest>> {
        self.call(RpcRequest::port_test(Some(tag))).await
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
    pub async fn queue_move_top<I>(&mut self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_top(ids, None)).await
    }

    /// Performs a `queue-move-top` request that can be tracked by `tag`.
    pub async fn queue_move_top_tagged<I>(&mut self, ids: I, tag: Tag)
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
    pub async fn queue_move_up<I>(&mut self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_up(ids, None)).await
    }

    /// Performs a `queue-move-up` request that can be tracked by `tag`.
    pub async fn queue_move_up_tagged<I>(&mut self, ids: I, tag: Tag)
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
    pub async fn queue_move_down<I>(&mut self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_down(ids, None)).await
    }

    /// Performs a `queue-move-down` request that can be tracked by `tag`.
    pub async fn queue_move_down_tagged<I>(&mut self, ids: I, tag: Tag)
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
    pub async fn queue_move_bottom<I>(&mut self, ids: I) -> Result<RpcResponse<Nothing>>
    where
        I: IntoIterator<Item = Id>,
    {
        self.call(RpcRequest::queue_move_bottom(ids, None)).await
    }

    /// Performs a `queue-move-bottom` request that can be tracked by `tag`.
    pub async fn queue_move_bottom_tagged<I>(&mut self, ids: I, tag: Tag)
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
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
    ///             tag1,
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
        &mut self,
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
        &mut self,
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url, basic_auth);
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
        &mut self,
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
        &mut self,
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
    ///     let res1: RpcResponse<Nothing> = client
    ///         .torrent_action(TorrentAction::Start, vec![Id::Id(1)])
    ///         .await?;
    ///     assert_eq!(res1.tag, None);
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
        &mut self,
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
        &mut self,
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<Nothing> =
    ///         client.torrent_remove(vec![Id::Id(1)], false).await?;
    ///     assert_eq!(res.tag, None);
    ///     println!("Remove result: {:?}", &res.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_remove<I>(
        &mut self,
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
        &mut self,
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<Nothing> = client
    ///         .torrent_set_location(
    ///             vec![Id::Id(1)],
    ///             String::from("/new/location"),
    ///             Option::from(false),
    ///         )
    ///         .await?;
    ///     assert_eq!(res.tag, None);
    ///     println!("Set-location result: {:?}", &res.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_set_location<I>(
        &mut self,
        ids: I,
        location: String,
        move_from: Option<bool>,
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
        &mut self,
        ids: I,
        location: String,
        move_from: Option<bool>,
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
    ///     types::{BasicAuth, Id, Result, RpcResponse, TorrentRenamePath},
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<TorrentRenamePath> = client
    ///         .torrent_rename_path(
    ///             vec![Id::Id(1)],
    ///             String::from("Folder/OldFile.jpg"),
    ///             String::from("NewFile.jpg"),
    ///         )
    ///         .await?;
    ///     assert_eq!(res.tag, None);
    ///     println!("rename-path result: {:#?}", res);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_rename_path<I>(
        &mut self,
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
        &mut self,
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
    ///     TransClient,
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
    ///     let mut client = TransClient::with_auth(url.parse()?, basic_auth);
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
        &mut self,
        add: TorrentAddArgs,
    ) -> Result<RpcResponse<TorrentAddedOrDuplicate>> {
        assert!(
            add.metainfo.is_some() || add.filename.is_some(),
            "Metainfo or Filename should be provided"
        );
        self.call(RpcRequest::torrent_add(add, None)).await
    }

    /// Performs a `torrent-add` request that can be tracked by `tag`.
    pub async fn torrent_add_tagged(
        &mut self,
        add: TorrentAddArgs,
        tag: Tag,
    ) -> Result<RpcResponse<TorrentAddedOrDuplicate>> {
        assert!(
            add.metainfo.is_some() || add.filename.is_some(),
            "Metainfo or Filename should be provided"
        );
        self.call(RpcRequest::torrent_add(add, Some(tag))).await
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
    pub async fn group_get(&mut self, groups: Option<Vec<String>>)
        -> Result<RpcResponse<Vec<GroupGet>>>
    {
        self.call(RpcRequest::group_get(groups, None)).await
    }

    /// Performs a group-get request that can be tracked by `tag`.
    pub async fn group_get_tagged(&mut self, groups: Option<Vec<String>>, tag: Tag)
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
    pub async fn group_set(&mut self, args: GroupSetArgs) -> Result<RpcResponse<Nothing>>
    {
        self.call(RpcRequest::group_set(args, None)).await
    }

    /// Performs a group-set request that can be tracked by `tag`.
    pub async fn group_set_tagged(&mut self, args: GroupSetArgs, tag: Tag)
        -> Result<RpcResponse<Nothing>>
    {
        self.call(RpcRequest::group_set(args, Some(tag))).await
    }

    /// Performs a JRPC call to the server
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    async fn call<RS>(&mut self, mut request: RpcRequest) -> Result<RpcResponse<RS>>
    where
        RS: RpcResponseArgument + DeserializeOwned + std::fmt::Debug,
    {
        let mut remaining_retries = MAX_RETRIES;
        loop {
            remaining_retries = remaining_retries
                .checked_sub(1)
                .ok_or(TransError::MaxRetriesReached)?;

            if let Some(semver) = &self.semver {
                // Flag that the request should be transformed into JSON-RPC.
                request.jsonrpc = (semver >= &"6.0.0".parse::<Version>()?)
                    .then_some(JSON_RPC_VERSION_2_0.to_string());
            }

            debug!("Loaded auth: {:?}", &self.auth);
            let rq = match &self.session_id {
                None => self.rpc_request(),
                Some(id) => self.rpc_request().header("X-Transmission-Session-Id", id),
            }
            .json(&request);

            debug!(
                "Request body: {:?}",
                rq.try_clone()
                    .expect("Unable to get the request body")
                    .body_string()?
            );

            let rsp: reqwest::Response = rq.send().await?;
            if matches!(rsp.status(), StatusCode::CONFLICT) {
                // "Starting from rpc-version-semver 6.0.0, Transmission returns the RPC version in
                //  an HTTP header X-Transmission-Rpc-Version: {rpc_version_semver} in the CSRF
                //  HTTP 409 response. This is so that clients supporting both JSON-RPC and the old
                //  bespoke API can determine which scheme to use without making any extra
                //  requests. Example: X-Transmission-Rpc-Version: 6.0.0"
                self.semver = rsp
                    .headers()
                    .get("X-Transmission-Rpc-Version")
                    .map(HeaderValue::to_str)
                    .transpose()?
                    .map(Version::parse)
                    .transpose()?;
                if let Some(semver) = &self.semver {
                    debug!("Got rpc-semver: {}", semver);
                }

                let session_id = rsp
                    .headers()
                    .get("X-Transmission-Session-Id")
                    .ok_or(TransError::NoSessionIdReceived)?
                    .to_str()?;
                self.session_id = Some(String::from(session_id));

                debug!("Got new session_id: {}. Retrying request.", session_id);
            } else {
                let rpc_response: RpcResponse<RS> = match request.jsonrpc.is_some() {
                    true => {
                        let resp = rsp.json::<JsonRpcResponse<RS>>().await?;
                        debug!("JSON-RPC response body: {:#?}", resp);
                        if resp.jsonrpc != JSON_RPC_VERSION_2_0 {
                            // This probably means that the request was handled by a new
                            // Transmission version with an upgrade JSON-RPC protocol.
                            let err = TransError::UnhandledJsonRpcVersion(resp.jsonrpc.clone());
                            return Err(Box::new(err));
                        }
                        resp.into()
                    },
                    false => {
                        let resp = rsp.json().await?;
                        debug!("Response body: {:#?}", resp);
                        resp
                    },
                };

                return Ok(rpc_response);
            }
        }
    }
}

trait BodyString {
    fn body_string(self) -> Result<String>;
}

impl BodyString for reqwest::RequestBuilder {
    fn body_string(self) -> Result<String> {
        let rq = self.build()?;
        let body = rq.body().unwrap().as_bytes().unwrap();
        Ok(std::str::from_utf8(body)?.to_string())
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
        env_logger::init();
        let url = env::var("TURL")?;

        let mut client;
        if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
            client = TransClient::with_auth(url.parse()?, BasicAuth { user, password });
        } else {
            client = TransClient::new(url.parse()?);
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
