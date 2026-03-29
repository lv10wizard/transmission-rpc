use std::collections::HashMap;

use serde::Deserialize;
use serde::de::Deserializer;

use super::{IpProtocol, Tag};
use crate::json_rpc::{JsonRpcResponse, JsonRpcResult};

pub use session_get::{SessionGet, SessionGetUnits};
pub use session_stats::{SessionStats, Stats};
pub use torrent::{
    ErrorType, File, FileStat, Peer, PeersFrom, Torrent, TorrentStatus, Tracker, TrackerStat,
    TrackerState, WebseedsEx,
};

mod session_get;
mod session_stats;
mod torrent;

#[cfg(test)]
mod torrent_get_serde_tests;

const SUCCESS: &'static str = "success";

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T: RpcResponseArgument> {
    /// "An optional `arguments` object of key/value pairs. Its keys contents are defined by the
    ///  `method` and `arguments` of the original request."
    pub arguments: T,
    /// "A required `result` string whose value MUST be `success` on success, or an error string on
    ///  failure."
    pub result: String,
    /// "An optional `tag` number as described in [`2.1`]." <sup>[[1]]</sup>
    ///
    /// [`2.1`]: <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#21-requests>
    /// [1]: <https://github.com/transmission/transmission/blob/4.0.6/libtransmission/rpcimpl.cc#L2520>
    pub tag: Option<Tag>,
}

/// Converts a `JsonRpcResponse<T>` into a backwards compatible `RpcReponse<T>` for external use.
impl<T: RpcResponseArgument> From<JsonRpcResponse<T>> for RpcResponse<T> {
    fn from(value: JsonRpcResponse<T>) -> Self {
        let result = match &value.result {
            JsonRpcResult::Result(_) => SUCCESS.to_string(),
            JsonRpcResult::Error(err) => format!("{} (code: {})", err.message, err.code),
        };

        Self {
            arguments: match value.result {
                JsonRpcResult::Result(data) => data,
                JsonRpcResult::Error(_) => T::default(),
            },
            result,
            tag: value.id.map(Into::into),
        }
    }
}

impl<T: RpcResponseArgument> RpcResponse<T> {
    pub fn is_ok(&self) -> bool {
        self.result == SUCCESS
    }
}
/// Rpc response data returned by the server.
///
/// This trait is bound by [`Default`] so that [`RpcResponse`] can maintain pre-semver-6.0.0
/// compatibility in case of a JSON-RPC protocol error.
pub trait RpcResponseArgument: Default {}

impl RpcResponseArgument for SessionGet {}
impl RpcResponseArgument for SessionStats {}

/// Represents the result of a [`blocklist_update`] by fetching the current [`blocklist_url`].
///
/// [blocklist]: <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
/// [`blocklist_update`]: crate::TransClient::blocklist_update
/// [`blocklist_url`]: SessionGet::blocklist_url
#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct BlocklistUpdate {
    /// The current number of rules in the [blocklist].
    ///
    /// [blocklist]: <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    #[serde(alias = "blocklist_size")]
    pub blocklist_size: Option<i32>, // TODO: Option<_> -> u64
}
impl RpcResponseArgument for BlocklistUpdate {}

/// Represents the result of a [`free_space`] query.
///
/// [`free_space`]: crate::TransClient::free_space
#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct FreeSpace {
    /// The directory that was queried in the [`free_space`] request.
    ///
    /// [`free_space`]: crate::TransClient::free_space
    pub path: String,
    /// The amount of free space, in bytes, of the [`path`] directory.
    ///
    /// [`path`]: Self::path
    #[serde(alias = "size_bytes")]
    pub size_bytes: i64, // TODO: u64
    /// The total capacity, in bytes, of the [`path`] directory.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [`path`]: Self::path
    #[serde(alias = "total_size")]
    pub total_size: Option<i64>, // TODO: Option<u64>
}
impl RpcResponseArgument for FreeSpace {}

/// Represents the result of a [`port_test`] query.
///
/// [`port_test`]: crate::TransClient::port_test
#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct PortTest {
    /// True if the [`peer_port`] is open, false if closed.
    ///
    /// [`peer_port`]: SessionGet::peer_port
    #[serde(alias = "port_is_open")]
    pub port_is_open: bool,
    /// Which IP version the test was performed on. This may be `None` either if the rpc server
    /// version is too low (unimplemented) or if the version could not be determined.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(alias = "ip_protocol")]
    pub ip_protocol: Option<IpProtocol>,
}
impl RpcResponseArgument for PortTest {}

#[derive(Deserialize, Default, Debug)]
pub struct Torrents<T> {
    pub torrents: Vec<T>,
}
impl RpcResponseArgument for Torrents<Torrent> {}

#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Default, Debug, Clone)]
pub enum TorrentAddedOrDuplicate {
    TorrentDuplicate(Torrent),
    TorrentAdded(Torrent),
    #[default]
    Error,
}

impl RpcResponseArgument for TorrentAddedOrDuplicate {}

impl<'de> Deserialize<'de> for TorrentAddedOrDuplicate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let mut res: HashMap<String, Torrent> = Deserialize::deserialize(deserializer)?;

        let added = res.remove("torrent-added");
        let duplicate = res.remove("torrent-duplicate");
        match (added, duplicate) {
            (Some(torrent), None) => Ok(TorrentAddedOrDuplicate::TorrentAdded(torrent)),
            (None, Some(torrent)) => Ok(TorrentAddedOrDuplicate::TorrentDuplicate(torrent)),
            _ => Ok(TorrentAddedOrDuplicate::Error),
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TorrentRenamePath {
    pub path: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}
impl RpcResponseArgument for TorrentRenamePath {}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GroupGet {
    #[serde(alias = "honorsSessionLimits")]
    #[serde(alias = "honors_session_limits")]
    pub honors_session_limits: bool,
    pub name: String,
    #[serde(alias = "speed_limit_down_enabled")]
    pub speed_limit_down_enabled: bool,
    #[serde(alias = "speed_limit_down")]
    pub speed_limit_down: u64,
    #[serde(alias = "speed_limit_up_enabled")]
    pub speed_limit_up_enabled: bool,
    #[serde(alias = "speed_limit_up")]
    pub speed_limit_up: u64,
}
impl RpcResponseArgument for Vec<GroupGet> {}

#[cfg(test)]
mod serde_tests {
    use crate::{
        json_rpc::JsonRpcResponse,
        types::{
            BlocklistUpdate, FreeSpace, IpProtocol, PortTest, Result, RpcResponse,
            TorrentAddedOrDuplicate,
        },
    };
    use serde_json;
    use serde_json::Value;

    #[test]
    fn test_torrent_added_failure_with_torrent_added_or_duplicate() {
        let v: RpcResponse<TorrentAddedOrDuplicate> =
            serde_json::from_str(torrent_added_failure()).expect("Failure expected");
        println!("{v:#?}");
        assert!(!v.is_ok());
    }

    #[test]
    fn test_torrent_added_success_with_torrent_added_or_duplicate() -> Result<()> {
        let v: RpcResponse<TorrentAddedOrDuplicate> =
            serde_json::from_str(torrent_added_success())?;
        println!("{v:#?}");
        Ok(())
    }

    #[test]
    fn test_torrent_added_success_with_value() -> Result<()> {
        let v: Value = serde_json::from_str(torrent_added_success())?;
        println!("{v:?} {}", serde_json::to_string_pretty(&v).expect(""));
        Ok(())
    }

    #[test]
    fn test_torrent_added_failure_with_value() -> Result<()> {
        let v: Value = serde_json::from_str(torrent_added_failure())?;
        println!("{v:?} {}", serde_json::to_string_pretty(&v).expect(""));
        Ok(())
    }

    fn torrent_added_success() -> &'static str {
        r#"
        {
            "arguments": {
                "torrent-added": {
                    "hashString": "bbdaece7c8daa85e1619469ab25d422a612cf923",
                    "id": 2,
                    "name": "toto.torrent"}
                },
            "result": "success"
        }
        "#
    }

    fn torrent_added_failure() -> &'static str {
        r#"
        {
            "arguments": {},
            "result": "download directory path is not absolute"
        }
        "#
    }

    // ---------------------------------------------------------------------------------------------

    #[test]
    fn blocklist_update_v300() -> Result<()> {
        let resp = serde_json::from_str::<RpcResponse<BlocklistUpdate>>(
            r#"
            {
              "arguments": {
                "blocklist-size": 1023
              },
              "result": "success",
              "tag": 12345
            }
            "#
        )?;

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.blocklist_size, Some(1023));
        Ok(())
    }

    #[test]
    fn blocklist_update_v411() -> Result<()> {
        let resp: RpcResponse<_> = serde_json::from_str::<JsonRpcResponse<BlocklistUpdate>>(
            r#"
            {
              "id": 12345,
              "jsonrpc": "2.0",
              "result": {
                "blocklist_size": 2041
              }
            }
            "#
        )?
        .into();

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.blocklist_size, Some(2041));
        Ok(())
    }

    #[test]
    fn free_space_v300() -> Result<()> {
        let resp = serde_json::from_str::<RpcResponse<FreeSpace>>(
            r#"
            {
              "arguments": {
                "path": "/incomplete",
                "size-bytes": 456789
              },
              "result": "success",
              "tag": 12345
            }
            "#
        )?;

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.path, "/incomplete".to_string());
        assert_eq!(resp.arguments.size_bytes, 456789);
        Ok(())
    }

    #[test]
    fn free_space_v411() -> Result<()> {
        let resp: RpcResponse<_> = serde_json::from_str::<JsonRpcResponse<FreeSpace>>(
            r#"
            {
              "id": 12345,
              "jsonrpc": "2.0",
              "result": {
                "path": "/downloads",
                "size_bytes": 4321,
                "total_size": 257698037760
              }
            }
            "#
        )?
        .into();

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.path, "/downloads".to_string());
        assert_eq!(resp.arguments.size_bytes, 4321);
        assert_eq!(resp.arguments.total_size, Some(257698037760));
        Ok(())
    }

    #[test]
    fn port_test_v300() -> Result<()> {
        let resp = serde_json::from_str::<RpcResponse<PortTest>>(
            r#"
            {
              "arguments": {
                "port-is-open": false
              },
              "result": "success",
              "tag": 12345
            }
            "#
        )?;

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.port_is_open, false);
        assert_eq!(resp.arguments.ip_protocol, None);
        Ok(())
    }

    #[test]
    fn port_test_v411() -> Result<()> {
        let resp: RpcResponse<_> = serde_json::from_str::<JsonRpcResponse<PortTest>>(
            r#"
            {
              "id": 12345,
              "jsonrpc": "2.0",
              "result": {
                "port_is_open": true,
                "ip_protocol": "ipv6"
              }
            }
            "#
        )?
        .into();

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.port_is_open, true);
        assert_eq!(resp.arguments.ip_protocol, Some(IpProtocol::Ipv6));
        Ok(())
    }
}
