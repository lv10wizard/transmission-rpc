//! Defines types for [Transmission] RPC handling and communication.
//!
//! [Transmission]: <https://transmissionbt.com/>

use std::fmt::{self, Display};

use bitflags::{self, parser};
use serde::{Deserialize, Serialize, de::Error as _};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(unused_imports)]
pub(crate) use self::request::{RpcRequest, SessionGetArgs};
pub use self::request::{
    ArgumentFields, GroupSetArgs, PortTestArgs, SessionGetField, SessionSetArgs, TorrentAction,
    TorrentAddArgs, TorrentGetField, TorrentRenamePathArgs, TorrentSetArgs, TrackerReplaceArgs,
    TrackerReplacePair,
};

pub use self::response::{
    BlocklistUpdate, ErrorType, FreeSpace, GroupGet, Nothing, PortTest, RpcResponse,
    RpcResponseArgument, SessionGet, SessionGetUnits, SessionStats, Torrent,
    TorrentAddedOrDuplicate, TorrentRenamePath, TorrentStatus, Torrents, TrackerState,
};

/// [`Torrent`] field sub-type. You probably won't need to interact with this directly.
pub use self::response::{File, FileStat, Peer, PeersFrom, Tracker, TrackerStat, WebseedsEx};

/// [`SessionStats`] current- and cumulative-stats sub-type. You probably won't need to interact
/// with this directly.
pub use self::response::Stats;

pub use self::tracker::{TrackerList, TrackerTier};

pub(crate) mod request;
mod response;
mod tracker;

pub(crate) const JSON_RPC_VERSION_2_0: &'static str = "2.0";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type TrackerId = u32;

/// [HTTP Basic Authentication] with a [Transmission] rpc server.
///
/// [HTTP Basic Authentication]: <https://en.wikipedia.org/wiki/Basic_access_authentication>
/// [Transmission]: <https://transmissionbt.com/>
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

/// Represents a unique identifier of a [`Torrent`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum Id {
    /// A unique numeric identifier of a torrent. Note that integer torrent ids are not stable
    /// across Transmission daemon restarts. Use [torrent hashes] if you need stable ids.
    ///
    /// [torrent hashes]: Torrent::hash_string
    Id(i64),
    /// A unique [SHA1] hash string identifier of a torrent.
    ///
    /// [SHA1]: <https://en.wikipedia.org/wiki/SHA-1>
    Hash(String),
}

/// Represents a [`Torrent`]'s bandwidth and file download priority.
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum Priority {
    Low = -1,
    Normal = 0,
    High = 1,
}

/// Represents how a [`Torrent`] handles its [`seed_idle_limit`] setting.
///
/// [`seed_idle_limit`]: Torrent::seed_idle_limit
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum IdleMode {
    /// Follow the global settings.
    Global = 0,
    /// Override the global settings, seeding until a certain idle time.
    Single = 1,
    /// Override the global settings, seeding regardless of activity.
    Unlimited = 2,
}

/// Represents how a [`Torrent`] handles its [`seed_ratio_limit`] setting.
///
/// [`seed_ratio_limit`]: Torrent::seed_idle_limit
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum RatioMode {
    /// Follow the global settings.
    Global = 0,
    /// Override the global settings, seeding until a certain ratio.
    Single = 1,
    /// Override the global settings, seeding regardless of ratio.
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
    /// > Renamed from `tolerated` to `allowed` in Transmission 4.1.0.
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
    /// let mwf = AltSpeedDay::MONDAY | AltSpeedDay::WEDNESDAY | AltSpeedDay::FRIDAY;
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
    Tcp,

    /// [Micro Transport Protocol](https://en.wikipedia.org/wiki/Micro_Transport_Protocol) (aka
    /// "μTP" or "uTP")
    Utp,
}

impl Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Tcp => "tcp",
            Self::Utp => "utp",
        })
    }
}

/// Serialization helper [`SessionSetArgs`] [`alt_speed_time_begin`] and [`alt_speed_time_end`]
/// type to clamp the number of minutes to within a single day.
///
/// [`alt_speed_time_begin`]: SessionSetArgs::alt_speed_time_begin
/// [`alt_speed_time_end`]: SessionSetArgs::alt_speed_time_end
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MinutesAfterMidnight(pub u64);

impl MinutesAfterMidnight {
    const MINUTES_PER_DAY: u64 = 60 * 24;

    /// Clamps the number of minutes into a single day.
    fn clamp(&self) -> u64 {
        self.0 % Self::MINUTES_PER_DAY
    }
}

impl Serialize for MinutesAfterMidnight {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_u64(self.clamp())
    }
}

impl Display for MinutesAfterMidnight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} minutes after midnight", self.clamp())
    }
}

/// Represents the IP version used for [`port_test`].
///
/// [`port_test`]: crate::TransClient::port_test
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum IpProtocol {
    Ipv4,
    Ipv6,
}

/// Represents an arbitrary `tag` number used by clients to track responses. <sup>[[1]][[2]]</sup>
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

#[cfg(test)]
mod serde_tests {
    use test_case::test_case;

    use super::*;

    #[test_case(AltSpeedDay::SUNDAY => "1" ; "sunday")]
    #[test_case(AltSpeedDay::MONDAY => "2" ; "monday")]
    #[test_case(AltSpeedDay::TUESDAY => "4" ; "tuesday")]
    #[test_case(AltSpeedDay::WEDNESDAY => "8" ; "wednesday")]
    #[test_case(AltSpeedDay::THURSDAY => "16" ; "thursday")]
    #[test_case(AltSpeedDay::FRIDAY => "32" ; "friday")]
    #[test_case(AltSpeedDay::SATURDAY => "64" ; "saturday")]
    #[test_case(AltSpeedDay::WEEKDAY => "62" ; "all weekdays")]
    #[test_case(AltSpeedDay::WEEKEND => "65" ; "weekend")]
    #[test_case(AltSpeedDay::all() => "127" ; "every day")]
    #[test_case(AltSpeedDay::empty() => "0" ; "empty")]
    fn alt_speed_day_serialize(day: AltSpeedDay) -> String {
        match serde_json::to_string(&day) {
            Ok(s) => s,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case("1" => AltSpeedDay::SUNDAY ; "sunday")]
    #[test_case("2" => AltSpeedDay::MONDAY ; "monday")]
    #[test_case("4" => AltSpeedDay::TUESDAY ; "tuesday")]
    #[test_case("8" => AltSpeedDay::WEDNESDAY ; "wednesday")]
    #[test_case("16" => AltSpeedDay::THURSDAY ; "thursday")]
    #[test_case("32" => AltSpeedDay::FRIDAY ; "friday")]
    #[test_case("64" => AltSpeedDay::SATURDAY ; "saturday")]
    #[test_case("62" => AltSpeedDay::WEEKDAY ; "all weekdays")]
    #[test_case("65" => AltSpeedDay::WEEKEND ; "weekend")]
    #[test_case("127" => AltSpeedDay::all() ; "every day")]
    #[test_case("0" => AltSpeedDay::empty() ; "empty")]
    fn alt_speed_day_deserialize(num: &str) -> AltSpeedDay {
        match serde_json::from_str(num) {
            Ok(day) => day,
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn ip_protocol_v4_serialize() -> Result<()> {
        assert_eq!(serde_json::to_string(&IpProtocol::Ipv4)?, "\"ipv4\"");
        Ok(())
    }

    #[test]
    fn ip_protocol_v6_serialize() -> Result<()> {
        assert_eq!(serde_json::to_string(&IpProtocol::Ipv6)?, "\"ipv6\"");
        Ok(())
    }

    #[test]
    fn ip_protocol_v4_deserialize() -> Result<()> {
        assert_eq!(serde_json::from_str::<IpProtocol>("\"ipv4\"")?, IpProtocol::Ipv4);
        Ok(())
    }

    #[test]
    fn ip_protocol_v6_deserialize() -> Result<()> {
        assert_eq!(serde_json::from_str::<IpProtocol>("\"ipv6\"")?, IpProtocol::Ipv6);
        Ok(())
    }

    #[test_case("-1" => Priority::Low ; "low")]
    #[test_case("0" => Priority::Normal ; "normal")]
    #[test_case("1" => Priority::High ; "high")]
    #[test_case("-2" => panics "invalid value: -2, expected one of: -1, 0, 1"
        ; "invalid negative")]
    #[test_case("2" => panics "invalid value: 2, expected one of: -1, 0, 1" ; "invalid positive")]
    fn priority_deserialize(repr: &str) -> Priority {
        match serde_json::from_str(repr) {
            Ok(prio) => prio,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case("0" => IdleMode::Global ; "global")]
    #[test_case("1" => IdleMode::Single ; "single")]
    #[test_case("2" => IdleMode::Unlimited ; "unlimited")]
    #[test_case("-1" => panics "invalid value: -1, expected one of: 0, 1, 2"
        ; "invalid negative")]
    #[test_case("3" => panics "invalid value: 3, expected one of: 0, 1, 2"
        ; "invalid positive")]
    fn idle_mode_deserialize(repr: &str) -> IdleMode {
        match serde_json::from_str(repr) {
            Ok(prio) => prio,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case("0" => RatioMode::Global ; "global")]
    #[test_case("1" => RatioMode::Single ; "single")]
    #[test_case("2" => RatioMode::Unlimited ; "unlimited")]
    #[test_case("-1" => panics "invalid value: -1, expected one of: 0, 1, 2"
        ; "invalid negative")]
    #[test_case("3" => panics "invalid value: 3, expected one of: 0, 1, 2"
        ; "invalid positive")]
    fn ratio_mode_deserialize(repr: &str) -> RatioMode {
        match serde_json::from_str(repr) {
            Ok(prio) => prio,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case(Transport::Tcp => "\"tcp\"" ; "tcp")]
    #[test_case(Transport::Utp => "\"utp\"" ; "utp")]
    fn transport_serialize(transport: Transport) -> String {
        match serde_json::to_string(&transport) {
            Ok(s) => s,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case("\"tcp\"" => Transport::Tcp ; "tcp")]
    #[test_case("\"utp\"" => Transport::Utp ; "utp")]
    #[test_case("\"unimplemented foo bar\""
        => panics "unknown variant `unimplemented foo bar`, expected `tcp` or `utp`"
        ; "unimplemented")]
    fn transport_deserialize(repr: &str) -> Transport {
        match serde_json::from_str(repr) {
            Ok(transport) => transport,
            Err(err) => panic!("{err}"),
        }
    }
}
