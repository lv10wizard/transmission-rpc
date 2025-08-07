use serde::{Deserialize, Serialize};

mod request;
mod response;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

/// Represents an arbitrary `tag` number used by clients to track responses. <sup>[1][2]</sup>
///
/// [1]: <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#21-requests>
/// [2]: <https://github.com/transmission/transmission/blob/4.0.6/libtransmission/rpcimpl.cc#L2520>
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag(pub i64);

impl From<i64> for Tag {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<&i64> for Tag {
    fn from(value: &i64) -> Self {
        Self(*value)
    }
}

pub(crate) use self::request::RpcRequest;
pub use self::request::{
    ArgumentFields, Id, IdleMode, Priority, RatioMode, SessionSetArgs, TorrentAction,
    TorrentAddArgs, TorrentGetField, TorrentRenamePathArgs, TorrentSetArgs, TrackerList,
};

pub use self::response::{
    BlocklistUpdate, ErrorType, FreeSpace, Nothing, PortTest, RpcResponse, RpcResponseArgument,
    SessionClose, SessionGet, SessionSet, SessionStats, Torrent, TorrentAddedOrDuplicate,
    TorrentRenamePath, TorrentStatus, Torrents, TrackerState,
};

/// [`Torrent`] field sub-type. You probably won't need to interact with this directly.
pub use self::response::{File, FileStat, Peer, PeersFrom, TrackerStat, Trackers};
