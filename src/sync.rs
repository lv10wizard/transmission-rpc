//! Sharable version of `TransClient`.
//!
//! It lifted the requirement of `&mut self` on
//! all requests methods by using a lock on inner state. This may introduce some
//! overhead so choose as needed.

use std::{ops::Deref, sync::RwLock};

use reqwest::{Client, StatusCode, Url, header::CONTENT_TYPE};
use serde::de::DeserializeOwned;

use crate::{
    BodyString, MAX_RETRIES, TransError,
    types::{
        BasicAuth, BlocklistUpdate, FreeSpace, Id, Nothing, PortTest, Result, RpcRequest,
        RpcResponse, RpcResponseArgument, SessionClose, SessionGet, SessionSet, SessionSetArgs,
        SessionStats, Tag, Torrent, TorrentAction, TorrentAddArgs, TorrentAddedOrDuplicate,
        TorrentGetField, TorrentRenamePath, TorrentSetArgs, Torrents,
    },
};

pub struct SharableTransClient {
    url: Url,
    auth: Option<BasicAuth>,
    session_id: RwLock<Option<String>>,
    client: Client,
}

impl SharableTransClient {
    /// Returns HTTP(S) client with configured Basic Auth
    #[must_use]
    pub fn with_auth(url: Url, basic_auth: BasicAuth) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: Some(basic_auth),
            session_id: RwLock::new(None),
            client: Client::new(),
        }
    }

    /// Returns HTTP(S) client
    #[must_use]
    pub fn new(url: Url) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: None,
            session_id: RwLock::new(None),
            client: Client::new(),
        }
    }

    #[must_use]
    pub fn new_with_client(url: Url, client: Client) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: None,
            session_id: RwLock::new(None),
            client,
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
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, SessionSet, SessionSetArgs},
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
    ///     let response: Result<RpcResponse<SessionSet>> = client.session_set(args).await;
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
    pub async fn session_set(&self, args: SessionSetArgs) -> Result<RpcResponse<SessionSet>> {
        self.call(RpcRequest::session_set(args, None)).await
    }

    /// Performs a session-set request that can be tracked by `tag`.
    pub async fn session_set_tagged(
        &self,
        args: SessionSetArgs,
        tag: Tag,
    ) -> Result<RpcResponse<SessionSet>> {
        self.call(RpcRequest::session_set(args, Some(tag))).await
    }

    /// Performs a session get call
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
    ///     types::{BasicAuth, Result, RpcResponse, SessionGet},
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
    ///     let response: Result<RpcResponse<SessionSet>> = client.session_get(args).await;
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
    pub async fn session_get(&self) -> Result<RpcResponse<SessionGet>> {
        self.call(RpcRequest::session_get(None)).await
    }

    /// Performs a session-get request that can be tracked by `tag`.
    pub async fn session_get_tagged(&self, tag: Tag) -> Result<RpcResponse<SessionGet>> {
        self.call(RpcRequest::session_get(Some(tag))).await
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
    ///     types::{BasicAuth, Result, RpcResponse, SessionClose},
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
    ///     let response: Result<RpcResponse<SessionSet>> = client.session_close().await;
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
    pub async fn session_close(&self) -> Result<RpcResponse<SessionClose>> {
        self.call(RpcRequest::session_close(None)).await
    }

    /// Performs a session-close request that can be tracked by `tag`.
    pub async fn session_close_tagged(&self, tag: Tag) -> Result<RpcResponse<SessionClose>> {
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
    pub async fn port_test(&self) -> Result<RpcResponse<PortTest>> {
        self.call(RpcRequest::port_test(None)).await
    }

    /// Performs a port-test request that can be tracked by `tag`.
    pub async fn port_test_tagged(&self, tag: Tag) -> Result<RpcResponse<PortTest>> {
        self.call(RpcRequest::port_test(Some(tag))).await
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
        &self,
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
        &self,
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
        assert!(
            !(add.metainfo.is_none() && add.filename.is_none()),
            "Metainfo or Filename should be provided"
        );
        self.call(RpcRequest::torrent_add(add, None)).await
    }

    /// Performs a `torrent-add` request that can be tracked by `tag`.
    pub async fn torrent_add_tagged(
        &self,
        add: TorrentAddArgs,
        tag: Tag,
    ) -> Result<RpcResponse<TorrentAddedOrDuplicate>> {
        assert!(
            add.metainfo.is_some() || add.filename.is_some(),
            "Metainfo or Filename should be provided"
        );
        self.call(RpcRequest::torrent_add(add, Some(tag))).await
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
            let rq = match &self.session_id.read().expect("lock being poisoned").deref() {
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

            debug!("Response: {:?}", &rsp);
            if matches!(rsp.status(), StatusCode::CONFLICT) {
                let session_id = rsp
                    .headers()
                    .get("X-Transmission-Session-Id")
                    .ok_or(TransError::NoSessionIdReceived)?
                    .to_str()?;
                *self.session_id.write().expect("lock being poisoned") =
                    Some(String::from(session_id));

                debug!("Got new session_id: {}. Retrying request.", session_id);
            } else {
                let rpc_response: RpcResponse<RS> = rsp.json().await?;
                debug!("Response body: {:#?}", rpc_response);

                return Ok(rpc_response);
            }
        }
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
