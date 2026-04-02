use std::{
    net::IpAddr,
    result::Result as StdResult,
};

use base64::{Engine as _, engine::general_purpose::STANDARD as base64};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, de::Error as _};
use serde_json::Value;
use serde_repr::Deserialize_repr;
use url::Url;

use crate::types::{Id, IdleMode, Priority, RatioMode, TrackerId, TrackerList};

/// Represents a torrent from a [`torrent_get`] request.
///
/// [`torrent_get`]: crate::TransClient::torrent_get
#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    /// The last time we uploaded or downloaded piece data on this torrent.
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "activity_date")]
    pub activity_date: Option<DateTime<Utc>>,
    /// When the torrent was first added.
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "added_date")]
    pub added_date: Option<DateTime<Utc>>,
    /// An array of [`piece_count`] numbers representing the number of connected peers that have
    /// each piece, or -1 if we already have the piece ourselves.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17).
    ///
    /// [`piece_count`]: Self::piece_count
    pub availability: Option<Vec<i16>>,
    /// The torrent's bandwidth priority.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "bandwidth_priority")]
    pub bandwidth_priority: Option<Priority>,
    /// An array of `tr_info.filecount` numbers. Each is the completed bytes for the corresponding
    /// file.
    ///
    /// > (?) Added in Transmission `4.1.0` (`rpc-version-semver` 6.0.0, `rpc-version`: 18) [in
    /// [transmission:d0996479d]].
    ///
    /// [transmission:d0996479d]:
    /// <https://github.com/transmission/transmission/commit/d0996479de7cbeb6a3522bac20fe92a03a93cc6f>
    #[serde(alias = "bytes_completed")]
    pub bytes_completed: Option<Vec<u64>>,
    /// An arbitrary comment string (may be empty).
    pub comment: Option<String>,
    /// Byte count of all the corrupt data you've ever downloaded for this torrent. If you're on a
    /// poisoned torrent, this number can grow very large.
    #[serde(alias = "corrupt_ever")]
    pub corrupt_ever: Option<u64>,
    /// An arbitrary creation program string (may be empty).
    pub creator: Option<String>,
    /// When the torrent was created (may be [`DateTime::UNIX_EPOCH`]).
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "date_created")]
    pub date_created: Option<DateTime<Utc>>,
    /// Byte count of all the piece data we want and don't have yet, but that a connected peer does
    /// have; range: [0, [`left_until_done`]].
    ///
    /// [`left_until_done`]: Self::left_until_done
    #[serde(alias = "desired_available")]
    pub desired_available: Option<u64>,
    /// When the torrent finished downloading.
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "done_date")]
    pub done_date: Option<DateTime<Utc>>,
    /// Where the files are when the torrent is complete.
    ///
    /// > Added in Transmission 1.50 (`rpc-version-semver` 1.3.0, `rpc-version`: 4)
    #[serde(alias = "download_dir")]
    pub download_dir: Option<String>,
    /// Byte count of all the non-corrupt data you've ever downloaded for this torrent. If you
    /// deleted the files and downloaded a second time, this will be: 2*[`total_size`].
    ///
    /// [`total_size`]: Self::total_size
    #[serde(alias = "downloaded_ever")]
    pub downloaded_ever: Option<u64>,
    /// Maximum download speed (kB/s).
    #[serde(alias = "download_limit")]
    pub download_limit: Option<u64>,
    /// True if [`download_limit`] is honored.
    ///
    /// [`download_limit`]: Self::download_limit
    #[serde(alias = "download_limited")]
    pub download_limited: Option<bool>,
    /// The last time during this session that a rarely-changing field changed -- e.g. any
    /// `tr_torrent_metainfo` field (trackers, filenames, name) or download directory. RPC clients
    /// can monitor this to know when to reload fields that rarely change.
    ///
    /// > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16)
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "edit_date")]
    pub edit_date: Option<DateTime<Utc>>,
    /// Defines what kind of text is in [`error_string`].
    ///
    /// [`error_string`]: Self::error_string
    pub error: Option<ErrorType>,
    /// A warning or error message regarding the torrent. See also: [`error`].
    ///
    /// [`error`]: Self::error
    #[serde(alias = "error_string")]
    pub error_string: Option<String>,
    /// If downloading, estimated number of seconds left until the torrent is done.
    /// If seeding, estimated number of seconds left until seed ratio is reached.
    pub eta: Option<i64>,
    /// If seeding, number of seconds left until the [idle time limit] is reached.
    ///
    /// > Added in Transmission 2.80 (`rpc-version-semver` 5.1.0, `rpc-version`: 15)
    ///
    /// [idle time limit]: Self::seed_idle_limit
    #[serde(alias = "eta_idle")]
    pub eta_idle: Option<i64>,
    /// Number of files in the torrent.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    #[serde(alias = "file-count")]
    #[serde(alias = "file_count")]
    pub file_count: Option<usize>,
    /// The torrent's file data.
    pub files: Option<Vec<File>>,
    /// The torrent's files non-constant properties, in the same order as [`files`].
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [`files`]: Self::files
    #[serde(alias = "file_stats")]
    pub file_stats: Option<Vec<FileStat>>,
    /// The name of this torrent's bandwidth group.
    ///
    ///  > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    pub group: Option<String>,
    /// The torrent's [SHA1] hash string identifier.
    ///
    /// [SHA1]: <https://en.wikipedia.org/wiki/SHA-1>
    #[serde(alias = "hash_string")]
    pub hash_string: Option<String>,
    /// Byte count of all the partial piece data we have for this torrent. As [`pieces`] become
    /// complete, this value may decrease as portions of it are moved to [`corrupt_ever`] or
    /// [`have_valid`].
    ///
    /// [`pieces`]: Self::pieces
    /// [`corrupt_ever`]: Self::corrupt_ever
    /// [`have_valid`]: Self::have_valid
    #[serde(alias = "have_unchecked")]
    pub have_unchecked: Option<u64>,
    /// Byte count of all the checksum-verified data we have for this torrent.
    #[serde(alias = "have_valid")]
    pub have_valid: Option<u64>,
    /// True if session upload limits are honored.
    #[serde(alias = "honors_session_limits")]
    pub honors_session_limits: Option<bool>,
    /// The torrent's unique numeric identifier. Note that integer torrent ids are not stable
    /// across Transmission daemon restarts. Use [torrent hashes] if you need stable ids.
    ///
    /// [torrent hashes]: Self::hash_string
    pub id: Option<i64>,
    /// A torrent is considered finished if it has met its seed ratio.
    /// As a result, only paused torrents can be finished.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    #[serde(alias = "is_finished")]
    pub is_finished: Option<bool>,
    /// Whether the torrent is private (`true`) or not (`false`).
    #[serde(alias = "is_private")]
    pub is_private: Option<bool>,
    /// True if the torrent is running, but has been idle for long enough to be considered stalled.
    /// See: [`SessionGet::queue_stalled_minutes`].
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`SessionGet::queue_stalled_minutes`]: super::SessionGet::queue_stalled_minutes
    #[serde(alias = "is_stalled")]
    pub is_stalled: Option<bool>,
    /// An array of the torrent's user-created (arbitrary) labels.
    ///
    /// > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16)
    pub labels: Option<Vec<String>>,
    /// Byte count of how much data is left to be downloaded until we've got all the pieces that we
    /// want; range: [0, [`size_when_done`]].
    ///
    /// [`size_when_done`]: Self::size_when_done
    #[serde(alias = "left_until_done")]
    pub left_until_done: Option<u64>,
    /// The torrent's [magnet link].
    ///
    /// > Added in Transmission 1.80 (`rpc-version-semver` 3.0.0, `rpc-version`: 7)
    ///
    /// [magnet link]: <https://en.wikipedia.org/wiki/Magnet_URI_scheme>
    #[serde(alias = "magnet_link")]
    pub magnet_link: Option<String>,
    /// Time when one or more of the torrent's trackers will allow you to manually ask for more
    /// peers, or [`DateTime::UNIX_EPOCH`] if you can't.
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18): "it
    /// never worked".
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "manual_announce_time")]
    pub manual_announce_time: Option<DateTime<Utc>>,
    /// The maximum number of [`peers`] allowed to connect for this torrent. (This seems to be an
    /// alias for [`peer_limit`]).
    ///
    /// [`peers`]: Self::peers
    /// [`peer_limit`]: Self::peer_limit
    #[serde(alias = "max_connected_peers")]
    pub max_connected_peers: Option<u16>,
    /// How much of the metadata the torrent has. For torrents added from a torrent this will
    /// always be `1.0`. For [magnet links], this number will range from 0 to 1 as the metadata is
    /// downloaded. Range is [0, 1].
    ///
    /// > Added in Transmission 1.80 (`rpc-version-semver` 3.0.0, `rpc-version`: 7)
    ///
    /// [magnet links]: <https://en.wikipedia.org/wiki/Magnet_URI_scheme>
    #[serde(alias = "metadata_percent_complete")]
    pub metadata_percent_complete: Option<f32>,
    /// The torrent's name.
    pub name: Option<String>,
    /// The maximum number of [`peers`] allowed to connect for this torrent. (This seems to be an
    /// alias for [`max_connected_peers`]).
    ///
    /// [`peers`]: Self::peers
    /// [`max_connected_peers`]: Self::max_connected_peers
    #[serde(alias = "peer-limit")]
    #[serde(alias = "peer_limit")]
    pub peer_limit: Option<u16>,
    /// Known peers data.
    pub peers: Option<Vec<Peer>>,
    /// Number of [`peers`] that we're connected to.
    ///
    /// [`peers`]: Self::peers
    #[serde(alias = "peers_connected")]
    pub peers_connected: Option<u16>,
    /// How many connected [`peers`] we found out about from various sources.
    ///
    /// [`peers`]: Self::peers
    #[serde(alias = "peers_from")]
    pub peers_from: Option<PeersFrom>,
    /// Number of [`peers`] that we're sending data to.
    ///
    /// [`peers`]: Self::peers
    #[serde(alias = "peers_getting_from_us")]
    pub peers_getting_from_us: Option<u16>,
    /// Number of [`peers`] that are sending data to us.
    ///
    /// [`peers`]: Self::peers
    #[serde(alias = "peers_sending_to_us")]
    pub peers_sending_to_us: Option<u16>,
    /// How much has been downloaded of the entire torrent. Range is [0, 1].
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    #[serde(alias = "percent_complete")]
    pub percent_complete: Option<f32>,
    /// How much has been downloaded of the files the user wants. This differs from
    /// [`percent_complete`] if the user wants only some of the torrent's files. Range is [0, 1].
    ///
    /// See: [`left_until_done`].
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [`percent_complete`]: Self::percent_complete
    /// [`left_until_done`]: Self::left_until_done
    #[serde(alias = "percent_done")]
    pub percent_done: Option<f32>,
    /// A bitfield holding [`piece_count`] flags which are set to `true` if we have the piece
    /// matching that position. JSON doesn't allow raw binary data, so this is a [base64]-encoded
    /// string.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [`piece_count`]: Self::piece_count
    /// [base64]: <https://en.wikipedia.org/wiki/Base64>
    #[serde(deserialize_with = "from_bitfield_option", default)]
    pub pieces: Option<Vec<u8>>,
    /// Total number of [`pieces`] in the torrent.
    ///
    /// [`pieces`]: Self::pieces
    #[serde(alias = "piece_count")]
    pub piece_count: Option<u64>,
    /// The torrent's piece size (aka. piece length).
    #[serde(alias = "piece_size")]
    pub piece_size: Option<u64>,
    /// An array of file download priorities, in the same order as [`files`].
    ///
    /// [`files`]: Self::files
    pub priorities: Option<Vec<Priority>>,
    /// The [mime-type] (e.g. `audio/x-flac`) that matches more of the torrent's content than any
    /// other [mime-type].
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, :rpc-version: 17)
    ///
    /// [mime-type]: <https://en.wikipedia.org/wiki/Media_type>
    #[serde(alias = "primary-mime-type")]
    #[serde(alias = "primary_mime_type")]
    pub primary_mime_type: Option<String>,
    /// This torrent's queue position. All torrents have a queue position, even if it's not queued.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    #[serde(alias = "queue_position")]
    pub queue_position: Option<usize>,
    /// The rate we are receiving data for this torrent in `B/s`.
    #[serde(alias = "rate_download")]
    pub rate_download: Option<u64>,
    /// The rate we are sending data for this torrent in `B/s`.
    #[serde(alias = "rate_upload")]
    pub rate_upload: Option<u64>,
    /// When [`status`] is [`TorrentStatus::Verifying`] or [`TorrentStatus::QueuedToVerify`], this
    /// is the percentage of how much of the files has been verified. When it gets to `1.0`, the
    /// verify process is done. Range is [0, 1].
    ///
    /// [`status`]: Self::status
    #[serde(alias = "recheck_progress")]
    pub recheck_progress: Option<f32>,
    /// Cumulative seconds the torrent's ever spent downloading.
    #[serde(alias = "seconds_downloading")]
    pub seconds_downloading: Option<u64>,
    /// Cumulative seconds the torrent's ever spent seeding.
    #[serde(alias = "seconds_seeding")]
    pub seconds_seeding: Option<u64>,
    /// Number of minutes seeding inactivity until the torrent is paused based on its
    /// [`seed_idle_mode`].
    ///
    /// See: [`is_stalled`].
    ///
    /// [`seed_idle_mode`]: Self::seed_idle_mode
    /// [`is_stalled`]: Self::is_stalled
    #[serde(alias = "seed_idle_limit")]
    pub seed_idle_limit: Option<u64>,
    /// How [`seed_idle_limit`] is treated for this torrent.
    ///
    /// [`seed_idle_limit`]: Self::seed_idle_limit
    #[serde(alias = "seed_idle_mode")]
    pub seed_idle_mode: Option<IdleMode>,
    /// The ratio to seed the torrent until pausing it based on its [`seed_ratio_mode`].
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [`seed_ratio_mode`]: Self::seed_ratio_mode
    #[serde(alias = "seed_ratio_limit")]
    pub seed_ratio_limit: Option<f64>,
    /// How [`seed_ratio_limit`] is treated for this torrent.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [`seed_ratio_limit`]: Self::seed_ratio_limit
    #[serde(alias = "seed_ratio_mode")]
    pub seed_ratio_mode: Option<RatioMode>,
    /// Whether the torrent's [`pieces`] are downloaded sequentially.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [`pieces`]: Self::pieces
    #[serde(alias = "sequential_download")]
    pub sequential_download: Option<bool>,
    /// Which piece the torrent will start sequentially downloading from when
    /// [`sequential_download`] is enabled.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [`sequential_download`]: Self::sequential_download
    #[serde(rename = "sequential_download_from_piece")]
    pub sequential_download_from_piece: Option<u64>,
    /// Byte count of all the piece data we'll have downloaded when we're done, whether or not we
    /// have it yet. If we only want some of the files, this may be less than [`total_size`].
    /// Range: [0, [`total_size`]].
    ///
    /// [`total_size`]: Self::total_size
    #[serde(alias = "size_when_done")]
    pub size_when_done: Option<u64>,
    /// When the torrent was last started. Will be [`DateTime::UNIX_EPOCH`] if the torrent has
    /// never been started.
    #[serde(deserialize_with = "from_ts_option", default)]
    #[serde(alias = "start_date")]
    pub start_date: Option<DateTime<Utc>>,
    /// What the torrent is doing right now.
    pub status: Option<TorrentStatus>,
    /// Path to torrent's backing torrent file.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "torrent_file")]
    pub torrent_file: Option<String>,
    /// Total size of the torrent, in bytes.
    #[serde(alias = "total_size")]
    pub total_size: Option<u64>,
    /// Array of the torrent's tracker data. This information is a subset of [`tracker_stats`].
    ///
    /// [`tracker_stats`]: Self::tracker_stats
    pub trackers: Option<Vec<Tracker>>,
    /// String of announce URLs, one per line, and a blank line between tiers.
    ///
    /// eg. `"http://bt1.archive.org:6969/announce\n\nhttp://bt2.archive.org:6969/announce\n"`
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    #[serde(alias = "tracker_list")]
    pub tracker_list: Option<TrackerList>,
    /// Array of the torrent's tracker data. This information is a superset of [`trackers`].
    ///
    /// [`trackers`]: Self::trackers
    #[serde(alias = "tracker_stats")]
    pub tracker_stats: Option<Vec<TrackerStat>>,
    /// Byte count of all data you've ever uploaded for this torrent.
    #[serde(alias = "uploaded_ever")]
    pub uploaded_ever: Option<u64>,
    #[serde(alias = "upload_limit")]
    pub upload_limit: Option<u64>, // Can this be negative?
    #[serde(alias = "upload_limited")]
    pub upload_limited: Option<bool>,
    /// Total [uploaded bytes] / [`size_when_done`].
    ///
    /// NB: In Transmission 3.00 and earlier, this was `total upload / download`,
    /// which caused edge cases when total download was less than [`size_when_done`].
    ///
    /// [uploaded bytes]: Self::uploaded_ever
    /// [`size_when_done`]: Self::size_when_done
    #[serde(alias = "upload_ratio")]
    pub upload_ratio: Option<f32>,
    /// Each element represents whether the corresponding file in [`files`] will be downloaded
    /// (`true`) or not (`false`).
    ///
    /// [`files`]: Self::files
    #[serde(deserialize_with = "from_arr_bool_option", default)]
    pub wanted: Option<Vec<bool>>,
    /// A list of [webseed] urls.
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?):
    /// Use [`webseeds_ex`] instead.
    ///
    /// [webseed]: <https://www.bittorrent.org/beps/bep_0019.html>
    /// [`webseeds_ex`]: Self::webseeds_ex
    pub webseeds: Option<Vec<Url>>,
    /// A list of [webseed] data.
    ///
    /// > Added in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?)
    ///
    /// [webseed]: <https://www.bittorrent.org/beps/bep_0019.html>
    #[serde(rename = "webseed_ex")]
    pub webseeds_ex: Option<Vec<WebseedsEx>>,
    /// Number of [webseeds] that are sending data to us.
    ///
    /// [webseeds]: <https://www.bittorrent.org/beps/bep_0019.html>
    #[serde(alias = "webseeds_sending_to_us")]
    pub webseeds_sending_to_us: Option<u16>,
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

/// Represents what kind of [`error_string`] a [`Torrent`] contains.
///
/// [`error_string`]: Torrent::error_string
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ErrorType {
    Ok = 0,
    TrackerWarning = 1,
    TrackerError = 2,
    LocalError = 3,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// The total size of the file.
    pub length: u64,
    /// The current size of the file, i.e. how much we've downloaded.
    #[serde(alias = "bytes_completed")]
    pub bytes_completed: u64,
    /// This file's name. Includes the full subpath in the torrent.
    pub name: String,
    /// Piece index where this file starts.
    ///
    /// Should be `Some(_)` if the Transmission version >= `4.1.0`, `None` if the version is less
    /// than `4.1.0`.
    ///
    /// > Added in Transmission `4.1.0` (`rpc-version-semver` 6.0.0, `rpc-version`: 18).
    #[serde(alias = "begin_piece")]
    pub begin_piece: Option<u64>,
    /// Piece index where this file ends (exclusive).
    ///
    /// See [`begin_piece`](Self::begin_piece).
    ///
    /// > Added in Transmission `4.1.0` (`rpc-version-semver` 6.0.0, `rpc-version`: 18).
    #[serde(alias = "end_piece")]
    pub end_piece: Option<u64>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileStat {
    /// The current size of the file, i.e. how much we've downloaded.
    #[serde(alias = "bytes_completed")]
    pub bytes_completed: u64,
    /// The file's priority.
    pub priority: Priority,
    /// Do we want to download this file?
    pub wanted: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    /// The peer's ip address.
    // FIXME? serde doesn't like simplified ipv6 addresses
    // FIXME? (does transmission emit simplified ipv6? eg. "::1")
    pub address: IpAddr,
    /// The number of bytes the peer has sent to us.
    ///
    /// > (?) Added in Transmission 4.1.0 (` rpc_version_semver ` 6.0.0, `rpc_version`: 18) [in
    /// [transmission:a2d2097b9]]
    ///
    /// [transmission:a2d2097b9]:
    /// <https://github.com/transmission/transmission/commit/a2d2097b9f381ed887107207ec62b72c5be2f902>
    #[serde(default, alias = "bytes_to_client")]
    pub bytes_to_client: u64,
    /// The number of bytes we have sent to the peer.
    ///
    /// > (?) Added in Transmission 4.1.0 (` rpc_version_semver ` 6.0.0, `rpc_version`: 18) [in
    /// [transmission:a2d2097b9]]
    ///
    /// [transmission:a2d2097b9]:
    /// <https://github.com/transmission/transmission/commit/a2d2097b9f381ed887107207ec62b72c5be2f902>
    #[serde(default, alias = "bytes_to_peer")]
    pub bytes_to_peer: u64,
    /// Whether or not the peer is choking<sup>[[1]]</sup> us.
    ///
    /// [1]: <https://www.bittorrent.org/beps/bep_0003.html#peer-protocol>
    #[serde(alias = "client_is_choked")]
    pub client_is_choked: bool,
    /// Whether or not we've indicated to the peer that we would download from them if
    /// unchoked<sup>[[1]]</sup>.
    ///
    /// [1]: <https://www.bittorrent.org/beps/bep_0003.html#peer-protocol>
    #[serde(alias = "client_is_interested")]
    pub client_is_interested: bool,
    /// The user agent, e.g. `BitTorrent 7.9.1`. Will be an empty string if the agent cannot be
    /// determined.
    #[serde(alias = "client_name")]
    pub client_name: String,
    /// A string encoding the peer's state.
    ///
    /// | State                                 | Flag |
    /// |---------------------------------------|------|
    /// | peer using utp                        | 'T'  |
    /// | peer is optimistic<sup>[[1]]</sup>    | 'O'  |
    /// | we are downloading from peer          | 'D'  |
    /// | we are interested but choked          | 'd'  |
    /// | we are uploading to peer              | 'U'  |
    /// | peer is interested but choked         | 'u'  |
    /// | we are neither choked nor interested  | 'K'  |
    /// | peer is neither choked nor interested | '?'  |
    /// | peer connection is encrypted          | 'E'  |
    /// | peer discovered through [DHT]         | 'H'  |
    /// | peer discovered through [PEX]         | 'X'  |
    /// | peer is an incoming connection        | 'I'  |
    ///
    /// [1]: <https://www.bittorrent.org/beps/bep_0003.html#peer-protocol>
    /// [DHT]: https://wikipedia.org/wiki/Distributed_hash_table
    /// [PEX]: https://wikipedia.org/wiki/Peer_exchange
    #[serde(alias = "flag_str")]
    pub flag_str: String,
    /// Whether we are downloading from the peer.
    #[serde(alias = "is_downloading_from")]
    pub is_downloading_from: bool,
    /// Whether the connection to the peer is encrypted.
    #[serde(alias = "is_encrypted")]
    pub is_encrypted: bool,
    /// Whether the peer is an incoming connection.
    #[serde(alias = "is_incoming")]
    pub is_incoming: bool,
    /// Whether we are uploading to the peer.
    #[serde(alias = "is_uploading_to")]
    pub is_uploading_to: bool,
    /// Whether the connection to the peer is [uTP].
    ///
    /// > Added in Transmission 2.30 (`rpc-version-semver` 4.0.0, `rpc-version`: 13)
    ///
    /// [uTP]: https://wikipedia.org/wiki/Micro_Transport_Protocol
    #[serde(alias = "isUTP")]
    #[serde(alias = "is_utp")]
    pub is_utp: bool,
    /// A string of length 20 which this downloader uses as its id. Each downloader generates its
    /// own id at random at the start of a new download. This value will also almost certainly have
    /// to be escaped.<sup>[[1]]</sup>
    ///
    /// > (?) Added in Transmission 4.1.0 (` rpc_version_semver ` 6.0.0, `rpc_version`: 18) [in
    /// [transmission:5db90f9ed]]
    ///
    /// [1]: <https://www.bittorrent.org/beps/bep_0003.html#trackers>
    /// [transmission:5db90f9ed]:
    /// <https://github.com/transmission/transmission/commit/5db90f9ed952edfc645d2503a3c0d69c26ef72db>
    #[serde(default, alias = "peer_id")]
    pub peer_id: String,
    /// Whether or not we've choked this peer.
    #[serde(alias = "peer_is_choked")]
    pub peer_is_choked: bool,
    /// Whether or not the peer has indicated it will download from us.
    #[serde(alias = "peer_is_interested")]
    pub peer_is_interested: bool,
    /// The peer's port.
    ///
    /// > Added in Transmission 1.40 (`rpc-version-semver` 1.1.0, `rpc-version`: 2)
    pub port: u16,
    /// The peer's torrent download progress.
    pub progress: f32,
    #[serde(alias = "rate_to_client")]
    /// The rate we are downloading from the peer in `B/s`.
    pub rate_to_client: u64,
    /// The rate we are uploading to the peer in `B/s`.
    #[serde(alias = "rate_to_peer")]
    pub rate_to_peer: u64,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PeersFrom {
    /// Peers found in the [.resume file].
    ///
    /// [.resume file]:
    /// <https://github.com/transmission/transmission/blob/main/docs/Transmission-Resume-Files.md>
    #[serde(alias = "from_cache")]
    pub from_cache: u16,
    /// Peers found from the [DHT].
    ///
    /// [DHT]: <https://wikipedia.org/wiki/Distributed_hash_table>
    #[serde(alias = "from_dht")]
    pub from_dht: u16,
    /// Connections made to the listening port.
    #[serde(alias = "from_incoming")]
    pub from_incoming: u16,
    /// Peers found by local announcements.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    #[serde(alias = "from_lpd")]
    pub from_lpd: u16,
    /// Peer address provided in an [LTEP] handshake.
    ///
    /// [LTEP]: <https://www.bittorrent.org/beps/bep_0010.html>
    #[serde(alias = "from_ltep")]
    pub from_ltep: u16,
    /// Peers found from [PEX].
    ///
    /// [PEX]: <https://wikipedia.org/wiki/Peer_exchange>
    #[serde(alias = "from_pex")]
    pub from_pex: u16,
    /// Peers found from a tracker.
    #[serde(alias = "from_tracker")]
    pub from_tracker: u16,
}

/// Represents the [`Torrent`]'s current status.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TorrentStatus {
    /// Torrent is stopped.
    Stopped = 0,
    /// Queued to check files.
    QueuedToVerify = 1,
    /// Checking files.
    Verifying = 2,
    /// Queued to download.
    QueuedToDownload = 3,
    /// Downloading.
    Downloading = 4,
    /// Queued to seed.
    QueuedToSeed = 5,
    /// Seeding.
    Seeding = 6,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Tracker {
    /// Unique transmission-generated ID for use in libtransmission API.
    pub id: TrackerId,
    /// The tracker's full announce URL.
    pub announce: Url,
    /// The tracker's full scrape URL.
    pub scrape: Url,
    /// The tracker site's name. Uses the first label before the [public suffix] in the announce
    /// URL's host. e.g. `https://www.example.co.uk/announce/`'s sitename is `example`.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17)
    ///
    /// [public suffix]: <https://publicsuffix.org/>
    #[serde(default)]
    pub sitename: String,
    /// which tier this tracker is in.
    pub tier: usize,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackerStat {
    /// The tracker's full announce URL.
    pub announce: Url,
    /// Whether we're announcing, waiting to announce, etc.
    #[serde(alias = "announce_state")]
    pub announce_state: TrackerState,
    /// Number of times this torrent's been downloaded, or -1 if unknown.
    #[serde(alias = "download_count")]
    pub download_count: i64,
    /// Number of downloaders ([BEP-21]) the tracker knows of, or -1 if unknown.
    ///
    /// This does not include partial seeds (peers that are incomplete and are no longer
    /// downloading in multi-file torrents).
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [BEP-21]: <https://www.bittorrent.org/beps/bep_0021.html>
    #[serde(alias = "downloader_count", default = "missing_downloader_count")]
    pub downloader_count: i64, // TODO? Option<_> differentiate between unknown and unimplemented?
    /// True iff we've announced to this tracker during this session.
    #[serde(alias = "has_announced")]
    pub has_announced: bool,
    /// True iff we've scraped this tracker during this session.
    #[serde(alias = "has_scraped")]
    pub has_scraped: bool,
    /// Uniquely-identifying tracker name (`${host}:${port}`)
    pub host: String,
    /// Unique transmission-generated ID for use in libtransmission API.
    pub id: TrackerId,
    /// Only one tracker per tier is used; the others are kept as backups.
    #[serde(alias = "is_backup")]
    pub is_backup: bool,
    /// If [`has_announced`], the number of peers the tracker gave us.
    ///
    /// [`has_announced`]: Self::has_announced
    #[serde(alias = "last_announce_peer_count")]
    pub last_announce_peer_count: i64,
    /// If [`has_announced`], the human-readable result of latest announce.
    ///
    /// [`has_announced`]: Self::has_announced
    #[serde(alias = "last_announce_result")]
    pub last_announce_result: String,
    /// If [`has_announced`], when the latest announce request was sent.
    ///
    /// [`has_announced`]: Self::has_announced
    #[serde(deserialize_with = "from_ts")]
    #[serde(alias = "last_announce_start_time")]
    pub last_announce_start_time: DateTime<Utc>,
    /// If [`has_announced`], whether or not the latest announce succeeded
    ///
    /// [`has_announced`]: Self::has_announced
    #[serde(alias = "last_announce_succeeded")]
    pub last_announce_succeeded: bool,
    /// If [`has_announced`], when the latest announce reply was received.
    ///
    /// [`has_announced`]: Self::has_announced
    #[serde(deserialize_with = "from_ts")]
    #[serde(alias = "last_announce_time")]
    pub last_announce_time: DateTime<Utc>,
    /// True iff the latest announce request timed out.
    #[serde(alias = "last_announce_timed_out")]
    pub last_announce_timed_out: bool,
    /// If [`has_scraped`], the human-readable result of the latest scrape.
    ///
    /// [`has_scraped`]: Self::has_scraped
    #[serde(alias = "last_scrape_result")]
    pub last_scrape_result: String,
    /// If [`has_scraped`], when the latest scrape request was sent.
    ///
    /// [`has_scraped`]: Self::has_scraped
    #[serde(deserialize_with = "from_ts")]
    #[serde(alias = "last_scrape_start_time")]
    pub last_scrape_start_time: DateTime<Utc>,
    /// If [`has_scraped`], whether or not the latest scrape succeeded.
    ///
    /// [`has_scraped`]: Self::has_scraped
    #[serde(alias = "last_scrape_succeeded")]
    pub last_scrape_succeeded: bool,
    /// If [`has_scraped`], when the latest scrape reply was received.
    ///
    /// [`has_scraped`]: Self::has_scraped
    #[serde(deserialize_with = "from_ts")]
    #[serde(alias = "last_scrape_time")]
    pub last_scrape_time: DateTime<Utc>,
    /// True iff the latest scrape request timed out.
    #[serde(alias = "last_scrape_timed_out")]
    pub last_scrape_timed_out: bool,
    /// Number of leechers the tracker knows of, or -1 if unknown.
    #[serde(alias = "leecher_count")]
    pub leecher_count: i64,
    /// If [`announce_state`] == [`TrackerState::Waiting`], time of next announce; otherwise:
    /// [`DateTime::UNIX_EPOCH`].
    ///
    /// [`announce_state`]: Self::announce_state
    #[serde(deserialize_with = "from_ts")]
    #[serde(alias = "next_announce_time")]
    pub next_announce_time: DateTime<Utc>,
    /// If [`scrape_state`] == [`TrackerState::Waiting`], time of next scrape; otherwise:
    /// [`DateTime::UNIX_EPOCH`].
    ///
    /// [`scrape_state`]: Self::scrape_state
    #[serde(deserialize_with = "from_ts")]
    #[serde(alias = "next_scrape_time")]
    pub next_scrape_time: DateTime<Utc>,
    /// Whether we're scraping, waiting to scrape, etc.
    #[serde(alias = "scrape_state")]
    pub scrape_state: TrackerState,
    /// The tracker's full scrape URL.
    pub scrape: Url,
    /// Number of seeders the tracker knows of, or -1 if unknown.
    #[serde(alias = "seeder_count")]
    pub seeder_count: i64,
    /// The tracker site's name. Uses the first label before the [public suffix] in the announce
    /// URL's host. e.g. `https://www.example.co.uk/announce/`'s sitename is `example`.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17)
    ///
    /// [public suffix]: <https://publicsuffix.org/>
    #[serde(default)]
    pub sitename: String,
    /// Which tier this tracker is in.
    pub tier: usize,
}

/// Represents the state of a torrent [`Tracker`].
#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TrackerState {
    /// We won't (announce,scrape) this torrent to this tracker because the torrent is stopped, or
    /// because of an error, or whatever.
    Inactive = 0,
    /// We will (announce,scrape) this torrent to this tracker, and are waiting for enough time to
    /// pass to satisfy the tracker's interval.
    Waiting = 1,
    /// It's time to (announce,scrape) this torrent, and we're waiting on a free slot to open up in
    /// the announce manager.
    Queued = 2,
    /// We're (announcing,scraping) this torrent right now.
    Active = 3,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")] // Added after semver-6.0.0.
pub struct WebseedsEx {
    /// The url to download from.
    pub url: Url,
    /// Can be true even if speed is 0, e.g. slow download
    pub is_downloading: bool,
    /// Current download speed
    pub download_bytes_per_second: u64,
}

fn from_ts<'de, D>(deserializer: D) -> StdResult<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: i64 = Deserialize::deserialize(deserializer)?;
    // The transmission rpc server responds with 0 or -1 (in the case of manualAnnounceTime /
    // manual_announce_time) when the date is unset or invalid.
    // Consolidate any response <= 0 as UNIX_EPOCH to denote these cases.
    if ts <= 0 {
        return Ok(DateTime::UNIX_EPOCH);
    }
    DateTime::<Utc>::from_timestamp(ts, 0)
        .ok_or_else(|| D::Error::custom(format!("timestamp out of range: {ts}")))
}

fn from_ts_option<'de, D>(deserializer: D) -> StdResult<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    from_ts(deserializer)
        .map(|dt| Some(dt))
}

/// Attempts to deserialize a [`base64`]-encoded string into a `Vec<u8>`.
///
/// [`base64`]: mod@base64
fn from_bitfield_option<'de, D>(deserializer: D) -> StdResult<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: &str = Deserialize::deserialize(deserializer)?;
    let bitfield = base64.decode(encoded).map_err(D::Error::custom)?;
    Ok(Some(bitfield))
}

/// Attempts to deserialize an array of bools or ints (`0` or `1`) into a `Vec<bool>`, treating `0`
/// as false and `1` as true.
fn from_arr_bool_option<'de, D>(deserializer: D) -> StdResult<Option<Vec<bool>>, D::Error>
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
                Value::Number(num) => match num.as_i64() {
                    Some(0) => Ok(false),
                    Some(1) => Ok(true),
                    Some(x) => Err(D::Error::custom(format!("unexpected number: {x}"))),
                    None => Err(unexpected()),
                },
                // rpc server misbehaving (got an unexpected type).
                _ => Err(unexpected()),
            })
            .collect::<StdResult<Vec<_>, _>>()?,
        // `wanted` should be an array.
        _ => Err(unexpected())?,
    };
    Ok(Some(wanted))
}

fn missing_downloader_count() -> i64 {
    -1
}

#[cfg(test)]
mod legacy_tests;
#[cfg(test)]
mod peer_tests;
#[cfg(test)]
mod semver_600_tests;
#[cfg(test)]
mod sub_type_tests;
#[cfg(test)]
mod tracker_stat_tests;
