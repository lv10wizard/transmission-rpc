use compat_macros::GenerateCompat;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::{AltSpeedDay, Encryption, EncryptionCompat, MinutesAfterMidnight, Transport};

#[skip_serializing_none]
#[derive(GenerateCompat, Serialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionSetArgs {
    /// Max global download speed (kB/s).
    pub alt_speed_down: Option<u64>,
    /// True means use the alt speeds.
    pub alt_speed_enabled: Option<bool>,
    /// When to turn on alt speeds (units: minutes after midnight).
    pub alt_speed_time_begin: Option<MinutesAfterMidnight>,
    /// What day(s) to turn on alt speeds.
    pub alt_speed_time_day: Option<AltSpeedDay>,
    /// True means the scheduled on/off times are used.
    pub alt_speed_time_enabled: Option<bool>,
    /// When to turn off alt speeds (units: minutes after midnight).
    pub alt_speed_time_end: Option<MinutesAfterMidnight>,
    /// Max global upload speed (kB/s).
    pub alt_speed_up: Option<u64>,

    /// Enable a very basic brute force protection for the RPC server. See
    /// [`anti_brute_force_threshold`] below.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [`anti_brute_force_threshold`]: Self::anti_brute_force_threshold
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub anti_brute_force_enabled: Option<bool>,

    /// After this amount of failed authentication attempts is surpassed, the RPC server will deny
    /// any further authentication attempts until it is restarted. This is not tracked per IP but
    /// in total.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub anti_brute_force_threshold: Option<u64>,

    /// True means block peers based on [`blocklist_url`]. See also: [blocklists.md].
    ///
    /// [`blocklist_url`]: Self::blocklist_url
    /// [blocklists.md]:
    /// <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    pub blocklist_enabled: Option<bool>,
    /// Location of the blocklist to use. See: [blocklists.md].
    ///
    /// [blocklists.md]:
    /// <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    pub blocklist_url: Option<String>,

    /// Number in MiB to allocate for Transmission's memory cache. The cache is used to help batch
    /// disk IO together, so increasing the cache size can be used to reduce the number of disk
    /// reads and writes. The value is the total available to the Transmission instance. Set it to
    /// the smallest value tolerable by the random access performance of your storage medium to
    /// minimize data loss in case Transmission quit unexpectedly. Setting this to 0 bypasses the
    /// cache, which may be useful if your filesystem already has a cache layer that aggregates
    /// transactions. Pieces are guaranteed to be written to filesystem if sequential download is
    /// enabled. Otherwise, data might still be in cache only.
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?):
    /// The memory cache is being removed, making this setting moot. The setting will still be
    /// gettable and settable via RPC `session_get` and `session_set` until Transmission 5.0.0 to
    /// avoid client breakage, but it will be otherwise unused in libtransmission. Clients should
    /// stop using this key.
    #[compat(name = cache_size_mib)]
    pub cache_size_mb: Option<i32>,

    /// Announce URLs, one per line, and a blank line between [tiers].
    ///
    /// eg. `"http://bt1.archive.org:6969/announce\n\nhttp://bt2.archive.org:6969/announce\n"`
    /// 
    /// [tiers]: <https://www.bittorrent.org/beps/bep_0012.html>
    pub default_trackers: Option<String>,
    /// True means allow [Distrubted Hash Table] in public torrents.
    ///
    /// [Distrubted Hash Table]: <https://wikipedia.org/wiki/Distributed_hash_table>
    pub dht_enabled: Option<bool>,
    /// Default path to download torrents.
    pub download_dir: Option<String>,
    /// If true, limit how many torrents can be downloaded at once.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    pub download_queue_enabled: Option<bool>,
    /// Max number of torrents to download at once (see [`download_queue_enabled`])
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`download_queue_enabled`]: Self::download_queue_enabled
    pub download_queue_size: Option<u64>,
    /// Encryption preference. Encryption may help get around some ISP filtering, but at the cost
    /// of slightly higher CPU use.
    #[compat(type = Option<EncryptionCompat>, map = Option::map)]
    pub encryption: Option<Encryption>,
    /// Torrents we're seeding will be stopped if they're idle for this long.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    pub idle_seeding_limit: Option<u64>,
    /// True if the [seeding inactivity limit] is honored by default.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// [seeding inactivity limit]: Self::idle_seeding_limit
    pub idle_seeding_limit_enabled: Option<bool>,
    /// Path for incomplete torrents, when enabled.
    pub incomplete_dir: Option<String>,
    /// True means keep torrents in [`incomplete_dir`] until done.
    ///
    /// [`incomplete_dir`]: Self::incomplete_dir
    pub incomplete_dir_enabled: Option<bool>,
    /// True means allow [Local Peer Discovery] in public torrents.
    ///
    /// [Local Peer Discovery]: <https://en.wikipedia.org/wiki/Local_Peer_Discovery>
    pub lpd_enabled: Option<bool>,
    /// Maximum global number of peers.
    pub peer_limit_global: Option<u64>,
    /// Maximum number of peers per torrent.
    pub peer_limit_per_torrent: Option<u64>,
    /// True means pick a random peer port on launch.
    pub peer_port_random_on_start: Option<bool>,
    /// The daemon's port number.
    pub peer_port: Option<u16>,
    /// True means allow [Peer Exchange] in public torrents.
    ///
    /// [Peer Exchange]: <https://wikipedia.org/wiki/Peer_exchange>
    pub pex_enabled: Option<bool>,
    /// True means ask upstream router to forward the configured peer port to transmission using
    /// [UPnP] or [NAT-PMP].
    ///
    /// [UPnP]: <https://en.wikipedia.org/wiki/Universal_Plug_and_Play>
    /// [NAT-PMP]: <https://en.wikipedia.org/wiki/NAT_Port_Mapping_Protocol>
    pub port_forwarding_enabled: Option<bool>,

    /// List your preference of transport protocols in the order of preferred-first. Omitting the
    /// transport protocol from the list will disable it. *Note: Never disable TCP when you also
    /// disable µTP, because then your client would not be able to communicate. Disabling TCP might
    /// also break webseeds.*
    ///
    ///  > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub preferred_transports: Option<Vec<Transport>>,

    /// Whether or not to consider [idle torrents as stalled].
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [idle torrents as stalled]: Self::queue_stalled_minutes
    pub queue_stalled_enabled: Option<bool>,
    /// Torrents that are idle for N minuets aren't counted toward [`seed_queue_size`] or
    /// [`download_queue_size`].
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_size`]: Self::seed_queue_size
    /// [`download_queue_size`]: Self::download_queue_size
    pub queue_stalled_minutes: Option<u64>,
    /// True means append `.part` to incomplete files.
    ///
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    pub rename_partial_files: Option<bool>,
    /// The number of outstanding block requests a peer is allowed to queue in the client. The
    /// higher this number, the higher the max possible upload speed towards each peer.
    ///
    /// > (?) Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18) [in
    /// [transmission:62240393e]]
    ///
    /// [transmission:62240393e]:
    /// <https://github.com/transmission/transmission/commit/62240393ed056099a6a2ee60d778ac19928ef451>
    pub reqq: Option<u64>,
    /// Run a script when a torrent is added to Transmission. See: [scripts.md].
    ///
    /// [scripts.md]: <https://github.com/transmission/transmission/blob/main/docs/Scripts.md>
    pub script_torrent_added_enabled: Option<bool>,
    /// Path to script.
    pub script_torrent_added_filename: Option<String>,
    /// Run a script when a torrent is done downloading. See: [scripts.md].
    ///
    /// [scripts.md]: <https://github.com/transmission/transmission/blob/main/docs/Scripts.md>
    pub script_torrent_done_enabled: Option<bool>,
    /// Path to script.
    pub script_torrent_done_filename: Option<String>,
    /// Run a script when a torrent is done seeding. See: [scripts.md].
    ///
    /// > (?) Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17) [in
    /// [transmission:9f9b6cdaa]]
    ///
    /// [scripts.md]: <https://github.com/transmission/transmission/blob/main/docs/Scripts.md>
    /// [transmission:9f9b6cdaa]:
    /// <https://github.com/transmission/transmission/commit/9f9b6cdaa2e02727ee62b0f63a34d297e2246650>
    pub script_torrent_done_seeding_enabled: Option<bool>,
    /// Path to script.
    ///
    /// > (?) Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17) [in
    /// [transmission:9f9b6cdaa]]
    ///
    /// [transmission:9f9b6cdaa]:
    /// <https://github.com/transmission/transmission/commit/9f9b6cdaa2e02727ee62b0f63a34d297e2246650>
    pub script_torrent_done_seeding_filename: Option<String>,
    /// When true, Transmission will only seed [`seed_queue_size`] non-stalled torrents at once.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_size`]: Self::seed_queue_size
    pub seed_queue_enabled: Option<bool>,
    /// Max number of torrents to uploaded at once (see [`seed_queue_enabled`]).
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_enabled`]: Self::seed_queue_enabled
    pub seed_queue_size: Option<u64>,
    /// The default seed ratio for torrents to use.
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    /// True if [`seed_ratio_limit`] is honored by default.
    ///
    /// [`seed_ratio_limit`]: Self::seed_ratio_limit
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,

    /// True means sequential download is enabled by default for added torrents.
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download: Option<bool>,
    /// Download from a specific piece when [sequential download] is enabled.
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    ///
    /// [sequential download]: Self::sequential_download
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download_from_piece: Option<u64>,

    /// Max global download speed (kB/s).
    pub speed_limit_down: Option<u64>,
    /// Whether [`speed_limit_down`] is respected.
    ///
    /// [`speed_limit_down`]: Self::speed_limit_down
    pub speed_limit_down_enabled: Option<bool>,
    /// Max global upload speed (kB/s).
    pub speed_limit_up: Option<i32>,
    /// Whether [`speed_limit_up`] is respected.
    ///
    /// [`speed_limit_up`]: Self::speed_limit_up
    pub speed_limit_up_enabled: Option<bool>,
    /// Start torrents as soon as they are added.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    pub start_added_torrents: Option<bool>,
    /// Delete torrents added from the watch directory.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    pub trash_original_torrent_files: Option<bool>,
    /// True means allow [uTP].
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use [`preferred_transports`] instead.
    ///
    /// [uTP]: <https://wikipedia.org/wiki/Micro_Transport_Protocol>
    /// [`preferred_transports`]: Self::preferred_transports
    pub utp_enabled: Option<bool>,
}

#[cfg(test)]
mod serde_tests {
    use crate::types::{
        JSON_RPC_VERSION_2_0, MinutesAfterMidnight, Result, Transport,
        request::{*, test_helper::verify},
    };
    use super::*;

    #[test]
    fn request_session_set_legacy_alt_speed_down() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_down: Some(321),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-down\":321")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_down() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_down: Some(321),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_down\":321")
    }

    #[test]
    fn request_session_set_legacy_alt_speed_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_alt_speed_time_begin() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_begin: Some(MinutesAfterMidnight(123)),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-time-begin\":123")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_time_begin() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_begin: Some(MinutesAfterMidnight(123)),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_begin\":123")
    }

    #[test]
    fn request_session_set_alt_speed_time_begin_clamp() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_begin: Some(MinutesAfterMidnight(60 * 24 + 111)),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_begin\":111")
    }

    #[test]
    fn request_session_set_legacy_alt_speed_time_day() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_day: Some({
                AltSpeedDay::MONDAY | AltSpeedDay::TUESDAY | AltSpeedDay::WEDNESDAY
                    | AltSpeedDay::THURSDAY | AltSpeedDay::FRIDAY
            }),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-time-day\":62")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_time_day() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_day: Some({
                AltSpeedDay::MONDAY | AltSpeedDay::TUESDAY | AltSpeedDay::WEDNESDAY
                    | AltSpeedDay::THURSDAY | AltSpeedDay::FRIDAY
            }),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_day\":62")
    }

    #[test]
    fn request_session_set_legacy_alt_speed_time_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-time-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_time_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_alt_speed_time_end() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_end: Some(MinutesAfterMidnight(666)),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-time-end\":666")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_time_end() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_end: Some(MinutesAfterMidnight(666)),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_end\":666")
    }

    #[test]
    fn request_session_set_alt_speed_time_end_clamp() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_time_begin: Some(MinutesAfterMidnight(60 * 24 + 222)),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_begin\":222")
    }

    #[test]
    fn request_session_set_legacy_alt_speed_up() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_up: Some(250),
            ..Default::default()
        };
        verify(session_set_args, None, "\"alt-speed-up\":250")
    }

    #[test]
    fn request_session_set_semver_600_alt_speed_up() -> Result<()> {
        let session_set_args = SessionSetArgs {
            alt_speed_up: Some(250),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_up\":250")
    }

    #[test]
    fn request_session_set_legacy_anti_brute_force_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            anti_brute_force_enabled: Some(false),
            ..Default::default()
        };
        // `anti_brute_force_enabled` only exists post- semver-6.0.0.
        verify(session_set_args, None, "")
    }

    #[test]
    fn request_session_set_semver_600_anti_brute_force_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            anti_brute_force_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"anti_brute_force_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_anti_brute_force_threshold() -> Result<()> {
        let session_set_args = SessionSetArgs {
            anti_brute_force_threshold: Some(101),
            ..Default::default()
        };
        // `anti_brute_force_threshold` only exists post- semver-6.0.0.
        verify(session_set_args, None, "")
    }

    #[test]
    fn request_session_set_semver_600_anti_brute_force_threshold() -> Result<()> {
        let session_set_args = SessionSetArgs {
            anti_brute_force_threshold: Some(101),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"anti_brute_force_threshold\":101")
    }

    #[test]
    fn request_session_set_legacy_blocklist_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            blocklist_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"blocklist-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_blocklist_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            blocklist_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"blocklist_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_blocklist_url() -> Result<()> {
        let session_set_args = SessionSetArgs {
            blocklist_url: Some("https://example.com".to_string()),
            ..Default::default()
        };
        verify(session_set_args, None, "\"blocklist-url\":\"https://example.com\"")
    }

    #[test]
    fn request_session_set_semver_600_blocklist_url() -> Result<()> {
        let session_set_args = SessionSetArgs {
            blocklist_url: Some("https://example.com".to_string()),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"blocklist_url\":\"https://example.com\"")
    }

    #[test]
    fn request_session_set_legacy_cache_size_mb() -> Result<()> {
        let session_set_args = SessionSetArgs {
            cache_size_mb: Some(8),
            ..Default::default()
        };
        verify(session_set_args, None, "\"cache-size-mb\":8")
    }

    #[test]
    fn request_session_set_semver_600_cache_size_mb() -> Result<()> {
        let session_set_args = SessionSetArgs {
            cache_size_mb: Some(8),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"cache_size_mib\":8")
    }

    #[test]
    fn request_session_set_legacy_default_trackers() -> Result<()> {
        let session_set_args = SessionSetArgs {
            default_trackers: Some("https://example.com:5555".to_string()),
            ..Default::default()
        };
        verify(session_set_args, None, "\"default-trackers\":\"https://example.com:5555\"")
    }

    #[test]
    fn request_session_set_semver_600_default_trackers() -> Result<()> {
        let session_set_args = SessionSetArgs {
            default_trackers: Some("https://example.com:5555".to_string()),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"default_trackers\":\"https://example.com:5555\"")
    }

    #[test]
    fn request_session_set_legacy_dht_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            dht_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"dht-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_dht_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            dht_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"dht_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_download_dir() -> Result<()> {
        let session_set_args = SessionSetArgs {
            download_dir: Some("/downloads".to_string()),
            ..Default::default()
        };
        verify(session_set_args, None, "\"download-dir\":\"/downloads\"")
    }

    #[test]
    fn request_session_set_semver_600_download_dir() -> Result<()> {
        let session_set_args = SessionSetArgs {
            download_dir: Some("/downloads".to_string()),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"download_dir\":\"/downloads\"")
    }

    #[test]
    fn request_session_set_legacy_download_queue_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            download_queue_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"download-queue-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_download_queue_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            download_queue_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"download_queue_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_download_queue_size() -> Result<()> {
        let session_set_args = SessionSetArgs {
            download_queue_size: Some(1),
            ..Default::default()
        };
        verify(session_set_args, None, "\"download-queue-size\":1")
    }

    #[test]
    fn request_session_set_semver_600_download_queue_size() -> Result<()> {
        let session_set_args = SessionSetArgs {
            download_queue_size: Some(1),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"download_queue_size\":1")
    }

    #[test]
    fn request_session_set_legacy_encryption() -> Result<()> {
        let session_set_args = SessionSetArgs {
            encryption: Some(Encryption::Tolerated),
            ..Default::default()
        };
        verify(session_set_args, None, "\"encryption\":\"tolerated\"")
    }

    #[test]
    fn request_session_set_semver_600_encryption() -> Result<()> {
        let session_set_args = SessionSetArgs {
            encryption: Some(Encryption::Tolerated),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"encryption\":\"allowed\"")
    }

    #[test]
    fn request_session_set_legacy_idle_seeding_limit_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            idle_seeding_limit_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"idle-seeding-limit-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_idle_seeding_limit_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            idle_seeding_limit_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
            "\"idle_seeding_limit_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_idle_seeding_limit() -> Result<()> {
        let session_set_args = SessionSetArgs {
            idle_seeding_limit: Some(4320),
            ..Default::default()
        };
        verify(session_set_args, None, "\"idle-seeding-limit\":4320")
    }

    #[test]
    fn request_session_set_semver_600_idle_seeding_limit() -> Result<()> {
        let session_set_args = SessionSetArgs {
            idle_seeding_limit: Some(4320),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"idle_seeding_limit\":4320")
    }

    #[test]
    fn request_session_set_legacy_incomplete_dir_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            incomplete_dir_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"incomplete-dir-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_incomplete_dir_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            incomplete_dir_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"incomplete_dir_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_incomplete_dir() -> Result<()> {
        let session_set_args = SessionSetArgs {
            incomplete_dir: Some("/incomplete".to_string()),
            ..Default::default()
        };
        verify(session_set_args, None, "\"incomplete-dir\":\"/incomplete\"")
    }

    #[test]
    fn request_session_set_semver_600_incomplete_dir() -> Result<()> {
        let session_set_args = SessionSetArgs {
            incomplete_dir: Some("/incomplete".to_string()),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"incomplete_dir\":\"/incomplete\"")
    }

    #[test]
    fn request_session_set_legacy_lpd_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            lpd_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"lpd-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_lpd_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            lpd_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"lpd_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_peer_limit_global() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_limit_global: Some(120),
            ..Default::default()
        };
        verify(session_set_args, None, "\"peer-limit-global\":120")
    }

    #[test]
    fn request_session_set_semver_600_peer_limit_global() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_limit_global: Some(120),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_limit_global\":120")
    }

    #[test]
    fn request_session_set_legacy_peer_limit_per_torrent() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_limit_per_torrent: Some(10),
            ..Default::default()
        };
        verify(session_set_args, None, "\"peer-limit-per-torrent\":10")
    }

    #[test]
    fn request_session_set_semver_600_peer_limit_per_torrent() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_limit_per_torrent: Some(10),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_limit_per_torrent\":10")
    }

    #[test]
    fn request_session_set_legacy_peer_port_random_on_start() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_port_random_on_start: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"peer-port-random-on-start\":false")
    }

    #[test]
    fn request_session_set_semver_600_peer_port_random_on_start() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_port_random_on_start: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_port_random_on_start\":false")
    }

    #[test]
    fn request_session_set_legacy_peer_port() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_port: Some(44556),
            ..Default::default()
        };
        verify(session_set_args, None, "\"peer-port\":44556")
    }

    #[test]
    fn request_session_set_semver_600_peer_port() -> Result<()> {
        let session_set_args = SessionSetArgs {
            peer_port: Some(44556),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_port\":44556")
    }

    #[test]
    fn request_session_set_legacy_pex_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            pex_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"pex-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_pex_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            pex_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"pex_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_port_forwarding_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            port_forwarding_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"port-forwarding-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_port_forwarding_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            port_forwarding_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"port_forwarding_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_preferred_transports() -> Result<()> {
        let session_set_args = SessionSetArgs {
            preferred_transports: Some(vec![Transport::UTP, Transport::TCP]),
            ..Default::default()
        };
        // `preferred_transports` only exists post- semver-6.0.0.
        verify(session_set_args, None, "")
    }

    #[test]
    fn request_session_set_semver_600_preferred_transports() -> Result<()> {
        let session_set_args = SessionSetArgs {
            preferred_transports: Some(vec![Transport::UTP, Transport::TCP]),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"preferred_transports\":[\"utp\",\"tcp\"]")
    }

    #[test]
    fn request_session_set_legacy_queue_stalled_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            queue_stalled_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"queue-stalled-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_queue_stalled_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            queue_stalled_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"queue_stalled_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_queue_stalled_minutes() -> Result<()> {
        let session_set_args = SessionSetArgs {
            queue_stalled_minutes: Some(20),
            ..Default::default()
        };
        verify(session_set_args, None, "\"queue-stalled-minutes\":20")
    }

    #[test]
    fn request_session_set_semver_600_queue_stalled_minutes() -> Result<()> {
        let session_set_args = SessionSetArgs {
            queue_stalled_minutes: Some(20),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"queue_stalled_minutes\":20")
    }

    #[test]
    fn request_session_set_legacy_rename_partial_files() -> Result<()> {
        let session_set_args = SessionSetArgs {
            rename_partial_files: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"rename-partial-files\":false")
    }

    #[test]
    fn request_session_set_semver_600_rename_partial_files() -> Result<()> {
        let session_set_args = SessionSetArgs {
            rename_partial_files: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"rename_partial_files\":false")
    }

    #[test]
    fn request_session_set_legacy_reqq() -> Result<()> {
        let session_set_args = SessionSetArgs {
            reqq: Some(2500),
            ..Default::default()
        };
        verify(session_set_args, None, "\"reqq\":2500")
    }

    #[test]
    fn request_session_set_semver_600_reqq() -> Result<()> {
        let session_set_args = SessionSetArgs {
            reqq: Some(2500),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"reqq\":2500")
    }

    #[test]
    fn request_session_set_legacy_script_torrent_added_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_added_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"script-torrent-added-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_script_torrent_added_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_added_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
            "\"script_torrent_added_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_script_torrent_added_filename() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_added_filename: Some("/scripts/added.sh".to_string()),
            ..Default::default()
        };
        verify(session_set_args, None, "\"script-torrent-added-filename\":\"/scripts/added.sh\"")
    }

    #[test]
    fn request_session_set_semver_600_script_torrent_added_filename() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_added_filename: Some("/scripts/added.sh".to_string()),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"script_torrent_added_filename\":\"/scripts/added.sh\"")
    }

    #[test]
    fn request_session_set_legacy_script_torrent_done_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"script-torrent-done-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_script_torrent_done_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
            "\"script_torrent_done_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_script_torrent_done_filename() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_filename: Some("/scripts/done.sh".to_string()),
            ..Default::default()
        };
        verify(session_set_args, None, "\"script-torrent-done-filename\":\"/scripts/done.sh\"")
    }

    #[test]
    fn request_session_set_semver_600_script_torrent_done_filename() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_filename: Some("/scripts/done.sh".to_string()),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"script_torrent_done_filename\":\"/scripts/done.sh\"")
    }

    #[test]
    fn request_session_set_legacy_script_torrent_done_seeding_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_seeding_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"script-torrent-done-seeding-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_script_torrent_done_seeding_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_seeding_enabled: Some(true),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"script_torrent_done_seeding_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_script_torrent_done_seeding_filename() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_seeding_filename: Some("/scripts/done_seeding.sh".to_string()),
            ..Default::default()
        };
        verify(
            session_set_args,
            None,
            "\"script-torrent-done-seeding-filename\":\"/scripts/done_seeding.sh\"")
    }

    #[test]
    fn request_session_set_semver_600_script_torrent_done_seeding_filename() -> Result<()> {
        let session_set_args = SessionSetArgs {
            script_torrent_done_seeding_filename: Some("/scripts/done_seeding.sh".to_string()),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"script_torrent_done_seeding_filename\":\"/scripts/done_seeding.sh\"")
    }

    #[test]
    fn request_session_set_legacy_seed_queue_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_queue_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"seed-queue-enabled\":false")
    }

    #[test]
    fn request_session_set_semver_600_seed_queue_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_queue_enabled: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_queue_enabled\":false")
    }

    #[test]
    fn request_session_set_legacy_seed_queue_size() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_queue_size: Some(1000),
            ..Default::default()
        };
        verify(session_set_args, None, "\"seed-queue-size\":1000")
    }

    #[test]
    fn request_session_set_semver_600_seed_queue_size() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_queue_size: Some(1000),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_queue_size\":1000")
    }

    #[test]
    fn request_session_set_legacy_seed_ratio_limit() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_ratio_limit: Some(2.0),
            ..Default::default()
        };
        verify(session_set_args, None, "\"seedRatioLimit\":2.0")
    }

    #[test]
    fn request_session_set_semver_600_seed_ratio_limit() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_ratio_limit: Some(2.0),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_ratio_limit\":2.0")
    }

    #[test]
    fn request_session_set_legacy_seed_ratio_limited() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_ratio_limited: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"seedRatioLimited\":false")
    }

    #[test]
    fn request_session_set_semver_600_seed_ratio_limited() -> Result<()> {
        let session_set_args = SessionSetArgs {
            seed_ratio_limited: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_ratio_limited\":false")
    }

    #[test]
    fn request_session_set_legacy_sequential_download() -> Result<()> {
        let session_set_args = SessionSetArgs {
            sequential_download: Some(false),
            ..Default::default()
        };
        // `sequential_download` only exists post- semver-6.0.0.
        verify(session_set_args, None, "")
    }

    #[test]
    fn request_session_set_semver_600_sequential_download() -> Result<()> {
        let session_set_args = SessionSetArgs {
            sequential_download: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"sequential_download\":false")
    }

    #[test]
    fn request_session_set_legacy_sequential_download_from_piece() -> Result<()> {
        let session_set_args = SessionSetArgs {
            sequential_download_from_piece: Some(321),
            ..Default::default()
        };
        // `sequential_download_from_piece` only exists post- semver-6.0.0.
        verify(session_set_args, None, "")
    }

    #[test]
    fn request_session_set_semver_600_sequential_download_from_piece() -> Result<()> {
        let session_set_args = SessionSetArgs {
            sequential_download_from_piece: Some(403),
            ..Default::default()
        };
        verify(
            session_set_args,
            Some(JSON_RPC_VERSION_2_0),
            "\"sequential_download_from_piece\":403")
    }

    #[test]
    fn request_session_set_legacy_speed_limit_down_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_down_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"speed-limit-down-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_speed_limit_down_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_down_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_down_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_speed_limit_down() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_down: Some(2000),
            ..Default::default()
        };
        verify(session_set_args, None, "\"speed-limit-down\":2000")
    }

    #[test]
    fn request_session_set_semver_600_speed_limit_down() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_down: Some(2000),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_down\":2000")
    }

    #[test]
    fn request_session_set_legacy_speed_limit_up_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_up_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"speed-limit-up-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_speed_limit_up_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_up_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_up_enabled\":true")
    }

    #[test]
    fn request_session_set_legacy_speed_limit_up() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_up: Some(1000),
            ..Default::default()
        };
        verify(session_set_args, None, "\"speed-limit-up\":1000")
    }

    #[test]
    fn request_session_set_semver_600_speed_limit_up() -> Result<()> {
        let session_set_args = SessionSetArgs {
            speed_limit_up: Some(1000),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_up\":1000")
    }

    #[test]
    fn request_session_set_legacy_start_added_torrents() -> Result<()> {
        let session_set_args = SessionSetArgs {
            start_added_torrents: Some(false),
            ..Default::default()
        };
        verify(session_set_args, None, "\"start-added-torrents\":false")
    }

    #[test]
    fn request_session_set_semver_600_start_added_torrents() -> Result<()> {
        let session_set_args = SessionSetArgs {
            start_added_torrents: Some(false),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"start_added_torrents\":false")
    }

    #[test]
    fn request_session_set_legacy_trash_original_torrent_files() -> Result<()> {
        let session_set_args = SessionSetArgs {
            trash_original_torrent_files: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"trash-original-torrent-files\":true")
    }

    #[test]
    fn request_session_set_semver_600_trash_original_torrent_files() -> Result<()> {
        let session_set_args = SessionSetArgs {
            trash_original_torrent_files: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
            "\"trash_original_torrent_files\":true")
    }

    #[test]
    fn request_session_set_legacy_utp_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            utp_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, None, "\"utp-enabled\":true")
    }

    #[test]
    fn request_session_set_semver_600_utp_enabled() -> Result<()> {
        let session_set_args = SessionSetArgs {
            utp_enabled: Some(true),
            ..Default::default()
        };
        verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"utp_enabled\":true")
    }
}
