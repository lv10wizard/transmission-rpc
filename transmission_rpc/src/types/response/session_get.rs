use semver::Version;
use serde::Deserialize;
use url::Url;

use crate::types::{AltSpeedDay, Encryption, TrackerList, Transport};

/// Represents the response argument of a successful [`session_get`] request.
///
/// [`session_get`]: crate::TransClient::session_get
#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionGet {
    /// Max global download speed (kB/s).
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "alt_speed_down")]
    pub alt_speed_down: Option<u64>,
    /// `true` means use the alt speeds (ie, turtle mode).
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "alt_speed_enabled")]
    pub alt_speed_enabled: Option<bool>,
    /// When to turn on alt speeds (units: minutes after midnight).
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "alt_speed_time_begin")]
    pub alt_speed_time_begin: Option<u64>,
    /// What day(s) to turn on alt speeds (ie. turtle mode).
    #[serde(alias = "alt_speed_time_day")]
    pub alt_speed_time_day: Option<AltSpeedDay>,
    /// `true` means the scheduled on/off times are used.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "alt_speed_time_enabled")]
    pub alt_speed_time_enabled: Option<bool>,
    /// when to turn off alt speeds (units: minutes after midnight).
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "alt_speed_time_end")]
    pub alt_speed_time_end: Option<u64>,
    /// Max global upload speed (kB/s).
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "alt_speed_up")]
    pub alt_speed_up: Option<u64>,
    /// `true` means to enable a basic brute force protection for RPC server..
    ///
    /// > (?) Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(alias = "anti_brute_force_enabled")]
    pub anti_brute_force_enabled: Option<bool>,
    /// After this amount of failed authentication attempts is surpassed, the RPC server will deny
    /// any further authentication attempts until it is restarted. This is not tracked per IP but
    /// in total.
    ///
    /// > (?) Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18) [NOT
    /// DOCUMENTED IN RPC-SPEC]
    #[serde(alias = "anti_brute_force_threshold")]
    pub anti_brute_force_threshold: Option<u64>,
    /// `true` means block peers based on the configured blocklist (see: [blocklists.md]).
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [blocklists.md]:
    /// <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    #[serde(alias = "blocklist_enabled")]
    pub blocklist_enabled: Option<bool>,
    /// Number of rules in the [blocklists].
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    ///
    /// [blocklists]: <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    #[serde(alias = "blocklist_size")]
    pub blocklist_size: Option<u64>,
    /// location of the blocklist to use for `blocklist-update`.
    ///
    /// > Added in Transmission 2.12 (`rpc-version-semver` 3.5.0, `rpc-version`: 11)
    #[serde(alias = "blocklist_url")]
    pub blocklist_url: Option<Url>,
    /// Maximum size of the disk cache (MiB). Pieces are guaranteed to be written to filesystem if
    /// sequential download is enabled. Otherwise, data might still be in cache only.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// > Renamed `cache_size_mb` to `cache_size_mib` in Transmission 4.1.0 (`rpc-version-semver`
    /// > 6.0.0,, `rpc-version`: 18)
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?):
    /// The memory cache is being removed, making this setting moot. The setting will still be
    /// gettable and settable via RPC session_get and session_set until Transmission
    /// 5.0.0 to avoid client breakage, but it will be otherwise unused in libtransmission. Clients
    ///   should stop using this key.
    #[serde(alias = "cache_size_mib")]
    #[serde(alias = "cache_size_mb")]
    // TODO: #[deprecated = ... ]
    pub cache_size_mb: Option<u64>,
    /// Location of transmission's configuration directory.
    ///
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    #[serde(alias = "config_dir")]
    pub config_dir: Option<String>,
    /// Announce URLs, one per line, and a blank line between [tiers].
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [tiers]: https://www.bittorrent.org/beps/bep_0012.html
    #[serde(alias = "default_trackers")]
    pub default_trackers: Option<TrackerList>,
    /// `true` means allow [DHT] in public torrents.
    ///
    /// [DHT]: https://wikipedia.org/wiki/Distributed_hash_table
    #[serde(alias = "dht_enabled")]
    pub dht_enabled: Option<bool>,
    /// Default path to download torrents
    #[serde(alias = "download_dir")]
    pub download_dir: Option<String>,
    /// > Added in Transmission 2.20 (`rpc-version-semver` 3.6.0, `rpc-version`: 12)
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17):
    /// Use the `free-space` method instead.
    #[serde(alias = "download_dir_free_space")]
    pub download_dir_free_space: Option<u64>,
    /// If `true`, limit how many torrents can be downloaded at once.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    #[serde(alias = "download_queue_enabled")]
    pub download_queue_enabled: Option<bool>,
    /// Max number of torrents to download at once (see [`download_queue_enabled`]).
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`download_queue_enabled`]: Self::download_queue_enabled
    #[serde(alias = "download_queue_size")]
    pub download_queue_size: Option<u64>,
    /// The encryption type for peer connections.
    #[serde(alias = "encryption")]
    pub encryption: Option<Encryption>,
    /// `true` if the seeding inactivity limit is honored by default.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    #[serde(alias = "idle_seeding_limit_enabled")]
    pub idle_seeding_limit_enabled: Option<bool>,
    /// torrents we're seeding will be stopped if they're idle for this long (in minutes).
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    #[serde(alias = "idle_seeding_limit")]
    pub idle_seeding_limit: Option<u64>,
    /// `true` means keep torrents in [`incomplete_dir`] until done.
    ///
    /// > Added in Transmission 1.80 (`rpc-version-semver` 3.0.0, `rpc-version`: 7)
    ///
    /// [`incomplete_dir`]: Self::incomplete_dir
    #[serde(alias = "incomplete_dir_enabled")]
    pub incomplete_dir_enabled: Option<bool>,
    /// Path for incomplete torrents, when enabled.
    ///
    /// > Added in Transmission 1.80 (`rpc-version-semver` 3.0.0, `rpc-version`: 7)
    #[serde(alias = "incomplete_dir")]
    pub incomplete_dir: Option<String>,
    /// `true` means allow Local Peer Discovery in public torrents.
    #[serde(alias = "lpd_enabled")]
    pub lpd_enabled: Option<bool>,
    /// Maximum global number of peers.
    ///
    /// > Renamed from `peer-limit` to `peer-limit-global` in Transmission 1.60
    /// (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "peer_limit_global")]
    pub peer_limit_global: Option<u64>,
    /// Default maximum number of peers per torrent.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "peer_limit_per_torrent")]
    pub peer_limit_per_torrent: Option<u64>,
    /// `true` means pick a random peer port on launch.
    #[serde(alias = "peer_port_random_on_start")]
    pub peer_port_random_on_start: Option<bool>,
    /// The port that the daemon listens for peer connections on.
    ///
    /// > Renamed from `port` to `peer-port` in Transmission 1.60 (`rpc-version-semver` 2.0.0,
    /// `rpc-version`: 5)
    #[serde(alias = "peer_port")]
    pub peer_port: Option<u16>,
    /// `true` means allow [PEX] in public torrents.
    ///
    /// > Renamed from `pex-allowed` to `pex-enabled` in Transmission 1.60 (`rpc-version-semver`
    /// 2.0.0, `rpc-version`: 5)
    ///
    /// [PEX]: https://wikipedia.org/wiki/Peer_exchange
    #[serde(alias = "pex_enabled")]
    pub pex_enabled: Option<bool>,
    /// `true` means ask upstream router to forward the configured peer port to transmission using
    /// [UPnP] or [NAT-PMP].
    ///
    /// [UPnP]: https://wikipedia.org/wiki/Universal_Plug_and_Play
    /// [NAT-PMP]: https://wikipedia.org/wiki/NAT_Port_Mapping_Protocol
    #[serde(alias = "port_forwarding_enabled")]
    pub port_forwarding_enabled: Option<bool>,
    /// List of preferred transport protocols in the order of preferred-first.
    ///
    /// * ["utp"](https://en.wikipedia.org/wiki/Micro_Transport_Protocol)
    /// * ["tcp"](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    #[serde(alias = "preferred_transports")]
    pub preferred_transports: Option<Vec<Transport>>,
    /// Whether or not to consider idle torrents as stalled.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    #[serde(alias = "queue_stalled_enabled")]
    pub queue_stalled_enabled: Option<bool>,
    /// Torrents that are idle for `queue_stalled_minutes` aren't counted toward
    /// [`seed_queue_size`] or [`download-queue-size`].
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_size`]: Self::seed_queue_size
    /// [`download-queue-size`]: Self::download-queue-size
    #[serde(alias = "queue_stalled_minutes")]
    pub queue_stalled_minutes: Option<u64>,
    /// `true` means append `.part` to incomplete files.
    /// 
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    #[serde(alias = "rename_partial_files")]
    pub rename_partial_files: Option<bool>,
    /// The number of outstanding block requests a peer is allowed to queue in the client.
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    #[serde(alias = "reqq")]
    pub reqq: Option<u64>,
    /// The minimum RPC API version supported by the RPC server. It changes when a new version of
    /// Transmission changes the RPC interface in a way that is not backwards compatible.
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use `rpc_version_semver` instead.
    #[serde(alias = "rpc_version_minimum")]
    pub rpc_version_minimum: Option<i32>, // TODO: i32 -> RpcVersion
    /// The current RPC API version in a [semver]-compatible string.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [semver]: https://semver.org/
    #[serde(alias = "rpc_version_semver")]
    pub rpc_version_semver: Option<Version>,
    /// the current RPC API version.
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use `rpc_version_semver` instead.
    #[serde(alias = "rpc_version")]
    pub rpc_version: Option<i32>, // TODO: i32 -> RpcVersion
    /// Whether or not to call the [added script] (see: [scripts.md]).
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [added script]: Self::script_torrent_added_filename
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    #[serde(alias = "script_torrent_added_enabled")]
    pub script_torrent_added_enabled: Option<bool>,
    /// Path of the script to run on torrent added (see: [scripts.md]).
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    #[serde(alias = "script_torrent_added_filename")]
    pub script_torrent_added_filename: Option<String>,
    /// Whether or not to call the [done script] (see: [scripts.md]).
    ///
    /// [done script]: Self::script_torrent_done_filename
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    #[serde(alias = "script_torrent_done_enabled")]
    pub script_torrent_done_enabled: Option<bool>,
    /// Path of the script to run on torrent completion (see: [scripts.md]).
    ///
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    #[serde(alias = "script_torrent_done_filename")]
    pub script_torrent_done_filename: Option<String>,
    /// Whether or not to call the [seeding-done] script (see: [scripts.md]).
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [seeding-done]: Self::script_torrent_done_seeding_filename
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    #[serde(alias = "script_torrent_done_seeding_enabled")]
    pub script_torrent_done_seeding_enabled: Option<bool>,
    /// Path of the script to run on torrent seeding completion (see: [scripts.md]).
    /// 
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    ///
    /// [scripts.md]: https://github.com/transmission/transmission/blob/main/docs/Scripts.md
    #[serde(alias = "script_torrent_done_seeding_filename")]
    pub script_torrent_done_seeding_filename: Option<String>,
    /// if `true`, limit how many torrents can be uploaded at once.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    #[serde(alias = "seed_queue_enabled")]
    pub seed_queue_enabled: Option<bool>,
    /// Max number of torrents to uploaded at once (see [seed_queue_enabled]).
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [seed_queue_enabled]: Self::seed_queue_enabled
    #[serde(alias = "seed_queue_size")]
    pub seed_queue_size: Option<u64>,
    /// The default seed ratio for torrents to use.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(alias = "seedRatioLimit")]
    #[serde(alias = "seed_ratio_limit")]
    pub seed_ratio_limit: Option<f64>,
    /// `true` if [`seed_ratio_limit`] is honored by default.
    ///
    /// [`seed_ratio_limit`]: Self::seed_ratio_limit
    #[serde(alias = "seedRatioLimited")]
    #[serde(alias = "seed_ratio_limited")]
    pub seed_ratio_limited: Option<bool>,
    /// `true` means sequential download is enabled by default for added torrents.
    /// 
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    #[serde(alias = "sequential_download")]
    pub sequential_download: Option<bool>,
    /// The current [`X-Transmission-Session-Id`] value.
    ///
    /// > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16)
    ///
    /// [`X-Transmission-Session-Id`]: 
    /// <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#231-csrf-protection>
    #[serde(alias = "session_id")]
    pub session_id: Option<String>,
    /// `true` means limit global download speed.
    #[serde(alias = "speed_limit_down_enabled")]
    pub speed_limit_down_enabled: Option<bool>,
    /// Max global download speed (kB/s).
    #[serde(alias = "speed_limit_down")]
    pub speed_limit_down: Option<u64>,
    /// `true` means limit global upload speed.
    #[serde(alias = "speed_limit_up_enabled")]
    pub speed_limit_up_enabled: Option<bool>,
    /// Max global upload speed (kB/s).
    #[serde(alias = "speed_limit_up")]
    pub speed_limit_up: Option<u64>,
    /// `true` means added torrents will be started right away.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    #[serde(alias = "start_added_torrents")]
    pub start_added_torrents: Option<bool>,
    /// `true` means allow [TCP].
    ///
    /// > (?) Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17) [in
    /// [transmission:fa8b6a5e0]].
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// `tcp_enabled`. Use `preferred_transports` instead.
    ///
    /// [TCP]: https://en.wikipedia.org/wiki/Transmission_Control_Protocol
    /// [transmission:fa8b6a5e0]:
    /// <https://github.com/transmission/transmission/commit/fa8b6a5e0aa22fe2f798b7c6dcc9c32dbac63dca>
    #[serde(alias = "tcp_enabled")]
    pub tcp_enabled: Option<bool>,
    /// `true` means the `.torrent` file of added torrents will be deleted.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    #[serde(alias = "trash_original_torrent_files")]
    pub trash_original_torrent_files: Option<bool>,
    /// The units used by the daemon (I think?).
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    #[serde(alias = "units")]
    pub units: Option<SessionGetUnits>,
    /// `true` means allow [uTP].
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// `utp_enabled`. Use `preferred_transports` instead.
    ///
    /// [uTP]: https://wikipedia.org/wiki/Micro_Transport_Protocol
    #[serde(alias = "utp_enabled")]
    pub utp_enabled: Option<bool>,
    /// Long version string, ie. `$version ($revision)`.
    ///
    /// > Added in Transmission 1.41 (`rpc-version-semver` 1.2.0, `rpc-version`: 3)
    #[serde(alias = "version")]
    pub version: Option<String>,
}

/// The units used by the [Transmission] instance. See: [`SessionGet::units`].
///
/// [Transmission]: <https://transmissionbt.com/>
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionGetUnits {
    /// 4 strings: KB/s, MB/s, GB/s, TB/s
    ///
    /// v4.1.1: 5 strings: B/s, KB/s, MB/s, GB/s, TB/s
    #[serde(alias = "speed_units")]
    pub speed_units: Vec<String>,
    /// number of bytes in a KB (1000 for kB; 1024 for KiB)
    #[serde(alias = "speed_bytes")]
    pub speed_bytes: usize,
    /// 4 strings: KB, MB, GB, TB
    ///
    /// v4.1.1: 5 strings: B, KB, MB, GB, TB
    #[serde(alias = "size_units")]
    pub size_units: Vec<String>,
    /// number of bytes in a KB (1000 for kB; 1024 for KiB)
    #[serde(alias = "size_bytes")]
    pub size_bytes: usize,
    /// 4 strings: KiB, MiB, GiB, TiB
    ///
    /// v4.1.1: 5 strings: B, KiB, MiB, GiB, TiB
    #[serde(alias = "memory_units")]
    pub memory_units: Vec<String>,
    /// number of bytes in a KB (1000 for kB; 1024 for KiB)
    #[serde(alias = "memory_bytes")]
    pub memory_bytes: usize,
}

#[cfg(test)]
mod legacy_deser_tests {
    use test_case::test_case;

    use crate::types::RpcResponse;
    use super::*;

    #[test_case(r#"{ "alt-speed-down": 500 }"# => SessionGet {
            alt_speed_down: Some(500),
            ..Default::default()
        } ; "legacy alt speed down"
    )]
    #[test_case(r#"{ "alt-speed-down": 0 }"# => SessionGet {
            alt_speed_down: Some(0),
            ..Default::default()
        } ; "legacy alt speed down zero"
    )]
    #[test_case(r#"{ "alt-speed-down": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy alt speed down negative"
    )]

    #[test_case(r#"{ "alt-speed-enabled": true }"# => SessionGet {
            alt_speed_enabled: Some(true),
            ..Default::default()
        } ; "legacy alt speed enabled"
    )]

    #[test_case(r#"{ "alt-speed-time-begin": 2 }"# => SessionGet {
            alt_speed_time_begin: Some(2),
            ..Default::default()
        } ; "legacy alt speed time begin"
    )]
    #[test_case(r#"{ "alt-speed-time-begin": 0 }"# => SessionGet {
            alt_speed_time_begin: Some(0),
            ..Default::default()
        } ; "legacy alt speed time begin zero"
    )]
    #[test_case(r#"{ "alt-speed-time-begin": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy alt speed time begin negative"
    )]

    #[test_case(r#"{ "alt-speed-time-day": 42 }"# => SessionGet {
            alt_speed_time_day: Some(
                AltSpeedDay::MONDAY | AltSpeedDay::WEDNESDAY | AltSpeedDay::FRIDAY
            ),
            ..Default::default()
        } ; "legacy alt speed time day"
    )]

    #[test_case(r#"{ "alt-speed-time-enabled": false }"# => SessionGet {
            alt_speed_time_enabled: Some(false),
            ..Default::default()
        } ; "legacy alt speed time enabled"
    )]

    #[test_case(r#"{ "alt-speed-time-end": 666 }"# => SessionGet {
            alt_speed_time_end: Some(666),
            ..Default::default()
        } ; "legacy alt speed time end"
    )]
    #[test_case(r#"{ "alt-speed-time-end": 0 }"# => SessionGet {
            alt_speed_time_end: Some(0),
            ..Default::default()
        } ; "legacy alt speed time end zero"
    )]
    #[test_case(r#"{ "alt-speed-time-end": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy alt speed time end negative"
    )]

    #[test_case(r#"{ "alt-speed-up": 1200 }"# => SessionGet {
            alt_speed_up: Some(1200),
            ..Default::default()
        } ; "legacy alt speed up"
    )]
    #[test_case(r#"{ "alt-speed-up": 0 }"# => SessionGet {
            alt_speed_up: Some(0),
            ..Default::default()
        } ; "legacy alt speed up zero"
    )]
    #[test_case(r#"{ "alt-speed-up": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy alt speed up negative"
    )]

    // NOTE: No legacy anti_brute_force_enabled test: doesn't exist pre- semver-6.0.0.
    // NOTE: No legacy anti_brute_force_threshold test: doesn't exist pre- semver-6.0.0.

    #[test_case(r#"{ "blocklist-enabled": true }"# => SessionGet {
            blocklist_enabled: Some(true),
            ..Default::default()
        } ; "legacy blocklist enabled"
    )]

    #[test_case(r#"{ "blocklist-size": 123 }"# => SessionGet {
            blocklist_size: Some(123),
            ..Default::default()
        } ; "legacy blocklist size"
    )]
    #[test_case(r#"{ "blocklist-size": 0 }"# => SessionGet {
            blocklist_size: Some(0),
            ..Default::default()
        } ; "legacy blocklist size zero"
    )]
    #[test_case(r#"{ "blocklist-size": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy blocklist size negative"
    )]

    #[test_case(r#"{ "blocklist-url": "http://example.com/lists/test.txt" }"# => SessionGet {
            blocklist_url: Some(
                Url::parse("http://example.com/lists/test.txt").expect("valid url")
            ),
            ..Default::default()
        } ; "legacy blocklist url"
    )]
    #[test_case(r#"{ "blocklist-url": "malformed" }"# => panics "relative URL without a base"
        ; "legacy blocklist url malformed"
    )]

    #[test_case(r#"{ "cache-size-mb": 16 }"# => SessionGet {
            cache_size_mb: Some(16),
            ..Default::default()
        } ; "legacy cache size mb"
    )]
    #[test_case(r#"{ "cache-size-mb": 0 }"# => SessionGet {
            cache_size_mb: Some(0),
            ..Default::default()
        } ; "legacy cache size mb zero"
    )]
    #[test_case(r#"{ "cache-size-mb": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy cache size mb negative"
    )]

    #[test_case(r#"{ "config-dir": "/home/user/config/transmission" }"# => SessionGet {
            config_dir: Some("/home/user/config/transmission".into()),
            ..Default::default()
        } ; "legacy config dir"
    )]

    #[test_case(r#"{
        "default-trackers": "http://one.example.com\n\nhttp://two.example.com\n"
    }"# => SessionGet {
            default_trackers: Some(vec![
                ["http://one.example.com"],
                ["http://two.example.com"],
            ].into()),
            ..Default::default()
        } ; "legacy default trackers"
    )]
    #[test_case(r#"{ "default-trackers": "" }"# => SessionGet {
            default_trackers: Some(TrackerList(vec![])),
            ..Default::default()
        } ; "legacy default trackers empty"
    )]

    #[test_case(r#"{ "dht-enabled": true }"# => SessionGet {
            dht_enabled: Some(true),
            ..Default::default()
        } ; "legacy dht enabled"
    )]

    #[test_case(r#"{ "download-dir": "/home/user/downloads" }"# => SessionGet {
            download_dir: Some("/home/user/downloads".into()),
            ..Default::default()
        } ; "legacy download dir"
    )]
    #[test_case(r#"{ "download-dir-free-space": 123456 }"# => SessionGet {
            download_dir_free_space: Some(123456),
            ..Default::default()
        } ; "legacy download dir free space"
    )]
    #[test_case(r#"{ "download-dir-free-space": 0 }"# => SessionGet {
            download_dir_free_space: Some(0),
            ..Default::default()
        } ; "legacy download dir free space zero"
    )]
    #[test_case(r#"{ "download-dir-free-space": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy download dir free space negative"
    )]

    #[test_case(r#"{ "download-queue-enabled": false }"# => SessionGet {
            download_queue_enabled: Some(false),
            ..Default::default()
        } ; "legacy download queue enabled"
    )]

    #[test_case(r#"{ "download-queue-size": 42 }"# => SessionGet {
            download_queue_size: Some(42),
            ..Default::default()
        } ; "legacy download queue size"
    )]
    #[test_case(r#"{ "download-queue-size": 0 }"# => SessionGet {
            download_queue_size: Some(0),
            ..Default::default()
        } ; "legacy download queue size zero"
    )]
    #[test_case(r#"{ "download-queue-size": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy download queue size negative"
    )]

    #[test_case(r#"{ "encryption": "tolerated" }"# => SessionGet {
            encryption: Some(Encryption::Tolerated),
            ..Default::default()
        } ; "legacy encryption"
    )]

    #[test_case(r#"{ "idle-seeding-limit-enabled": false }"# => SessionGet {
            idle_seeding_limit_enabled: Some(false),
            ..Default::default()
        } ; "legacy idle seeding limit enabled"
    )]

    #[test_case(r#"{ "idle-seeding-limit": 123 }"# => SessionGet {
            idle_seeding_limit: Some(123),
            ..Default::default()
        } ; "legacy idle seeding limit"
    )]
    #[test_case(r#"{ "idle-seeding-limit": 0 }"# => SessionGet {
            idle_seeding_limit: Some(0),
            ..Default::default()
        } ; "legacy idle seeding limit zero"
    )]
    #[test_case(r#"{ "idle-seeding-limit": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy idle seeding limit negative"
    )]

    #[test_case(r#"{ "incomplete-dir": "/home/user/incomplete" }"# => SessionGet {
            incomplete_dir: Some("/home/user/incomplete".into()),
            ..Default::default()
        } ; "legacy incomplete dir"
    )]

    #[test_case(r#"{ "lpd-enabled": true }"# => SessionGet {
            lpd_enabled: Some(true),
            ..Default::default()
        } ; "legacy lpd enabled"
    )]

    #[test_case(r#"{ "peer-limit-global": 3333 }"# => SessionGet {
            peer_limit_global: Some(3333),
            ..Default::default()
        } ; "legacy peer limit global"
    )]
    #[test_case(r#"{ "peer-limit-global": 0 }"# => SessionGet {
            peer_limit_global: Some(0),
            ..Default::default()
        } ; "legacy peer limit global zero"
    )]
    #[test_case(r#"{ "peer-limit-global": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy peer limit global negative"
    )]

    #[test_case(r#"{ "peer-limit-per-torrent": 3 }"# => SessionGet {
            peer_limit_per_torrent: Some(3),
            ..Default::default()
        } ; "legacy peer limit per torrent"
    )]
    #[test_case(r#"{ "peer-limit-per-torrent": 0 }"# => SessionGet {
            peer_limit_per_torrent: Some(0),
            ..Default::default()
        } ; "legacy peer limit per torrent zero"
    )]
    #[test_case(r#"{ "peer-limit-per-torrent": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy peer limit per torrent negative"
    )]

    #[test_case(r#"{ "peer-port-random-on-start": false }"# => SessionGet {
            peer_port_random_on_start: Some(false),
            ..Default::default()
        } ; "legacy peer port random on start"
    )]
    #[test_case(r#"{ "peer-port": 55555 }"# => SessionGet {
            peer_port: Some(55555),
            ..Default::default()
        } ; "legacy peer port"
    )]

    #[test_case(r#"{ "pex-enabled": true }"# => SessionGet {
            pex_enabled: Some(true),
            ..Default::default()
        } ; "legacy pex enabled"
    )]

    #[test_case(r#"{ "port-forwarding-enabled": false }"# => SessionGet {
            port_forwarding_enabled: Some(false),
            ..Default::default()
        } ; "legacy port forwarding enabled"
    )]

    // NOTE: No legacy preferred_transports test: doesn't exist pre- semver-6.0.0.

    #[test_case(r#"{ "queue-stalled-enabled": true }"# => SessionGet {
            queue_stalled_enabled: Some(true),
            ..Default::default()
        } ; "legacy queue stalled enabled"
    )]

    #[test_case(r#"{ "queue-stalled-minutes": 60 }"# => SessionGet {
            queue_stalled_minutes: Some(60),
            ..Default::default()
        } ; "legacy queue stalled minutes"
    )]
    #[test_case(r#"{ "queue-stalled-minutes": 0 }"# => SessionGet {
            queue_stalled_minutes: Some(0),
            ..Default::default()
        } ; "legacy queue stalled minutes zero"
    )]
    #[test_case(r#"{ "queue-stalled-minutes": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy queue stalled minutes negative"
    )]

    #[test_case(r#"{ "rename-partial-files": true }"# => SessionGet {
            rename_partial_files: Some(true),
            ..Default::default()
        } ; "legacy rename partial files"
    )]

    // NOTE: No legacy reqq test: doesn't exist pre- semver-6.0.0.

    #[test_case(r#"{ "rpc-version-minimum": 3 }"# => SessionGet {
            rpc_version_minimum: Some(3),
            ..Default::default()
        } ; "legacy rpc version minimum"
    )]
    #[test_case(r#"{ "rpc-version-minimum": 0 }"# => SessionGet {
            rpc_version_minimum: Some(0),
            ..Default::default()
        } ; "legacy rpc version minimum zero"
    )]
    #[test_case(r#"{ "rpc-version-minimum": -1 }"# => SessionGet {
            rpc_version_minimum: Some(-1),
            ..Default::default()
        } ; "legacy rpc version minimum negative"
    )]

    #[test_case(r#"{ "rpc-version-semver": "5.3.0" }"# => SessionGet {
            rpc_version_semver: Some(Version::parse("5.3.0").expect("valid semver")),
            ..Default::default()
        } ; "legacy rpc version semver"
    )]

    #[test_case(r#"{ "rpc-version": 18 }"# => SessionGet {
            rpc_version: Some(18),
            ..Default::default()
        } ; "legacy rpc version "
    )]
    #[test_case(r#"{ "rpc-version": 0 }"# => SessionGet {
            rpc_version: Some(0),
            ..Default::default()
        } ; "legacy rpc version zero"
    )]
    #[test_case(r#"{ "rpc-version": -1 }"# => SessionGet {
            rpc_version: Some(-1),
            ..Default::default()
        } ; "legacy rpc version negative"
    )]

    #[test_case(r#"{ "script-torrent-added-enabled": false }"# => SessionGet {
            script_torrent_added_enabled: Some(false),
            ..Default::default()
        } ; "legacy script torrent added enabled"
    )]
    #[test_case(r#"{ "script-torrent-added-filename": "/scripts/foo.sh" }"# => SessionGet {
            script_torrent_added_filename: Some("/scripts/foo.sh".into()),
            ..Default::default()
        } ; "legacy script torrent added filename"
    )]

    #[test_case(r#"{ "script-torrent-done-enabled": true }"# => SessionGet {
            script_torrent_done_enabled: Some(true),
            ..Default::default()
        } ; "legacy script torrent done enabled"
    )]
    #[test_case(r#"{ "script-torrent-done-filename": "/scripts/bar.sh" }"# => SessionGet {
            script_torrent_done_filename: Some("/scripts/bar.sh".into()),
            ..Default::default()
        } ; "legacy script torrent done filename"
    )]

    #[test_case(r#"{ "script-torrent-done-seeding-enabled": true }"# => SessionGet {
            script_torrent_done_seeding_enabled: Some(true),
            ..Default::default()
        } ; "legacy script torrent done seeding enabled"
    )]
    #[test_case(r#"{ "script-torrent-done-seeding-filename": "/scripts/baz.sh" }"# => SessionGet {
            script_torrent_done_seeding_filename: Some("/scripts/baz.sh".into()),
            ..Default::default()
        } ; "legacy script torrent done seeding filename"
    )]

    #[test_case(r#"{ "seed-queue-enabled": true }"# => SessionGet {
            seed_queue_enabled: Some(true),
            ..Default::default()
        } ; "legacy seed queue enabled"
    )]

    #[test_case(r#"{ "seed-queue-size": 23 }"# => SessionGet {
            seed_queue_size: Some(23),
            ..Default::default()
        } ; "legacy seed queue size"
    )]
    #[test_case(r#"{ "seed-queue-size": 0 }"# => SessionGet {
            seed_queue_size: Some(0),
            ..Default::default()
        } ; "legacy seed queue size zero"
    )]
    #[test_case(r#"{ "seed-queue-size": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy seed queue size negative"
    )]

    #[test_case(r#"{ "seedRatioLimit": 3.14 }"# => SessionGet {
            seed_ratio_limit: Some(3.14),
            ..Default::default()
        } ; "legacy seed ratio limit"
    )]
    #[test_case(r#"{ "seedRatioLimit": 0 }"# => SessionGet {
            seed_ratio_limit: Some(0.),
            ..Default::default()
        } ; "legacy seed ratio limit zero"
    )]
    #[test_case(r#"{ "seedRatioLimit": -1 }"# => SessionGet {
            seed_ratio_limit: Some(-1.),
            ..Default::default()
        } ; "legacy seed ratio limit negative"
    )]

    #[test_case(r#"{ "seedRatioLimited": true }"# => SessionGet {
            seed_ratio_limited: Some(true),
            ..Default::default()
        } ; "legacy seed ratio limited"
    )]

    // NOTE: No legacy sequential_download test: doesn't exist pre- semver-6.0.0.

    #[test_case(r#"{
        "session-id": "5C7Jx5rb62lGlJJo6Udy01hjGtlrfg23xaBn3YPkqUWF6uSX"
    }"# => SessionGet {
            session_id: Some("5C7Jx5rb62lGlJJo6Udy01hjGtlrfg23xaBn3YPkqUWF6uSX".into()),
            ..Default::default()
        } ; "legacy session id"
    )]

    #[test_case(r#"{ "speed-limit-down-enabled": true }"# => SessionGet {
            speed_limit_down_enabled: Some(true),
            ..Default::default()
        } ; "legacy speed limit down enabled"
    )]

    #[test_case(r#"{ "speed-limit-down": 1111 }"# => SessionGet {
            speed_limit_down: Some(1111),
            ..Default::default()
        } ; "legacy speed limit down"
    )]
    #[test_case(r#"{ "speed-limit-down": 0 }"# => SessionGet {
            speed_limit_down: Some(0),
            ..Default::default()
        } ; "legacy speed limit down zero"
    )]
    #[test_case(r#"{ "speed-limit-down": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy speed limit down negative"
    )]

    #[test_case(r#"{ "speed-limit-up-enabled": true }"# => SessionGet {
            speed_limit_up_enabled: Some(true),
            ..Default::default()
        } ; "legacy speed limit up enabled"
    )]

    #[test_case(r#"{ "speed-limit-up": 2222 }"# => SessionGet {
            speed_limit_up: Some(2222),
            ..Default::default()
        } ; "legacy speed limit up"
    )]
    #[test_case(r#"{ "speed-limit-up": 0 }"# => SessionGet {
            speed_limit_up: Some(0),
            ..Default::default()
        } ; "legacy speed limit up zero"
    )]
    #[test_case(r#"{ "speed-limit-up": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "legacy speed limit up negative"
    )]

    #[test_case(r#"{ "start-added-torrents": false }"# => SessionGet {
            start_added_torrents: Some(false),
            ..Default::default()
        } ; "legacy start added torrents"
    )]

    #[test_case(r#"{ "tcp-enabled": false }"# => SessionGet {
            tcp_enabled: Some(false),
            ..Default::default()
        } ; "legacy tcp enabled"
    )]

    #[test_case(r#"{ "trash-original-torrent-files": false }"# => SessionGet {
            trash_original_torrent_files: Some(false),
            ..Default::default()
        } ; "legacy trash original torrent files"
    )]

    #[test_case(r#"{
        "units": {
            "memory-bytes": 1024,
            "memory-units": [
                "KiB",
                "MiB",
                "GiB",
                "TiB"
            ],
            "size-bytes": 1000,
            "size-units": [
                "kB",
                "MB",
                "GB",
                "TB"
            ],
            "speed-bytes": 1000,
            "speed-units": [
                "kB/s",
                "MB/s",
                "GB/s",
                "TB/s"
            ]
        }
    }"# => SessionGet {
            units: Some(SessionGetUnits {
                memory_bytes: 1024,
                memory_units: vec![
                    "KiB".to_string(),
                    "MiB".to_string(),
                    "GiB".to_string(),
                    "TiB".to_string(),
                ],
                size_bytes: 1000,
                size_units: vec![
                    "kB".to_string(),
                    "MB".to_string(),
                    "GB".to_string(),
                    "TB".to_string(),
                ],
                speed_bytes: 1000,
                speed_units: vec![
                    "kB/s".to_string(),
                    "MB/s".to_string(),
                    "GB/s".to_string(),
                    "TB/s".to_string(),
                ],
            }),
            ..Default::default()
        } ; "legacy units"
    )]

    #[test_case(r#"{ "utp-enabled": true }"# => SessionGet {
            utp_enabled: Some(true),
            ..Default::default()
        } ; "legacy utp enabled"
    )]

    #[test_case(r#"{ "version": "3.00 (bb6b5a062e)" }"# => SessionGet {
            version: Some("3.00 (bb6b5a062e)".into()),
            ..Default::default()
        } ; "legacy version"
    )]

    fn session_get_deserialize(data: &str) -> SessionGet {
        let formatted = format!("{{\
            \"arguments\": {data},\
            \"result\":\"success\"\
        }}");
        println!("data>      {data}");
        println!("formatted> {formatted}");
        match serde_json::from_str::<RpcResponse<_>>(&formatted) {
            Ok(resp) => resp.arguments,
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod semver_600_deser_tests {
    use test_case::test_case;

    use crate::{
        JSON_RPC_VERSION_2_0, JsonRpcResponse,
        types::RpcResponse,
    };
    use super::*;

    #[test_case(r#"{ "alt_speed_down": 500 }"# => SessionGet {
            alt_speed_down: Some(500),
            ..Default::default()
        } ; "semver 6.0.0 alt speed down"
    )]
    #[test_case(r#"{ "alt_speed_down": 0 }"# => SessionGet {
            alt_speed_down: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 alt speed down zero"
    )]
    #[test_case(r#"{ "alt_speed_down": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 alt speed down negative"
    )]

    #[test_case(r#"{ "alt_speed_enabled": true }"# => SessionGet {
            alt_speed_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 alt speed enabled"
    )]

    #[test_case(r#"{ "alt_speed_time_begin": 2 }"# => SessionGet {
            alt_speed_time_begin: Some(2),
            ..Default::default()
        } ; "semver 6.0.0 alt speed time begin"
    )]
    #[test_case(r#"{ "alt_speed_time_begin": 0 }"# => SessionGet {
            alt_speed_time_begin: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 alt speed time begin zero"
    )]
    #[test_case(r#"{ "alt_speed_time_begin": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 alt speed time begin negative"
    )]

    #[test_case(r#"{ "alt_speed_time_day": 42 }"# => SessionGet {
            alt_speed_time_day: Some(
                AltSpeedDay::MONDAY | AltSpeedDay::WEDNESDAY | AltSpeedDay::FRIDAY
            ),
            ..Default::default()
        } ; "semver 6.0.0 alt speed time day"
    )]

    #[test_case(r#"{ "alt_speed_time_enabled": false }"# => SessionGet {
            alt_speed_time_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 alt speed time enabled"
    )]

    #[test_case(r#"{ "alt_speed_time_end": 666 }"# => SessionGet {
            alt_speed_time_end: Some(666),
            ..Default::default()
        } ; "semver 6.0.0 alt speed time end"
    )]
    #[test_case(r#"{ "alt_speed_time_end": 0 }"# => SessionGet {
            alt_speed_time_end: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 alt speed time end zero"
    )]
    #[test_case(r#"{ "alt_speed_time_end": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 alt speed time end negative"
    )]

    #[test_case(r#"{ "alt_speed_up": 1200 }"# => SessionGet {
            alt_speed_up: Some(1200),
            ..Default::default()
        } ; "semver 6.0.0 alt speed up"
    )]
    #[test_case(r#"{ "alt_speed_up": 0 }"# => SessionGet {
            alt_speed_up: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 alt speed up zero"
    )]
    #[test_case(r#"{ "alt_speed_up": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 alt speed up negative"
    )]

    #[test_case(r#"{ "anti_brute_force_enabled": false }"# => SessionGet {
            anti_brute_force_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 anti brute force enabled"
    )]

    #[test_case(r#"{ "anti_brute_force_threshold": 250 }"# => SessionGet {
            anti_brute_force_threshold: Some(250),
            ..Default::default()
        } ; "semver 6.0.0 anti brute force threshold"
    )]
    #[test_case(r#"{ "anti_brute_force_threshold": 0 }"# => SessionGet {
            anti_brute_force_threshold: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 anti brute force threshold zero"
    )]
    #[test_case(r#"{ "anti_brute_force_threshold": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 anti brute force threshold negative"
    )]

    #[test_case(r#"{ "blocklist_enabled": true }"# => SessionGet {
            blocklist_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 blocklist enabled"
    )]

    #[test_case(r#"{ "blocklist_size": 123 }"# => SessionGet {
            blocklist_size: Some(123),
            ..Default::default()
        } ; "semver 6.0.0 blocklist size"
    )]
    #[test_case(r#"{ "blocklist_size": 0 }"# => SessionGet {
            blocklist_size: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 blocklist size zero"
    )]
    #[test_case(r#"{ "blocklist_size": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 blocklist size negative"
    )]

    #[test_case(r#"{ "blocklist_url": "http://example.com/lists/test.txt" }"# => SessionGet {
            blocklist_url: Some(
                Url::parse("http://example.com/lists/test.txt").expect("valid url")
            ),
            ..Default::default()
        } ; "semver 6.0.0 blocklist url"
    )]
    #[test_case(r#"{ "blocklist_url": "malformed" }"# => panics "relative URL without a base"
        ; "semver 6.0.0 blocklist url malformed"
    )]

    #[test_case(r#"{ "cache_size_mib": 16 }"# => SessionGet {
            cache_size_mb: Some(16),
            ..Default::default()
        } ; "semver 6.0.0 cache size mib"
    )]
    #[test_case(r#"{ "cache_size_mib": 0 }"# => SessionGet {
            cache_size_mb: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 cache size mib zero"
    )]
    #[test_case(r#"{ "cache_size_mib": -1 }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 cache size mib negative"
    )]

    #[test_case(r#"{ "config_dir": "/home/user/config/transmission" }"# => SessionGet {
            config_dir: Some("/home/user/config/transmission".into()),
            ..Default::default()
        } ; "semver 6.0.0 config dir"
    )]

    #[test_case(r#"{
        "default_trackers": "http://one.example.com\n\nhttp://two.example.com\n"
    }"# => SessionGet {
            default_trackers: Some(vec![
                ["http://one.example.com"],
                ["http://two.example.com"],
            ].into()),
            ..Default::default()
        } ; "semver 6.0.0 default trackers"
    )]
    #[test_case(r#"{ "default_trackers": "" }"# => SessionGet {
            default_trackers: Some(TrackerList(vec![])),
            ..Default::default()
        } ; "semver 6.0.0 default trackers empty"
    )]

    #[test_case(r#"{ "dht_enabled": true }"# => SessionGet {
            dht_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 dht enabled"
    )]

    #[test_case(r#"{ "download_dir": "/home/user/downloads" }"# => SessionGet {
            download_dir: Some("/home/user/downloads".into()),
            ..Default::default()
        } ; "semver 6.0.0 download dir"
    )]
    #[test_case(r#"{ "download_dir_free_space": 123456 }"# => SessionGet {
            download_dir_free_space: Some(123456),
            ..Default::default()
        } ; "semver 6.0.0 download dir free space"
    )]
    #[test_case(r#"{ "download_dir_free_space": 0 }"# => SessionGet {
            download_dir_free_space: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 download dir free space zero"
    )]
    #[test_case(r#"{ "download_dir_free_space": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 download dir free space negative"
    )]

    #[test_case(r#"{ "download_queue_enabled": false }"# => SessionGet {
            download_queue_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 download queue enabled"
    )]

    #[test_case(r#"{ "download_queue_size": 42 }"# => SessionGet {
            download_queue_size: Some(42),
            ..Default::default()
        } ; "semver 6.0.0 download queue size"
    )]
    #[test_case(r#"{ "download_queue_size": 0 }"# => SessionGet {
            download_queue_size: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 download queue size zero"
    )]
    #[test_case(r#"{ "download_queue_size": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 download queue size negative"
    )]

    #[test_case(r#"{ "encryption": "allowed" }"# => SessionGet {
            encryption: Some(Encryption::Tolerated),
            ..Default::default()
        } ; "semver 6.0.0 encryption"
    )]

    #[test_case(r#"{ "idle_seeding_limit_enabled": false }"# => SessionGet {
            idle_seeding_limit_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 idle seeding limit enabled"
    )]

    #[test_case(r#"{ "idle_seeding_limit": 123 }"# => SessionGet {
            idle_seeding_limit: Some(123),
            ..Default::default()
        } ; "semver 6.0.0 idle seeding limit"
    )]
    #[test_case(r#"{ "idle_seeding_limit": 0 }"# => SessionGet {
            idle_seeding_limit: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 idle seeding limit zero"
    )]
    #[test_case(r#"{ "idle_seeding_limit": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 idle seeding limit negative"
    )]

    #[test_case(r#"{ "incomplete_dir": "/home/user/incomplete" }"# => SessionGet {
            incomplete_dir: Some("/home/user/incomplete".into()),
            ..Default::default()
        } ; "semver 6.0.0 incomplete dir"
    )]

    #[test_case(r#"{ "lpd_enabled": true }"# => SessionGet {
            lpd_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 lpd enabled"
    )]

    #[test_case(r#"{ "peer_limit_global": 3333 }"# => SessionGet {
            peer_limit_global: Some(3333),
            ..Default::default()
        } ; "semver 6.0.0 peer limit global"
    )]
    #[test_case(r#"{ "peer_limit_global": 0 }"# => SessionGet {
            peer_limit_global: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 peer limit global zero"
    )]
    #[test_case(r#"{ "peer_limit_global": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 peer limit global negative"
    )]

    #[test_case(r#"{ "peer_limit_per_torrent": 3 }"# => SessionGet {
            peer_limit_per_torrent: Some(3),
            ..Default::default()
        } ; "semver 6.0.0 peer limit per torrent"
    )]
    #[test_case(r#"{ "peer_limit_per_torrent": 0 }"# => SessionGet {
            peer_limit_per_torrent: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 peer limit per torrent zero"
    )]
    #[test_case(r#"{ "peer_limit_per_torrent": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 peer limit per torrent negative"
    )]

    #[test_case(r#"{ "peer_port_random_on_start": false }"# => SessionGet {
            peer_port_random_on_start: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 peer port random on start"
    )]
    #[test_case(r#"{ "peer_port": 55555 }"# => SessionGet {
            peer_port: Some(55555),
            ..Default::default()
        } ; "semver 6.0.0 peer port"
    )]

    #[test_case(r#"{ "pex_enabled": true }"# => SessionGet {
            pex_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 pex enabled"
    )]

    #[test_case(r#"{ "port_forwarding_enabled": false }"# => SessionGet {
            port_forwarding_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 port forwarding enabled"
    )]

    #[test_case(r#"{ "preferred_transports": ["tcp", "utp"] }"# => SessionGet {
            preferred_transports: Some(vec![Transport::Tcp, Transport::Utp]),
            ..Default::default()
        } ; "semver 6.0.0 preferred transports"
    )]

    #[test_case(r#"{ "queue_stalled_enabled": true }"# => SessionGet {
            queue_stalled_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 queue stalled enabled"
    )]

    #[test_case(r#"{ "queue_stalled_minutes": 60 }"# => SessionGet {
            queue_stalled_minutes: Some(60),
            ..Default::default()
        } ; "semver 6.0.0 queue stalled minutes"
    )]
    #[test_case(r#"{ "queue_stalled_minutes": 0 }"# => SessionGet {
            queue_stalled_minutes: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 queue stalled minutes zero"
    )]
    #[test_case(r#"{ "queue_stalled_minutes": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 queue stalled minutes negative"
    )]

    #[test_case(r#"{ "rename_partial_files": true }"# => SessionGet {
            rename_partial_files: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 rename partial files"
    )]

    #[test_case(r#"{ "reqq": 2000 }"# => SessionGet {
            reqq: Some(2000),
            ..Default::default()
        } ; "semver 6.0.0 reqq"
    )]
    #[test_case(r#"{ "reqq": 0 }"# => SessionGet {
            reqq: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 reqq zero"
    )]
    #[test_case(r#"{ "reqq": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 reqq negative"
    )]

    #[test_case(r#"{ "rpc_version_minimum": 3 }"# => SessionGet {
            rpc_version_minimum: Some(3),
            ..Default::default()
        } ; "semver 6.0.0 rpc version minimum"
    )]
    #[test_case(r#"{ "rpc_version_minimum": 0 }"# => SessionGet {
            rpc_version_minimum: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 rpc version minimum zero"
    )]
    #[test_case(r#"{ "rpc_version_minimum": -1 }"# => SessionGet {
            rpc_version_minimum: Some(-1),
            ..Default::default()
        } ; "semver 6.0.0 rpc version minimum negative"
    )]

    #[test_case(r#"{ "rpc_version_semver": "5.3.0" }"# => SessionGet {
            rpc_version_semver: Some(Version::parse("5.3.0").expect("valid semver")),
            ..Default::default()
        } ; "semver 6.0.0 rpc version semver"
    )]

    #[test_case(r#"{ "rpc_version": 18 }"# => SessionGet {
            rpc_version: Some(18),
            ..Default::default()
        } ; "semver 6.0.0 rpc version "
    )]
    #[test_case(r#"{ "rpc_version": 0 }"# => SessionGet {
            rpc_version: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 rpc version zero"
    )]
    #[test_case(r#"{ "rpc_version": -1 }"# => SessionGet {
            rpc_version: Some(-1),
            ..Default::default()
        } ; "semver 6.0.0 rpc version negative"
    )]

    #[test_case(r#"{ "script_torrent_added_enabled": false }"# => SessionGet {
            script_torrent_added_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 script torrent added enabled"
    )]
    #[test_case(r#"{ "script_torrent_added_filename": "/scripts/foo.sh" }"# => SessionGet {
            script_torrent_added_filename: Some("/scripts/foo.sh".into()),
            ..Default::default()
        } ; "semver 6.0.0 script torrent added filename"
    )]

    #[test_case(r#"{ "script_torrent_done_enabled": true }"# => SessionGet {
            script_torrent_done_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 script torrent done enabled"
    )]
    #[test_case(r#"{ "script_torrent_done_filename": "/scripts/bar.sh" }"# => SessionGet {
            script_torrent_done_filename: Some("/scripts/bar.sh".into()),
            ..Default::default()
        } ; "semver 6.0.0 script torrent done filename"
    )]

    #[test_case(r#"{ "script_torrent_done_seeding_enabled": true }"# => SessionGet {
            script_torrent_done_seeding_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 script torrent done seeding enabled"
    )]
    #[test_case(r#"{ "script_torrent_done_seeding_filename": "/scripts/baz.sh" }"# => SessionGet {
            script_torrent_done_seeding_filename: Some("/scripts/baz.sh".into()),
            ..Default::default()
        } ; "semver 6.0.0 script torrent done seeding filename"
    )]

    #[test_case(r#"{ "seed_queue_enabled": true }"# => SessionGet {
            seed_queue_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 seed queue enabled"
    )]

    #[test_case(r#"{ "seed_queue_size": 23 }"# => SessionGet {
            seed_queue_size: Some(23),
            ..Default::default()
        } ; "semver 6.0.0 seed queue size"
    )]
    #[test_case(r#"{ "seed_queue_size": 0 }"# => SessionGet {
            seed_queue_size: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 seed queue size zero"
    )]
    #[test_case(r#"{ "seed_queue_size": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 seed queue size negative"
    )]

    #[test_case(r#"{ "seed_ratio_limit": 3.14 }"# => SessionGet {
            seed_ratio_limit: Some(3.14),
            ..Default::default()
        } ; "semver 6.0.0 seed ratio limit"
    )]
    #[test_case(r#"{ "seed_ratio_limit": 0 }"# => SessionGet {
            seed_ratio_limit: Some(0.),
            ..Default::default()
        } ; "semver 6.0.0 seed ratio limit zero"
    )]
    #[test_case(r#"{ "seed_ratio_limit": -1 }"# => SessionGet {
            seed_ratio_limit: Some(-1.),
            ..Default::default()
        } ; "semver 6.0.0 seed ratio limit negative"
    )]

    #[test_case(r#"{ "seed_ratio_limited": true }"# => SessionGet {
            seed_ratio_limited: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 seed ratio limited"
    )]

    #[test_case(r#"{ "sequential_download": true }"# => SessionGet {
            sequential_download: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 sequential download"
    )]

    #[test_case(r#"{
        "session_id": "5C7Jx5rb62lGlJJo6Udy01hjGtlrfg23xaBn3YPkqUWF6uSX"
    }"# => SessionGet {
            session_id: Some("5C7Jx5rb62lGlJJo6Udy01hjGtlrfg23xaBn3YPkqUWF6uSX".into()),
            ..Default::default()
        } ; "semver 6.0.0 session id"
    )]

    #[test_case(r#"{ "speed_limit_down_enabled": true }"# => SessionGet {
            speed_limit_down_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 speed limit down enabled"
    )]

    #[test_case(r#"{ "speed_limit_down": 1111 }"# => SessionGet {
            speed_limit_down: Some(1111),
            ..Default::default()
        } ; "semver 6.0.0 speed limit down"
    )]
    #[test_case(r#"{ "speed_limit_down": 0 }"# => SessionGet {
            speed_limit_down: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 speed limit down zero"
    )]
    #[test_case(r#"{ "speed_limit_down": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 speed limit down negative"
    )]

    #[test_case(r#"{ "speed_limit_up_enabled": true }"# => SessionGet {
            speed_limit_up_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 speed limit up enabled"
    )]

    #[test_case(r#"{ "speed_limit_up": 2222 }"# => SessionGet {
            speed_limit_up: Some(2222),
            ..Default::default()
        } ; "semver 6.0.0 speed limit up"
    )]
    #[test_case(r#"{ "speed_limit_up": 0 }"# => SessionGet {
            speed_limit_up: Some(0),
            ..Default::default()
        } ; "semver 6.0.0 speed limit up zero"
    )]
    #[test_case(r#"{ "speed_limit_up": -1 }"#
        => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 speed limit up negative"
    )]

    #[test_case(r#"{ "start_added_torrents": false }"# => SessionGet {
            start_added_torrents: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 start added torrents"
    )]

    #[test_case(r#"{ "tcp_enabled": false }"# => SessionGet {
            tcp_enabled: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 tcp enabled"
    )]

    #[test_case(r#"{ "trash_original_torrent_files": false }"# => SessionGet {
            trash_original_torrent_files: Some(false),
            ..Default::default()
        } ; "semver 6.0.0 trash original torrent files"
    )]

    #[test_case(r#"{
        "units": {
            "memory_bytes": 1024,
            "memory_units": [
                "B",
                "KiB",
                "MiB",
                "GiB",
                "TiB"
            ],
            "size_bytes": 1000,
            "size_units": [
                "B",
                "kB",
                "MB",
                "GB",
                "TB"
            ],
            "speed_bytes": 1000,
            "speed_units": [
                "B/s",
                "kB/s",
                "MB/s",
                "GB/s",
                "TB/s"
            ]
        }
    }"# => SessionGet {
            units: Some(SessionGetUnits {
                memory_bytes: 1024,
                memory_units: vec![
                    "B".to_string(),
                    "KiB".to_string(),
                    "MiB".to_string(),
                    "GiB".to_string(),
                    "TiB".to_string(),
                ],
                size_bytes: 1000,
                size_units: vec![
                    "B".to_string(),
                    "kB".to_string(),
                    "MB".to_string(),
                    "GB".to_string(),
                    "TB".to_string(),
                ],
                speed_bytes: 1000,
                speed_units: vec![
                    "B/s".to_string(),
                    "kB/s".to_string(),
                    "MB/s".to_string(),
                    "GB/s".to_string(),
                    "TB/s".to_string(),
                ],
            }),
            ..Default::default()
        } ; "semver 6.0.0 units"
    )]

    #[test_case(r#"{ "utp_enabled": true }"# => SessionGet {
            utp_enabled: Some(true),
            ..Default::default()
        } ; "semver 6.0.0 utp enabled"
    )]

    #[test_case(r#"{ "version": "4.1.1 (56442e2929)" }"# => SessionGet {
            version: Some("4.1.1 (56442e2929)".into()),
            ..Default::default()
        } ; "semver 6.0.0 version"
    )]

    fn session_get_deserialize(data: &str) -> SessionGet {
        let formatted = format!("{{\
            \"id\": 0,\
            \"jsonrpc\": \"{JSON_RPC_VERSION_2_0}\",\
            \"result\": {data}\
        }}");
        println!("data>      {data}");
        println!("formatted> {formatted}");
        match serde_json::from_str::<JsonRpcResponse<_>>(&formatted) {
            Ok(resp) => {
                let resp: RpcResponse<_> = resp.into();
                resp.arguments
            },
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod response_deser_tests {
    use crate::{
        JsonRpcResponse,
        types::{Result, RpcResponse},
    };
    use super::*;

    #[test]
    fn session_get_v300() -> Result<()> {
        let resp = serde_json::from_str::<RpcResponse<SessionGet>>(
            r#"
            {
              "arguments": {
                "alt-speed-down": 500,
                "alt-speed-enabled": false,
                "alt-speed-time-begin": 540,
                "alt-speed-time-day": 0,
                "alt-speed-time-enabled": false,
                "alt-speed-time-end": 1020,
                "alt-speed-up": 500,
                "blocklist-enabled": false,
                "blocklist-size": 0,
                "blocklist-url": "http://www.example.com/blocklist",
                "cache-size-mb": 16,
                "config-dir": "/config",
                "dht-enabled": false,
                "download-dir": "/downloads",
                "download-dir-free-space": 123456789,
                "download-queue-enabled": true,
                "download-queue-size": 1,
                "encryption": "tolerated",
                "idle-seeding-limit": 30,
                "idle-seeding-limit-enabled": false,
                "incomplete-dir": "/incomplete",
                "incomplete-dir-enabled": false,
                "lpd-enabled": false,
                "peer-limit-global": 100,
                "peer-limit-per-torrent": 20,
                "peer-port": 55555,
                "peer-port-random-on-start": false,
                "pex-enabled": false,
                "port-forwarding-enabled": false,
                "queue-stalled-enabled": true,
                "queue-stalled-minutes": 30,
                "rename-partial-files": false,
                "rpc-version": 16,
                "rpc-version-minimum": 1,
                "script-torrent-done-enabled": true,
                "script-torrent-done-filename": "/usr/bin/test",
                "seed-queue-enabled": false,
                "seed-queue-size": 500,
                "seedRatioLimit": 2,
                "seedRatioLimited": false,
                "session-id": "w2oQxnkhWlut8omPLCDxgGR3g0pDX7gEan4Xz4QHqUlnlBEu",
                "speed-limit-down": 1000,
                "speed-limit-down-enabled": true,
                "speed-limit-up": 2000,
                "speed-limit-up-enabled": true,
                "start-added-torrents": false,
                "trash-original-torrent-files": true,
                "units": {
                  "memory-bytes": 1024,
                  "memory-units": [
                    "KiB",
                    "MiB",
                    "GiB",
                    "TiB"
                  ],
                  "size-bytes": 1000,
                  "size-units": [
                    "kB",
                    "MB",
                    "GB",
                    "TB"
                  ],
                  "speed-bytes": 1000,
                  "speed-units": [
                    "kB/s",
                    "MB/s",
                    "GB/s",
                    "TB/s"
                  ]
                },
                "utp-enabled": true,
                "version": "3.00 (bb6b5a062e)"
              },
              "result": "success"
            }
            "#,
        )?;

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.alt_speed_down, Some(500));
        assert_eq!(resp.arguments.alt_speed_enabled, Some(false));
        assert_eq!(resp.arguments.alt_speed_time_begin, Some(540));
        assert_eq!(resp.arguments.alt_speed_time_day, Some(AltSpeedDay::empty()));
        assert_eq!(resp.arguments.alt_speed_time_enabled, Some(false));
        assert_eq!(resp.arguments.alt_speed_time_end, Some(1020));
        assert_eq!(resp.arguments.alt_speed_up, Some(500));

        assert_eq!(resp.arguments.anti_brute_force_enabled, None);
        assert_eq!(resp.arguments.anti_brute_force_threshold, None);

        assert_eq!(resp.arguments.blocklist_enabled, Some(false));
        assert_eq!(resp.arguments.blocklist_size, Some(0));
        assert_eq!(
            resp.arguments.blocklist_url,
            Some(Url::parse("http://www.example.com/blocklist")?)
        );

        assert_eq!(resp.arguments.cache_size_mb, Some(16));
        assert_eq!(resp.arguments.config_dir, Some("/config".to_string()));
        assert_eq!(resp.arguments.default_trackers, None);
        assert_eq!(resp.arguments.dht_enabled, Some(false));

        assert_eq!(resp.arguments.download_dir, Some("/downloads".to_string()));
        assert_eq!(resp.arguments.download_dir_free_space, Some(123456789));
        assert_eq!(resp.arguments.download_queue_enabled, Some(true));
        assert_eq!(resp.arguments.download_queue_size, Some(1));

        assert_eq!(resp.arguments.encryption, Some(Encryption::Tolerated));
        assert_eq!(resp.arguments.idle_seeding_limit_enabled, Some(false));
        assert_eq!(resp.arguments.idle_seeding_limit, Some(30));

        assert_eq!(resp.arguments.incomplete_dir_enabled, Some(false));
        assert_eq!(resp.arguments.incomplete_dir, Some("/incomplete".to_string()));

        assert_eq!(resp.arguments.lpd_enabled, Some(false));
        assert_eq!(resp.arguments.peer_limit_global, Some(100));
        assert_eq!(resp.arguments.peer_limit_per_torrent, Some(20));
        assert_eq!(resp.arguments.peer_port_random_on_start, Some(false));
        assert_eq!(resp.arguments.peer_port, Some(55555));
        assert_eq!(resp.arguments.pex_enabled, Some(false));
        assert_eq!(resp.arguments.port_forwarding_enabled, Some(false));
        assert_eq!(resp.arguments.preferred_transports, None);
        assert_eq!(resp.arguments.queue_stalled_enabled, Some(true));
        assert_eq!(resp.arguments.queue_stalled_minutes, Some(30));
        assert_eq!(resp.arguments.rename_partial_files, Some(false));
        assert_eq!(resp.arguments.reqq, None);

        assert_eq!(resp.arguments.rpc_version_minimum, Some(1));
        assert_eq!(resp.arguments.rpc_version_semver, None);
        assert_eq!(resp.arguments.rpc_version, Some(16));

        assert_eq!(resp.arguments.script_torrent_added_enabled, None);
        assert_eq!(resp.arguments.script_torrent_added_filename, None);
        assert_eq!(resp.arguments.script_torrent_done_enabled, Some(true));
        assert_eq!(resp.arguments.script_torrent_done_filename, Some("/usr/bin/test".to_string()));
        assert_eq!(resp.arguments.script_torrent_done_seeding_enabled, None);
        assert_eq!(resp.arguments.script_torrent_done_seeding_filename, None);

        assert_eq!(resp.arguments.seed_queue_enabled, Some(false));
        assert_eq!(resp.arguments.seed_queue_size, Some(500));
        assert_eq!(resp.arguments.seed_ratio_limit, Some(2.0));
        assert_eq!(resp.arguments.seed_ratio_limited, Some(false));

        assert_eq!(resp.arguments.sequential_download, None);
        let session_id = "w2oQxnkhWlut8omPLCDxgGR3g0pDX7gEan4Xz4QHqUlnlBEu".to_string();
        assert_eq!(resp.arguments.session_id, Some(session_id));

        assert_eq!(resp.arguments.speed_limit_down_enabled, Some(true));
        assert_eq!(resp.arguments.speed_limit_down, Some(1000));
        assert_eq!(resp.arguments.speed_limit_up_enabled, Some(true));
        assert_eq!(resp.arguments.speed_limit_up, Some(2000));

        assert_eq!(resp.arguments.start_added_torrents, Some(false));
        assert_eq!(resp.arguments.tcp_enabled, None);
        assert_eq!(resp.arguments.trash_original_torrent_files, Some(true));

        let units = SessionGetUnits {
            memory_bytes: 1024,
            memory_units: ["KiB", "MiB", "GiB", "TiB"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            size_bytes: 1000,
            size_units: ["kB", "MB", "GB", "TB"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            speed_bytes: 1000,
            speed_units: ["kB/s", "MB/s", "GB/s", "TB/s"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        };
        assert_eq!(resp.arguments.units, Some(units));

        assert_eq!(resp.arguments.utp_enabled, Some(true));
        assert_eq!(resp.arguments.version, Some("3.00 (bb6b5a062e)".to_string()));

        Ok(())
    }

    // ---------------------------------------------------------------------

    #[test]
    fn session_get_v411() -> Result<()> {
        let resp: RpcResponse<_> = serde_json::from_str::<JsonRpcResponse<SessionGet>>(
            r#"{
              "id": 12345,
              "jsonrpc": "2.0",
              "result": {
                "alt_speed_down": 500,
                "alt_speed_enabled": false,
                "alt_speed_time_begin": 540,
                "alt_speed_time_day": 0,
                "alt_speed_time_enabled": false,
                "alt_speed_time_end": 1020,
                "alt_speed_up": 500,
                "anti_brute_force_enabled": false,
                "anti_brute_force_threshold": 101,
                "blocklist_enabled": false,
                "blocklist_size": 0,
                "blocklist_url": "http://www.example.com/blocklist",
                "cache_size_mib": 16,
                "config_dir": "/config",
                "default_trackers": "",
                "dht_enabled": false,
                "download_dir": "/downloads",
                "download_dir_free_space": 123456789,
                "download_queue_enabled": true,
                "download_queue_size": 2,
                "encryption": "allowed",
                "idle_seeding_limit": 30,
                "idle_seeding_limit_enabled": false,
                "incomplete_dir": "/incomplete",
                "incomplete_dir_enabled": false,
                "lpd_enabled": false,
                "peer_limit_global": 110,
                "peer_limit_per_torrent": 10,
                "peer_port": 55555,
                "peer_port_random_on_start": false,
                "pex_enabled": false,
                "port_forwarding_enabled": false,
                "preferred_transports": [
                  "utp",
                  "tcp"
                ],
                "queue_stalled_enabled": true,
                "queue_stalled_minutes": 30,
                "rename_partial_files": false,
                "reqq": 2000,
                "rpc_version": 19,
                "rpc_version_minimum": 14,
                "rpc_version_semver": "6.0.1",
                "script_torrent_added_enabled": false,
                "script_torrent_added_filename": "",
                "script_torrent_done_enabled": true,
                "script_torrent_done_filename": "/usr/bin/test",
                "script_torrent_done_seeding_enabled": false,
                "script_torrent_done_seeding_filename": "",
                "seed_queue_enabled": false,
                "seed_queue_size": 500,
                "seed_ratio_limit": 2.0,
                "seed_ratio_limited": false,
                "sequential_download": false,
                "session_id": "5C7Jx5rb62lGlJJo6Udy01hjGtlrfg23xaBn3YPkqUWF6uSX",
                "speed_limit_down": 1234,
                "speed_limit_down_enabled": true,
                "speed_limit_up": 4321,
                "speed_limit_up_enabled": true,
                "start_added_torrents": false,
                "tcp_enabled": true,
                "trash_original_torrent_files": true,
                "units": {
                  "memory_bytes": 1024,
                  "memory_units": [
                    "B",
                    "KiB",
                    "MiB",
                    "GiB",
                    "TiB"
                  ],
                  "size_bytes": 1000,
                  "size_units": [
                    "B",
                    "kB",
                    "MB",
                    "GB",
                    "TB"
                  ],
                  "speed_bytes": 1000,
                  "speed_units": [
                    "B/s",
                    "kB/s",
                    "MB/s",
                    "GB/s",
                    "TB/s"
                  ]
                },
                "utp_enabled": true,
                "version": "4.1.1 (56442e2929)"
              }
            }
            "#
        )?
        .into();

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.alt_speed_down, Some(500));
        assert_eq!(resp.arguments.alt_speed_enabled, Some(false));
        assert_eq!(resp.arguments.alt_speed_time_begin, Some(540));
        assert_eq!(resp.arguments.alt_speed_time_day, Some(AltSpeedDay::empty()));
        assert_eq!(resp.arguments.alt_speed_time_enabled, Some(false));
        assert_eq!(resp.arguments.alt_speed_time_end, Some(1020));
        assert_eq!(resp.arguments.alt_speed_up, Some(500));

        assert_eq!(resp.arguments.anti_brute_force_enabled, Some(false));
        assert_eq!(resp.arguments.anti_brute_force_threshold, Some(101));

        assert_eq!(resp.arguments.blocklist_enabled, Some(false));
        assert_eq!(resp.arguments.blocklist_size, Some(0));
        assert_eq!(
            resp.arguments.blocklist_url,
            Some(Url::parse("http://www.example.com/blocklist")?)
        );

        assert_eq!(resp.arguments.cache_size_mb, Some(16));
        assert_eq!(resp.arguments.config_dir, Some("/config".to_string()));
        assert_eq!(
            resp.arguments.default_trackers,
            Some(vec![[""]].into())
        );
        assert_eq!(resp.arguments.dht_enabled, Some(false));

        assert_eq!(resp.arguments.download_dir, Some("/downloads".to_string()));
        assert_eq!(resp.arguments.download_dir_free_space, Some(123456789));
        assert_eq!(resp.arguments.download_queue_enabled, Some(true));
        assert_eq!(resp.arguments.download_queue_size, Some(2));

        assert_eq!(resp.arguments.encryption, Some(Encryption::Tolerated));
        assert_eq!(resp.arguments.idle_seeding_limit_enabled, Some(false));
        assert_eq!(resp.arguments.idle_seeding_limit, Some(30));

        assert_eq!(resp.arguments.incomplete_dir_enabled, Some(false));
        assert_eq!(resp.arguments.incomplete_dir, Some("/incomplete".to_string()));

        assert_eq!(resp.arguments.lpd_enabled, Some(false));
        assert_eq!(resp.arguments.peer_limit_global, Some(110));
        assert_eq!(resp.arguments.peer_limit_per_torrent, Some(10));
        assert_eq!(resp.arguments.peer_port_random_on_start, Some(false));
        assert_eq!(resp.arguments.peer_port, Some(55555));
        assert_eq!(resp.arguments.pex_enabled, Some(false));
        assert_eq!(resp.arguments.port_forwarding_enabled, Some(false));
        let transports = vec![Transport::Utp, Transport::Tcp];
        assert_eq!(resp.arguments.preferred_transports, Some(transports));
        assert_eq!(resp.arguments.queue_stalled_enabled, Some(true));
        assert_eq!(resp.arguments.queue_stalled_minutes, Some(30));
        assert_eq!(resp.arguments.rename_partial_files, Some(false));
        assert_eq!(resp.arguments.reqq, Some(2000));

        assert_eq!(resp.arguments.rpc_version_minimum, Some(14));
        assert_eq!(
            resp.arguments.rpc_version_semver,
            Some(Version::parse("6.0.1")?)
        );
        assert_eq!(resp.arguments.rpc_version, Some(19));

        assert_eq!(resp.arguments.script_torrent_added_enabled, Some(false));
        assert_eq!(resp.arguments.script_torrent_added_filename, Some("".to_string()));
        assert_eq!(resp.arguments.script_torrent_done_enabled, Some(true));
        assert_eq!(resp.arguments.script_torrent_done_filename, Some("/usr/bin/test".to_string()));
        assert_eq!(resp.arguments.script_torrent_done_seeding_enabled, Some(false));
        assert_eq!(resp.arguments.script_torrent_done_seeding_filename, Some("".to_string()));

        assert_eq!(resp.arguments.seed_queue_enabled, Some(false));
        assert_eq!(resp.arguments.seed_queue_size, Some(500));
        assert_eq!(resp.arguments.seed_ratio_limit, Some(2.0));
        assert_eq!(resp.arguments.seed_ratio_limited, Some(false));

        assert_eq!(resp.arguments.sequential_download, Some(false));
        let session_id = "5C7Jx5rb62lGlJJo6Udy01hjGtlrfg23xaBn3YPkqUWF6uSX".to_string();
        assert_eq!(resp.arguments.session_id, Some(session_id));

        assert_eq!(resp.arguments.speed_limit_down, Some(1234));
        assert_eq!(resp.arguments.speed_limit_down_enabled, Some(true));
        assert_eq!(resp.arguments.speed_limit_up, Some(4321));
        assert_eq!(resp.arguments.speed_limit_up_enabled, Some(true));

        assert_eq!(resp.arguments.start_added_torrents, Some(false));
        assert_eq!(resp.arguments.tcp_enabled, Some(true));
        assert_eq!(resp.arguments.trash_original_torrent_files, Some(true));

        let units = SessionGetUnits {
            memory_bytes: 1024,
            memory_units: ["B", "KiB", "MiB", "GiB", "TiB"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            size_bytes: 1000,
            size_units: ["B", "kB", "MB", "GB", "TB"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            speed_bytes: 1000,
            speed_units: ["B/s", "kB/s", "MB/s", "GB/s", "TB/s"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        };
        assert_eq!(resp.arguments.units, Some(units));

        assert_eq!(resp.arguments.utp_enabled, Some(true));
        assert_eq!(resp.arguments.version, Some("4.1.1 (56442e2929)".to_string()));

        Ok(())
    }
}
