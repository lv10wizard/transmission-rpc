use std::fmt::{self, Display};

use bitflags::{self, parser};
use serde::{Deserialize, Serialize, de::Error as _};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(unused_imports)]
pub(crate) use self::request::{RpcRequest, SessionGetArgs};
pub use self::request::{
    ArgumentFields, GroupSetArgs, SessionGetField, SessionSetArgs, TorrentAction, TorrentAddArgs,
    TorrentGetField, TorrentRenamePathArgs, TorrentSetArgs, TrackerList,
};

pub use self::response::{
    BlocklistUpdate, ErrorType, FreeSpace, GroupGet, Nothing, PortTest, RpcResponse,
    RpcResponseArgument, SessionGet, SessionStats, Torrent, TorrentAddedOrDuplicate,
    TorrentRenamePath, TorrentStatus, Torrents, TrackerState, WebseedsEx,
};

/// [`Torrent`] field sub-type. You probably won't need to interact with this directly.
pub use self::response::{File, FileStat, Peer, PeersFrom, TrackerStat, Trackers};

mod request;
mod response;

pub(crate) const JSON_RPC_VERSION_2_0: &'static str = "2.0";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum Priority {
    Low = -1,
    Normal = 0,
    High = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum IdleMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum RatioMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

/// Represents how transmission handles peer connection encryption.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Encryption {
    /// Encrypt all peer connections.
    Required,
    /// Prefer encrypted peer connections.
    Preferred,
    /// Prefer unencrypted peer connections.
    ///
    /// > Renamed from "tolerated" to "allowed" in Transmission 4.1.0.
    #[serde(alias = "allowed")]
    Tolerated,
}

// XXX: Is there a way to utilize the serde implementation?
impl Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Required => "required",
            Self::Preferred => "preferred",
            Self::Tolerated => "tolerated",
        })
    }
}

/// Semver-6.0.0 compatibility serialization helper enum for [`Encryption`].
#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
enum EncryptionCompat {
    Required,
    Preferred,
    Allowed,
}

impl From<Encryption> for EncryptionCompat {
    fn from(value: Encryption) -> Self {
        match value {
            Encryption::Required => Self::Required,
            Encryption::Preferred => Self::Preferred,
            Encryption::Tolerated => Self::Allowed,
        }
    }
}

bitflags::bitflags! {
    /// One or more day(s) of the week represented as a bitfield used for session
    /// `alt-speed-time-day` (the day(s) to turn on alt speeds, ie. turtle mode).
    ///
    /// See: [`tr_sched_day`]
    ///
    /// # Example
    ///
    /// To specify weekends (Saturday and Sunday):
    ///
    /// ```rust
    /// let weekend = AltSpeedDay::WEEKEND;
    /// ```
    ///
    /// or:
    ///
    /// ```rust
    /// let weekend = AltSpeedDay::SATURDAY | AltSpeedDay::SUNDAY;
    /// ```
    ///
    /// To specify Monday, Wednesday, and Friday:
    ///
    /// ```rust
    /// let mwf = AltSpeedDay::Monday | AltSpeedDay::WEDNESDAY | AltSpeedDay::FRIDAY;
    /// ```
    ///
    /// [`tr_sched_day`]: https://github.com/transmission/transmission/blob/08ec7fb7c7b9c77ba52ff84d853833d70fd6f59b/libtransmission/transmission.h#L515-L527
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct AltSpeedDay: u8 {
        /// 0b_0000_0001
        const SUNDAY = 1 << 0;
        /// 0b_0000_0010
        const MONDAY = 1 << 1;
        /// 0b_0000_0100
        const TUESDAY = 1 << 2;
        /// 0b_0000_1000
        const WEDNESDAY = 1 << 3;
        /// 0b_0001_0000
        const THURSDAY = 1 << 4;
        /// 0b_0010_0000
        const FRIDAY = 1 << 5;
        /// 0b_0100_0000
        const SATURDAY = 1 << 6;
        /// 0b_0011_1110
        const WEEKDAY = {
            Self::MONDAY.bits() | Self::TUESDAY.bits() | Self::WEDNESDAY.bits()
                | Self::THURSDAY.bits() | Self::FRIDAY.bits()
        };
        /// 0b_0100_0001
        const WEEKEND = Self::SUNDAY.bits() | Self::SATURDAY.bits();
    }
}

impl Serialize for AltSpeedDay {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for AltSpeedDay {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let value: u8 = Deserialize::deserialize(deserializer)?;
        parser::from_str(&format!("0x{value:x}"))
            .map_err(D::Error::custom)
    }
}

/// Represents valid transport protocols recognized by various versions of Transmission.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    /// [Transmission control
    /// protocol](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
    TCP,

    /// [Micro Transport Protocol](https://en.wikipedia.org/wiki/Micro_Transport_Protocol) (aka
    /// "μTP" or "uTP")
    UTP,
}

impl Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::TCP => "tcp",
            Self::UTP => "utp",
        })
    }
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

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
