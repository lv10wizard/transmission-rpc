use std::{collections::HashMap, net::IpAddr};

use base64::{Engine as _, engine::general_purpose::STANDARD as base64};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::de::{Deserializer, Error as _};
use serde_json::Value;
use serde_repr::*;

use super::{AltSpeedDay, Encryption, Id, IdleMode, Priority, RatioMode, Tag};

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T: RpcResponseArgument> {
    pub arguments: T,
    pub result: String,
    /// "An optional `tag` number as described in [`2.1`]." <sup>[[1]]</sup>
    ///
    /// [`2.1`]: <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#21-requests>
    /// [1]: <https://github.com/transmission/transmission/blob/4.0.6/libtransmission/rpcimpl.cc#L2520>
    pub tag: Option<Tag>,
}

impl<T: RpcResponseArgument> RpcResponse<T> {
    pub fn is_ok(&self) -> bool {
        self.result == "success"
    }
}
pub trait RpcResponseArgument {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionGet {
    /// Max global download speed (kB/s)
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub alt_speed_down: Option<i32>,
    /// `true` means use the alt speeds (ie, turtle mode)
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub alt_speed_enabled: Option<bool>,
    /// When to turn on alt speeds (units: minutes after midnight)
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub alt_speed_time_begin: Option<i32>,
    /// What day(s) to turn on alt speeds (ie. turtle mode)
    pub alt_speed_time_day: Option<AltSpeedDay>,
    /// `true` means the scheduled on/off times are used
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub alt_speed_time_enabled: Option<bool>,
    /// when to turn off alt speeds (units: minutes after midnight)
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub alt_speed_time_end: Option<i32>,
    /// Max global upload speed (kB/s)
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub alt_speed_up: Option<i32>,
    /// `true` means to enable a basic brute force protection for RPC server.
    ///
    /// > Added in Transmission ???
    pub anti_brute_force_enabled: Option<bool>,
    /// Basic brute force protection threshold in an unknown unit.
    ///
    /// > Added in Transmission ??? [NOT DOCUMENTED IN RPC-SPEC]
    pub anti_brute_force_threshold: Option<i32>,
    /// `true` means block peers based on the configured blocklist (see: [blocklists.md])
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [blocklists.md]: https://github.com/transmission/transmission/blob/main/docs/Blocklists.md
    pub blocklist_enabled: Option<bool>,
    /// Number of rules in the blocklist
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub blocklist_size: Option<i32>,
    /// location of the blocklist to use for `blocklist-update`
    ///
    /// > Added in Transmission 2.12 (`rpc-version-semver` 3.5.0, `rpc-version`: 11)
    pub blocklist_url: Option<String>,
    /// Maximum size of the disk cache (MiB). Pieces are guaranteed to be written to filesystem if
    /// sequential download is enabled. Otherwise, data might still be in cache only.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// > Renamed `cache_size_mb` to `cache_size_mib` in Transmission 4.1.0 (`rpc-version-semver`
    /// > 6.0.0,, `rpc-version`: 18)
    #[serde(alias = "cache_size_mib")]
    pub cache_size_mb: Option<i32>,
    /// Location of transmission's configuration directory
    ///
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    pub config_dir: Option<String>,
    /// Announce URLs, one per line, and a blank line between [tiers].
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [tiers]: https://www.bittorrent.org/beps/bep_0012.html
    pub default_trackers: Option<String>,
    /// `true` means allow [DHT] in public torrents
    ///
    /// [DHT]: https://wikipedia.org/wiki/Distributed_hash_table
    pub dht_enabled: Option<bool>,
    /// Default path to download torrents
    pub download_dir: Option<String>,
    /// **DEPRECATED** Use the `free-space` method instead.
    ///
    /// > Added in Transmission 2.20 (`rpc-version-semver` 3.6.0, `rpc-version`: 12)
    pub download_dir_free_space: Option<u64>,
    /// If `true`, limit how many torrents can be downloaded at once
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    pub download_queue_enabled: Option<bool>,
    /// Max number of torrents to download at once (see [`download_queue_enabled`])
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`download_queue_enabled`]: Self::download_queue_enabled
    pub download_queue_size: Option<i32>,
    /// The encryption type for peer connections.
    pub encryption: Option<Encryption>,
    /// `true` if the seeding inactivity limit is honored by default
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    pub idle_seeding_limit_enabled: Option<bool>,
    /// torrents we're seeding will be stopped if they're idle for this long (in minutes)
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    pub idle_seeding_limit: Option<i32>,
    /// `true` means keep torrents in [`incomplete_dir`] until done
    ///
    /// > Added in Transmission 1.80 (`rpc-version-semver` 3.0.0, `rpc-version`: 7)
    ///
    /// [`incomplete_dir`]: Self::incomplete_dir
    pub incomplete_dir_enabled: Option<bool>,
    /// Path for incomplete torrents, when enabled
    ///
    /// > Added in Transmission 1.80 (`rpc-version-semver` 3.0.0, `rpc-version`: 7)
    pub incomplete_dir: Option<String>,
    /// `true` means allow Local Peer Discovery in public torrents
    pub lpd_enabled: Option<bool>,
    /// Maximum global number of peers
    ///
    /// > Renamed from `peer-limit` to `peer-limit-global` in Transmission 1.60
    /// (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub peer_limit_global: Option<i32>,
    /// Default maximum number of peers per torrent
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    pub peer_limit_per_torrent: Option<i32>,
    /// `true` means pick a random peer port on launch
    pub peer_port_random_on_start: Option<bool>,
    /// The port that the daemon listens for peer connections on
    ///
    /// > Renamed from `port` to `peer-port` in Transmission 1.60 (`rpc-version-semver` 2.0.0,
    /// `rpc-version`: 5)
    pub peer_port: Option<u16>,
    /// `true` means allow [PEX] in public torrents
    ///
    /// > Renamed from `pex-allowed` to `pex-enabled` in Transmission 1.60 (`rpc-version-semver`
    /// 2.0.0, `rpc-version`: 5)
    ///
    /// [PEX]: https://wikipedia.org/wiki/Peer_exchange
    pub pex_enabled: Option<bool>,
    /// `true` means ask upstream router to forward the configured peer port to transmission using
    /// [UPnP] or [NAT-PMP]
    ///
    /// [UPnP]: https://wikipedia.org/wiki/Universal_Plug_and_Play
    /// [NAT-PMP]: https://wikipedia.org/wiki/NAT_Port_Mapping_Protocol
    pub port_forwarding_enabled: Option<bool>,
    /// List of preferred transport protocols in the order of preferred-first.
    ///
    /// * ["utp"](https://en.wikipedia.org/wiki/Micro_Transport_Protocol)
    /// * ["tcp"](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    #[serde(alias = "preferred_transports")]
    pub preferred_transports: Option<Vec<String>>,
    /// Whether or not to consider idle torrents as stalled
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    pub queue_stalled_enabled: Option<bool>,
    /// Torrents that are idle for `queue_stalled_minutes` aren't counted toward
    /// [`seed_queue_size`] or [`download-queue-size`]
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_size`]: Self::seed_queue_size
    /// [`download-queue-size`]: Self::download-queue-size
    pub queue_stalled_minutes: Option<i32>,
    /// `true` means append `.part` to incomplete files
    /// 
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    pub rename_partial_files: Option<bool>,
    /// The number of outstanding block requests a peer is allowed to queue in the client
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 5.4.0, `rpc-version`: 18)
    pub reqq: Option<i32>,
    /// The minimum RPC API version supported by the RPC server. It changes when a new version of
    /// Transmission changes the RPC interface in a way that is not backwards compatible.
    pub rpc_version_minimum: Option<i32>,
    /// The current RPC API version in a [semver]-compatible string
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [semver]: https://semver.org/
    pub rpc_version_semver: Option<String>,
    /// the current RPC API version
    pub rpc_version: Option<i32>,
    /// Whether or not to call the [added script] (see: [scripts.md])
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [added script]: Self::script_torrent_added_filename
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    pub script_torrent_added_enabled: Option<bool>,
    /// Path of the script to run on torrent added (see: [scripts.md])
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    pub script_torrent_added_filename: Option<String>,
    /// Whether or not to call the [done script] (see: [scripts.md])
    ///
    /// [done script]: Self::script_torrent_done_filename
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    pub script_torrent_done_enabled: Option<bool>,
    /// Path of the script to run on torrent completion (see: [scripts.md])
    ///
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    pub script_torrent_done_filename: Option<String>,
    /// Whether or not to call the [seeding-done] script (see: [scripts.md])
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [seeding-done]: Self::script_torrent_done_seeding_filename
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    pub script_torrent_done_seeding_enabled: Option<bool>,
    /// Path of the script to run on torrent seeding completion (see: [scripts.md])
    /// 
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    pub script_torrent_done_seeding_filename: Option<String>,
    /// if `true`, limit how many torrents can be uploaded at once
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    pub seed_queue_enabled: Option<bool>,
    /// Max number of torrents to uploaded at once (see [seed_queue_enabled])
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [seed_queue_enabled]: Self::seed_queue_enabled
    pub seed_queue_size: Option<i32>,
    /// The default seed ratio for torrents to use
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f64>,
    /// `true` if [`seed_ratio_limit`] is honored by default
    ///
    /// [`seed_ratio_limit`]: Self::seed_ratio_limit
    #[serde(alias = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,
    /// `true` means sequential download is enabled by default for added torrents
    /// 
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 5.4.0, `rpc-version`: 18)
    #[serde(alias = "sequential_download")]
    pub sequential_download: Option<bool>,
    /// The current [`X-Transmission-Session-Id`] value
    ///
    /// > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16)
    ///
    /// [`X-Transmission-Session-Id`]: https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#231-csrf-protection
    pub session_id: Option<String>,
    /// `true` means limit global download speed
    pub speed_limit_down_enabled: Option<bool>,
    /// Max global download speed (kB/s)
    pub speed_limit_down: Option<i32>,
    /// `true` means limit global upload speed
    pub speed_limit_up_enabled: Option<bool>,
    /// Max global upload speed (kB/s)
    pub speed_limit_up: Option<i32>,
    /// `true` means added torrents will be started right away
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    pub start_added_torrents: Option<bool>,
    /// `true` means allow [TCP]
    ///
    /// [TCP]: https://en.wikipedia.org/wiki/Transmission_Control_Protocol
    pub tcp_enabled: Option<bool>,
    /// `true` means the `.torrent` file of added torrents will be deleted
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    pub trash_original_torrent_files: Option<bool>,
    /// The units used by the daemon (I think?)
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    pub units: Option<SessionGetUnits>,
    /// `true` means allow [UTP]
    ///
    /// [UTP]: https://wikipedia.org/wiki/Micro_Transport_Protocol
    pub utp_enabled: Option<bool>,
    /// Long version string, ie. `$version ($revision)`
    ///
    /// > Added in Transmission 1.41 (`rpc-version-semver` 1.2.0, `rpc-version`: 3)
    pub version: Option<String>,
}
impl RpcResponseArgument for SessionGet {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionGetUnits {
    /// 4 strings: KB/s, MB/s, GB/s, TB/s
    ///
    /// v4.1.1: 5 strings: B/s, KB/s, MB/s, GB/s, TB/s
    pub speed_units: Vec<String>,
    /// number of bytes in a KB (1000 for kB; 1024 for KiB)
    pub speed_bytes: usize,
    /// 4 strings: KB, MB, GB, TB
    ///
    /// v4.1.1: 5 strings: B, KB, MB, GB, TB
    pub size_units: Vec<String>,
    /// number of bytes in a KB (1000 for kB; 1024 for KiB)
    pub size_bytes: usize,
    /// 4 strings: KiB, MiB, GiB, TiB
    ///
    /// v4.1.1: 5 strings: B, KiB, MiB, GiB, TiB
    pub memory_units: Vec<String>,
    /// number of bytes in a KB (1000 for kB; 1024 for KiB)
    pub memory_bytes: usize,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub torrent_count: i32,
    pub active_torrent_count: i32,
    pub paused_torrent_count: i32,
    pub download_speed: i64,
    pub upload_speed: i64,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
}
impl RpcResponseArgument for SessionStats {}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct BlocklistUpdate {
    pub blocklist_size: Option<i32>,
}
impl RpcResponseArgument for BlocklistUpdate {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct FreeSpace {
    pub path: String,
    pub size_bytes: i64,
}
impl RpcResponseArgument for FreeSpace {}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct PortTest {
    pub port_is_open: bool,
}
impl RpcResponseArgument for PortTest {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TorrentStatus {
    Stopped = 0,
    QueuedToVerify = 1,
    Verifying = 2,
    QueuedToDownload = 3,
    Downloading = 4,
    QueuedToSeed = 5,
    Seeding = 6,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ErrorType {
    Ok = 0,
    TrackerWarning = 1,
    TrackerError = 2,
    LocalError = 3,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    #[serde(deserialize_with = "from_ts_option", default)]
    pub activity_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub added_date: Option<DateTime<Utc>>,
    /// "An array of `pieceCount` numbers representing the number of connected peers that have each
    /// piece, or -1 if we already have the piece ourselves."
    ///
    /// Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17).
    pub availability: Option<Vec<i16>>,
    pub bandwidth_priority: Option<Priority>,
    pub comment: Option<String>,
    pub corrupt_ever: Option<u64>,
    pub creator: Option<String>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub date_created: Option<DateTime<Utc>>,
    pub desired_available: Option<u64>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub done_date: Option<DateTime<Utc>>,
    pub download_dir: Option<String>,
    pub downloaded_ever: Option<u64>,
    pub download_limit: Option<u64>,
    pub download_limited: Option<bool>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub edit_date: Option<DateTime<Utc>>,
    pub error: Option<ErrorType>,
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub eta_idle: Option<i64>,
    pub group: Option<String>,
    pub hash_string: Option<String>,
    pub have_unchecked: Option<u64>,
    pub have_valid: Option<u64>,
    pub honors_session_limits: Option<bool>,
    pub id: Option<i64>,
    pub is_finished: Option<bool>,
    pub is_private: Option<bool>,
    pub is_stalled: Option<bool>,
    pub labels: Option<Vec<String>>,
    pub left_until_done: Option<i64>,
    pub magnet_link: Option<String>,
    /// [`DateTime::UNIX_EPOCH`] if never manually announced.
    #[serde(deserialize_with = "from_ts_option", default)]
    pub manual_announce_time: Option<DateTime<Utc>>,
    pub max_connected_peers: Option<u16>,
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<u16>,
    pub peers: Option<Vec<Peer>>,
    pub peers_connected: Option<i64>,
    pub peers_from: Option<PeersFrom>,
    pub peers_getting_from_us: Option<i64>,
    pub peers_sending_to_us: Option<i64>,
    pub percent_complete: Option<f32>,
    pub percent_done: Option<f32>,
    /// "A bitfield holding `pieceCount` flags which are set to 'true' if we have the piece
    /// matching that position. JSON doesn't allow raw binary data, so this is a base64-encoded
    /// string."
    #[serde(deserialize_with = "from_bitfield_option", default)]
    pub pieces: Option<Vec<u8>>,
    pub piece_count: Option<u64>,
    pub piece_size: Option<u64>,
    #[serde(rename = "primary-mime-type")]
    pub primary_mime_type: Option<String>,
    pub queue_position: Option<usize>,
    pub rate_download: Option<i64>,
    pub rate_upload: Option<i64>,
    pub recheck_progress: Option<f32>,
    pub seconds_downloading: Option<u64>,
    pub seconds_seeding: Option<i64>,
    pub seed_idle_limit: Option<u64>, // Can this be negative?
    pub seed_idle_mode: Option<IdleMode>,
    pub seed_ratio_limit: Option<f64>,
    pub seed_ratio_mode: Option<RatioMode>,
    pub sequential_download: Option<bool>,
    pub size_when_done: Option<i64>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub start_date: Option<DateTime<Utc>>,
    pub status: Option<TorrentStatus>,
    pub torrent_file: Option<String>,
    pub total_size: Option<i64>,
    pub trackers: Option<Vec<Trackers>>,
    pub tracker_list: Option<String>,
    pub tracker_stats: Option<Vec<TrackerStat>>,
    pub upload_ratio: Option<f32>,
    pub uploaded_ever: Option<i64>,
    pub upload_limit: Option<u64>, // Can this be negative?
    pub upload_limited: Option<bool>,
    pub files: Option<Vec<File>>,
    /// Each element represents whether the corresponding file in [`files`] will be downloaded
    /// (`true`) or not (`false`).
    ///
    /// [`files`]: Torrent::files
    #[serde(deserialize_with = "from_arr_bool_option", default)]
    pub wanted: Option<Vec<bool>>,
    pub webseeds: Option<Vec<String>>,
    pub webseeds_sending_to_us: Option<u16>,
    pub priorities: Option<Vec<Priority>>,
    pub file_stats: Option<Vec<FileStat>>,
    #[serde(rename = "file-count")]
    pub file_count: Option<usize>,
}

fn from_ts_option<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: i64 = Deserialize::deserialize(deserializer)?;
    // The transmission rpc server responds with 0 or -1 (in the case of manualAnnounceTime) when
    // the date is unset or invalid.
    // Consolidate any response <= 0 as UNIX_EPOCH to denote these cases.
    if ts <= 0 {
        return Ok(Some(DateTime::UNIX_EPOCH));
    }
    Ok(DateTime::<Utc>::from_timestamp(ts, 0))
}

/// Attempts to deserialize a [`base64`]-encoded string into a `Vec<u8>`.
fn from_bitfield_option<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bitfield = base64.decode(encoded).map_err(D::Error::custom)?;
    Ok(Some(bitfield))
}

/// Attempts to deserialize an array of bools or ints (`0` or `1`) into a `Vec<bool>`, treating `0`
/// as false and `1` as true.
fn from_arr_bool_option<'de, D>(deserializer: D) -> Result<Option<Vec<bool>>, D::Error>
where
    D: Deserializer<'de>,
{
    let unexpected = || D::Error::custom("unexpected type");

    let wanted = match Deserialize::deserialize(deserializer)? {
        Value::Array(arr) => arr
            .into_iter()
            .map(|val| match val {
                // transmission 5.0.0+ returns an array of booleans.
                Value::Bool(b) => Ok(b),
                // transmission 4.x.x and below returns an array of ints (1 true, 0 false).
                Value::Number(num) => num.as_i64().map(|n| n == 1).ok_or_else(unexpected),
                // rpc server misbehaving (got an unexpected type).
                _ => Err(unexpected()),
            })
            .collect::<Result<Vec<_>, _>>()?,
        // `wanted` should be an array.
        _ => Err(unexpected())?,
    };
    Ok(Some(wanted))
}

impl Torrent {
    /// Get either the ID or the hash string if exist, which are both unique and
    /// can be pass to the API.
    pub fn id(&self) -> Option<Id> {
        self.id
            .map(Id::Id)
            .or_else(|| self.hash_string.clone().map(Id::Hash))
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub files_added: i32,
    pub downloaded_bytes: i64,
    pub uploaded_bytes: i64,
    pub seconds_active: i64,
    pub session_count: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Torrents<T> {
    pub torrents: Vec<T>,
}
impl RpcResponseArgument for Torrents<Torrent> {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Trackers {
    pub id: i32,
    pub announce: String,
    pub scrape: String,
    /// `the first label before the public suffix in the announce URL's host. eg.
    /// "https://www.example.co.uk/announce"'s sitename is "example"`
    /// Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17)
    #[serde(default)]
    pub sitename: String,
    pub tier: usize,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// "the total size of the file"
    pub length: i64,
    /// "the current size of the file, i.e. how much we've downloaded"
    pub bytes_completed: i64,
    /// "This file's name. Includes the full subpath in the torrent."
    pub name: String,
    /// "piece index where this file starts"
    ///
    /// Should be `Some(_)` if the Transmission version >= `4.1.0`, `None` if the version is less
    /// than `4.1.0`.
    ///
    /// Added in Transmission `4.1.0` (`rpc-version-semver` 5.4.0, `rpc-version`: 18).
    pub begin_piece: Option<u64>,
    /// "piece index where this file ends (exclusive)"
    ///
    /// See [`begin_piece`](File::begin_piece).
    ///
    /// Added in Transmission `4.1.0` (`rpc-version-semver` 5.4.0, `rpc-version`: 18).
    pub end_piece: Option<u64>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileStat {
    pub bytes_completed: i64,
    pub wanted: bool,
    pub priority: Priority,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    // FIXME? serde doesn't like simplified ipv6 addresses
    // FIXME? (does transmission emit simplified ipv6? eg. "::1")
    pub address: IpAddr,
    pub client_name: String,
    pub client_is_choked: bool,
    pub client_is_interested: bool,
    pub flag_str: String,
    pub is_downloading_from: bool,
    pub is_encrypted: bool,
    pub is_incoming: bool,
    pub is_uploading_to: bool,
    #[serde(rename = "isUTP")]
    pub is_utp: bool,
    pub peer_is_choked: bool,
    pub peer_is_interested: bool,
    pub port: u16,
    pub progress: f32,
    pub rate_to_client: u64, // (B/s)
    pub rate_to_peer: u64,   // (B/s)
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PeersFrom {
    pub from_cache: u16,
    pub from_dht: u16,
    pub from_incoming: u16,
    pub from_lpd: u16,
    pub from_ltep: u16,
    pub from_pex: u16,
    pub from_tracker: u16,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackerStat {
    pub announce_state: TrackerState,
    pub announce: String,
    pub download_count: i64,
    pub has_announced: bool,
    pub has_scraped: bool,
    pub host: String,
    pub id: Id,
    pub is_backup: bool,
    pub last_announce_peer_count: i64,
    pub last_announce_result: String,
    #[serde(deserialize_with = "from_ts")]
    pub last_announce_start_time: DateTime<Utc>,
    pub last_announce_succeeded: bool,
    #[serde(deserialize_with = "from_ts")]
    pub last_announce_time: DateTime<Utc>,
    pub last_announce_timed_out: bool,
    pub last_scrape_result: String,
    #[serde(deserialize_with = "from_ts")]
    pub last_scrape_start_time: DateTime<Utc>,
    pub last_scrape_succeeded: bool,
    #[serde(deserialize_with = "from_ts")]
    pub last_scrape_time: DateTime<Utc>,
    pub last_scrape_timed_out: bool,
    pub leecher_count: i64,
    #[serde(deserialize_with = "from_ts")]
    pub next_announce_time: DateTime<Utc>,
    #[serde(deserialize_with = "from_ts")]
    pub next_scrape_time: DateTime<Utc>,
    pub scrape_state: TrackerState,
    pub scrape: String,
    pub seeder_count: i64,
    /// `the first label before the public suffix in the announce URL's host. eg.
    /// "https://www.example.co.uk/announce"'s sitename is "example"`
    /// Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17)
    #[serde(default)]
    pub sitename: String,
    pub tier: usize,
}

#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum TrackerState {
    Inactive = 0,
    Waiting = 1,
    Queued = 2,
    Active = 3,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Debug, Clone)]
pub enum TorrentAddedOrDuplicate {
    TorrentDuplicate(Torrent),
    TorrentAdded(Torrent),
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TorrentRenamePath {
    pub path: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}
impl RpcResponseArgument for TorrentRenamePath {}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct GroupGet {
    #[serde(rename = "honorsSessionLimits")]
    pub honors_session_limits: bool,
    pub name: String,
    pub speed_limit_down_enabled: bool,
    pub speed_limit_down: u64,
    pub speed_limit_up_enabled: bool,
    pub speed_limit_up: u64,
}
impl RpcResponseArgument for Vec<GroupGet> {}

#[cfg(test)]
mod tests {
    use crate::types::{Result, RpcResponse, TorrentAddedOrDuplicate};
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
}
