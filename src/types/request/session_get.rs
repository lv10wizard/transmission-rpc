use compat_macros::GenerateCompat;
use serde::Serialize;

use super::{Args, Method, RpcRequest, map_vec};

/// Represents internal request arguments for the [`session_get`] method for json serialization
/// purposes.
///
/// An empty `fields` collection will request all [`SessionGet`] fields. Note that this behavior
/// differs from manual rpc requests where any empty `fields` array will yield an empty
/// `session-get` response.
///
/// # Constructor
///
/// * [`Into`] converts any collection of [`SessionGetField`]s that implements [`IntoIterator`]
/// (like [`Vec`], [`HashSet`], etc)
///
/// # Example
///
/// ```rust
/// let args: SessionGetArgs = vec![
///     SessionGetField::ConfigDir,
///     SessionGetField::Encryption,
///     SessionGetField::RpcVersion,
/// ].into();
/// ```
///
/// [`session_get`]: RpcRequest::session_get
/// [`SessionGet`]: crate::types::SessionGet
/// [`HashSet`]: std::collections::HashSet
#[derive(GenerateCompat, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SessionGetArgs {
    #[compat(type = Vec<__semver_600_compat_SessionGetField>, map = map_vec)]
    #[serde(skip_serializing_if = "Vec::is_empty")] // Treat empty the same as `None`.
    pub(crate) fields: Vec<SessionGetField>,
}

impl<I> From<I> for SessionGetArgs
where
    I: IntoIterator<Item = SessionGetField>,
{
    fn from(value: I) -> Self {
        Self {
            fields: value.into_iter().collect(),
        }
    }
}

impl<I> From<I> for Args
where
    I: IntoIterator<Item = SessionGetField>,
{
    fn from(value: I) -> Self {
        let session_get: SessionGetArgs = value.into();
        session_get.into()
    }
}

impl<I> From<I> for RpcRequest
where
    I: IntoIterator<Item = SessionGetField>,
{
    fn from(value: I) -> Self {
        let session_get: SessionGetArgs = value.into();
        session_get.into()
    }
}

impl From<SessionGetArgs> for Args {
    fn from(value: SessionGetArgs) -> Self {
        Self::SessionGet(value)
    }
}

impl From<SessionGetArgs> for RpcRequest {
    fn from(value: SessionGetArgs) -> Self {
        Self {
            method: Method::SessionGet,
            arguments: Some(value.into()),
            tag: None,
            jsonrpc: None,
        }
    }
}

#[derive(GenerateCompat, Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SessionGetField {
    AltSpeedDown,
    AltSpeedEnabled,
    AltSpeedTimeBegin,
    AltSpeedTimeDay,
    AltSpeedTimeEnabled,
    AltSpeedTimeEnd,
    AltSpeedUp,
    AntiBruteForceEnabled,
    AntiBruteForceThreshold,
    BlocklistEnabled,
    BlocklistSize,
    BlocklistUrl,
    #[compat(name = CacheSizeMib)]
    CacheSizeMb,
    ConfigDir,
    DefaultTrackers,
    DhtEnabled,
    DownloadDir,
    DownloadDirFreeSpace,
    DownloadQueueEnabled,
    DownloadQueueSize,
    Encryption,
    IdleSeedingLimitEnabled,
    IdleSeedingLimit,
    IncompleteDirEnabled,
    IncompleteDir,
    LpdEnabled,
    PeerLimitGlobal,
    PeerLimitPerTorrent,
    PeerPortRandomOnStart,
    PeerPort,
    PexEnabled,
    PortForwardingEnabled,
    PreferredTransports,
    QueueStalledEnabled,
    QueueStalledMinutes,
    RenamePartialFiles,
    Reqq,
    RpcVersionMinimum,
    RpcVersionSemver,
    RpcVersion,
    ScriptTorrentAddedEnabled,
    ScriptTorrentAddedFilename,
    ScriptTorrentDoneEnabled,
    ScriptTorrentDoneFilename,
    ScriptTorrentDoneSeedingEnabled,
    ScriptTorrentDoneSeedingFilename,
    SeedQueueEnabled,
    SeedQueueSize,
    #[serde(rename = "seedRatioLimit")]
    SeedRatioLimit,
    #[serde(rename = "seedRatioLimited")]
    SeedRatioLimited,
    #[serde(rename = "sequential_download")] // Doesn't exist pre- semver-6.0.0
    SequentialDownload,
    SessionId,
    SpeedLimitDownEnabled,
    SpeedLimitDown,
    SpeedLimitUpEnabled,
    SpeedLimitUp,
    StartAddedTorrents,
    TrashOriginalTorrentFiles,
    Units,
    UtpEnabled,
    Version,
}

#[cfg(test)]
mod serde_tests {
    use crate::types::{JSON_RPC_VERSION_2_0, Result, request::test_helper::verify};
    use super::*;

    fn legacy_test<I>(fields: I, expected: &str) -> Result<()>
    where
        I: IntoIterator<Item = SessionGetField>,
    {
        verify(fields, None,
            &format!("{{\
                \"method\":\"session-get\",\
                \"arguments\":{{\
                    \"fields\":[\
                        \"{expected}\"\
                    ]\
                }}\
            }}"))
    }

    fn semver_600_test<I>(fields: I, expected: &str) -> Result<()>
    where
        I: IntoIterator<Item = SessionGetField>,
    {
        verify(fields, Some(JSON_RPC_VERSION_2_0),
            &format!("{{\
                \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
                \"method\":\"session_get\",\
                \"params\":{{\
                    \"fields\":[\
                        \"{expected}\"\
                    ]\
                }},\
                \"id\":0\
            }}"))
    }

    // -----------------------------------------------------------------

    #[test]
    fn request_session_get_legacy_alt_speed_down() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedDown], "alt-speed-down")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_down() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedDown], "alt_speed_down")
    }

    #[test]
    fn request_session_get_legacy_alt_speed_enabled() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedEnabled], "alt-speed-enabled")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_enabled() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedEnabled], "alt_speed_enabled")
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_begin() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedTimeBegin], "alt-speed-time-begin")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_begin() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedTimeBegin], "alt_speed_time_begin")
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_day() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedTimeDay], "alt-speed-time-day")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_day() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedTimeDay], "alt_speed_time_day")
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_enabled() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedTimeEnabled], "alt-speed-time-enabled")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_enabled() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedTimeEnabled], "alt_speed_time_enabled")
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_end() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedTimeEnd], "alt-speed-time-end")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_end() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedTimeEnd], "alt_speed_time_end")
    }

    #[test]
    fn request_session_get_legacy_alt_speed_up() -> Result<()> {
        legacy_test([SessionGetField::AltSpeedUp], "alt-speed-up")
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_up() -> Result<()> {
        semver_600_test([SessionGetField::AltSpeedUp], "alt_speed_up")
    }

    #[test]
    fn request_session_get_legacy_anti_brute_force_enabled() -> Result<()> {
        legacy_test([SessionGetField::AntiBruteForceEnabled], "anti-brute-force-enabled")
    }

    #[test]
    fn request_session_get_semver_600_anti_brute_force_enabled() -> Result<()> {
        semver_600_test([SessionGetField::AntiBruteForceEnabled], "anti_brute_force_enabled")
    }

    #[test]
    fn request_session_get_legacy_anti_brute_force_threshold() -> Result<()> {
        legacy_test([SessionGetField::AntiBruteForceThreshold], "anti-brute-force-threshold")
    }

    #[test]
    fn request_session_get_semver_600_anti_brute_force_threshold() -> Result<()> {
        semver_600_test([SessionGetField::AntiBruteForceThreshold], "anti_brute_force_threshold")
    }

    #[test]
    fn request_session_get_legacy_blocklist_enabled() -> Result<()> {
        legacy_test([SessionGetField::BlocklistEnabled], "blocklist-enabled")
    }

    #[test]
    fn request_session_get_semver_600_blocklist_enabled() -> Result<()> {
        semver_600_test([SessionGetField::BlocklistEnabled], "blocklist_enabled")
    }

    #[test]
    fn request_session_get_legacy_blocklist_size() -> Result<()> {
        legacy_test([SessionGetField::BlocklistSize], "blocklist-size")
    }

    #[test]
    fn request_session_get_semver_600_blocklist_size() -> Result<()> {
        semver_600_test([SessionGetField::BlocklistSize], "blocklist_size")
    }

    #[test]
    fn request_session_get_legacy_blocklist_url() -> Result<()> {
        legacy_test([SessionGetField::BlocklistUrl], "blocklist-url")
    }

    #[test]
    fn request_session_get_semver_600_blocklist_url() -> Result<()> {
        semver_600_test([SessionGetField::BlocklistUrl], "blocklist_url")
    }

    #[test]
    fn request_session_get_legacy_cache_size_mb() -> Result<()> {
        legacy_test([SessionGetField::CacheSizeMb], "cache-size-mb")
    }

    #[test]
    fn request_session_get_semver_600_cache_size_mb() -> Result<()> {
        semver_600_test([SessionGetField::CacheSizeMb], "cache_size_mib")
    }

    #[test]
    fn request_session_get_legacy_config_dir() -> Result<()> {
        legacy_test([SessionGetField::ConfigDir], "config-dir")
    }

    #[test]
    fn request_session_get_semver_600_config_dir() -> Result<()> {
        semver_600_test([SessionGetField::ConfigDir], "config_dir")
    }

    #[test]
    fn request_session_get_legacy_default_trackers() -> Result<()> {
        legacy_test([SessionGetField::DefaultTrackers], "default-trackers")
    }

    #[test]
    fn request_session_get_semver_600_default_trackers() -> Result<()> {
        semver_600_test([SessionGetField::DefaultTrackers], "default_trackers")
    }

    #[test]
    fn request_session_get_legacy_dht_enabled() -> Result<()> {
        legacy_test([SessionGetField::DhtEnabled], "dht-enabled")
    }

    #[test]
    fn request_session_get_semver_600_dht_enabled() -> Result<()> {
        semver_600_test([SessionGetField::DhtEnabled], "dht_enabled")
    }

    #[test]
    fn request_session_get_legacy_download_dir() -> Result<()> {
        legacy_test([SessionGetField::DownloadDir], "download-dir")
    }

    #[test]
    fn request_session_get_semver_600_download_dir() -> Result<()> {
        semver_600_test([SessionGetField::DownloadDir], "download_dir")
    }

    #[test]
    fn request_session_get_legacy_download_dir_free_space() -> Result<()> {
        legacy_test([SessionGetField::DownloadDirFreeSpace], "download-dir-free-space")
    }

    #[test]
    fn request_session_get_semver_600_download_dir_free_space() -> Result<()> {
        semver_600_test([SessionGetField::DownloadDirFreeSpace], "download_dir_free_space")
    }

    #[test]
    fn request_session_get_legacy_download_queue_enabled() -> Result<()> {
        legacy_test([SessionGetField::DownloadQueueEnabled], "download-queue-enabled")
    }

    #[test]
    fn request_session_get_semver_600_download_queue_enabled() -> Result<()> {
        semver_600_test([SessionGetField::DownloadQueueEnabled], "download_queue_enabled")
    }

    #[test]
    fn request_session_get_legacy_download_queue_size() -> Result<()> {
        legacy_test([SessionGetField::DownloadQueueSize], "download-queue-size")
    }

    #[test]
    fn request_session_get_semver_600_download_queue_size() -> Result<()> {
        semver_600_test([SessionGetField::DownloadQueueSize], "download_queue_size")
    }

    #[test]
    fn request_session_get_legacy_encryption() -> Result<()> {
        legacy_test([SessionGetField::Encryption], "encryption")
    }

    #[test]
    fn request_session_get_semver_600_encryption() -> Result<()> {
        semver_600_test([SessionGetField::Encryption], "encryption")
    }

    #[test]
    fn request_session_get_legacy_idle_seeding_limit() -> Result<()> {
        legacy_test([SessionGetField::IdleSeedingLimit], "idle-seeding-limit")
    }

    #[test]
    fn request_session_get_semver_600_idle_seeding_limit() -> Result<()> {
        semver_600_test([SessionGetField::IdleSeedingLimit], "idle_seeding_limit")
    }

    #[test]
    fn request_session_get_legacy_idle_seeding_limit_enabled() -> Result<()> {
        legacy_test([SessionGetField::IdleSeedingLimitEnabled], "idle-seeding-limit-enabled")
    }

    #[test]
    fn request_session_get_semver_600_idle_seeding_limit_enabled() -> Result<()> {
        semver_600_test([SessionGetField::IdleSeedingLimitEnabled], "idle_seeding_limit_enabled")
    }

    #[test]
    fn request_session_get_legacy_incomplete_dir() -> Result<()> {
        legacy_test([SessionGetField::IncompleteDir], "incomplete-dir")
    }

    #[test]
    fn request_session_get_semver_600_incomplete_dir() -> Result<()> {
        semver_600_test([SessionGetField::IncompleteDir], "incomplete_dir")
    }

    #[test]
    fn request_session_get_legacy_incomplete_dir_enabled() -> Result<()> {
        legacy_test([SessionGetField::IncompleteDirEnabled], "incomplete-dir-enabled")
    }

    #[test]
    fn request_session_get_semver_600_incomplete_dir_enabled() -> Result<()> {
        semver_600_test([SessionGetField::IncompleteDirEnabled], "incomplete_dir_enabled")
    }

    #[test]
    fn request_session_get_legacy_lpd_enabled() -> Result<()> {
        legacy_test([SessionGetField::LpdEnabled], "lpd-enabled")
    }

    #[test]
    fn request_session_get_semver_600_lpd_enabled() -> Result<()> {
        semver_600_test([SessionGetField::LpdEnabled], "lpd_enabled")
    }

    #[test]
    fn request_session_get_legacy_peer_limit_global() -> Result<()> {
        legacy_test([SessionGetField::PeerLimitGlobal], "peer-limit-global")
    }

    #[test]
    fn request_session_get_semver_600_peer_limit_global() -> Result<()> {
        semver_600_test([SessionGetField::PeerLimitGlobal], "peer_limit_global")
    }

    #[test]
    fn request_session_get_legacy_peer_limit_per_torrent() -> Result<()> {
        legacy_test([SessionGetField::PeerLimitPerTorrent], "peer-limit-per-torrent")
    }

    #[test]
    fn request_session_get_semver_600_peer_limit_per_torrent() -> Result<()> {
        semver_600_test([SessionGetField::PeerLimitPerTorrent], "peer_limit_per_torrent")
    }

    #[test]
    fn request_session_get_legacy_peer_port_random_on_start() -> Result<()> {
        legacy_test([SessionGetField::PeerPortRandomOnStart], "peer-port-random-on-start")
    }

    #[test]
    fn request_session_get_semver_600_peer_port_random_on_start() -> Result<()> {
        semver_600_test([SessionGetField::PeerPortRandomOnStart], "peer_port_random_on_start")
    }

    #[test]
    fn request_session_get_legacy_peer_port() -> Result<()> {
        legacy_test([SessionGetField::PeerPort], "peer-port")
    }

    #[test]
    fn request_session_get_semver_600_peer_port() -> Result<()> {
        semver_600_test([SessionGetField::PeerPort], "peer_port")
    }

    #[test]
    fn request_session_get_legacy_pex_enabled() -> Result<()> {
        legacy_test([SessionGetField::PexEnabled], "pex-enabled")
    }

    #[test]
    fn request_session_get_semver_600_pex_enabled() -> Result<()> {
        semver_600_test([SessionGetField::PexEnabled], "pex_enabled")
    }

    #[test]
    fn request_session_get_legacy_port_forwarding_enabled() -> Result<()> {
        legacy_test([SessionGetField::PortForwardingEnabled], "port-forwarding-enabled")
    }

    #[test]
    fn request_session_get_semver_600_port_forwarding_enabled() -> Result<()> {
        semver_600_test([SessionGetField::PortForwardingEnabled], "port_forwarding_enabled")
    }

    #[test]
    fn request_session_get_legacy_preferred_transports() -> Result<()> {
        legacy_test([SessionGetField::PreferredTransports], "preferred-transports")
    }

    #[test]
    fn request_session_get_semver_600_preferred_transports() -> Result<()> {
        semver_600_test([SessionGetField::PreferredTransports], "preferred_transports")
    }

    #[test]
    fn request_session_get_legacy_queue_stalled_enabled() -> Result<()> {
        legacy_test([SessionGetField::QueueStalledEnabled], "queue-stalled-enabled")
    }

    #[test]
    fn request_session_get_semver_600_queue_stalled_enabled() -> Result<()> {
        semver_600_test([SessionGetField::QueueStalledEnabled], "queue_stalled_enabled")
    }

    #[test]
    fn request_session_get_legacy_queue_stalled_minutes() -> Result<()> {
        legacy_test([SessionGetField::QueueStalledMinutes], "queue-stalled-minutes")
    }

    #[test]
    fn request_session_get_semver_600_queue_stalled_minutes() -> Result<()> {
        semver_600_test([SessionGetField::QueueStalledMinutes], "queue_stalled_minutes")
    }

    #[test]
    fn request_session_get_legacy_rename_partial_files() -> Result<()> {
        legacy_test([SessionGetField::RenamePartialFiles], "rename-partial-files")
    }

    #[test]
    fn request_session_get_semver_600_rename_partial_files() -> Result<()> {
        semver_600_test([SessionGetField::RenamePartialFiles], "rename_partial_files")
    }

    #[test]
    fn request_session_get_legacy_reqq() -> Result<()> {
        legacy_test([SessionGetField::Reqq], "reqq")
    }

    #[test]
    fn request_session_get_semver_600_reqq() -> Result<()> {
        semver_600_test([SessionGetField::Reqq], "reqq")
    }

    #[test]
    fn request_session_get_legacy_rpc_version_minimum() -> Result<()> {
        legacy_test([SessionGetField::RpcVersionMinimum], "rpc-version-minimum")
    }

    #[test]
    fn request_session_get_semver_600_rpc_version_minimum() -> Result<()> {
        semver_600_test([SessionGetField::RpcVersionMinimum], "rpc_version_minimum")
    }

    #[test]
    fn request_session_get_legacy_rpc_version_semver() -> Result<()> {
        legacy_test([SessionGetField::RpcVersionSemver], "rpc-version-semver")
    }

    #[test]
    fn request_session_get_semver_600_rpc_version_semver() -> Result<()> {
        semver_600_test([SessionGetField::RpcVersionSemver], "rpc_version_semver")
    }

    #[test]
    fn request_session_get_legacy_rpc_version() -> Result<()> {
        legacy_test([SessionGetField::RpcVersion], "rpc-version")
    }

    #[test]
    fn request_session_get_semver_600_rpc_version() -> Result<()> {
        semver_600_test([SessionGetField::RpcVersion], "rpc_version")
    }

    #[test]
    fn request_session_get_legacy_script_torrent_added_enabled() -> Result<()> {
        legacy_test([SessionGetField::ScriptTorrentAddedEnabled], "script-torrent-added-enabled")
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_added_enabled() -> Result<()> {
        semver_600_test(
            [SessionGetField::ScriptTorrentAddedEnabled],
            "script_torrent_added_enabled")
    }

    #[test]
    fn request_session_get_legacy_script_torrent_added_filename() -> Result<()> {
        legacy_test([SessionGetField::ScriptTorrentAddedFilename], "script-torrent-added-filename")
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_added_filename() -> Result<()> {
        semver_600_test(
            [SessionGetField::ScriptTorrentAddedFilename],
            "script_torrent_added_filename")
    }

    #[test]
    fn request_session_get_legacy_script_torrent_done_enabled() -> Result<()> {
        legacy_test([SessionGetField::ScriptTorrentDoneEnabled], "script-torrent-done-enabled")
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_enabled() -> Result<()> {
        semver_600_test(
            [SessionGetField::ScriptTorrentDoneEnabled],
            "script_torrent_done_enabled")
    }

    #[test]
    fn request_session_get_legacy_script_torrent_done_filename() -> Result<()> {
        legacy_test([SessionGetField::ScriptTorrentDoneFilename], "script-torrent-done-filename")
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_filename() -> Result<()> {
        semver_600_test(
            [SessionGetField::ScriptTorrentDoneFilename],
            "script_torrent_done_filename")
    }

    #[test]
    fn request_session_get_legacy_script_torrent_done_seeding_enabled() -> Result<()> {
        legacy_test(
            [SessionGetField::ScriptTorrentDoneSeedingEnabled],
            "script-torrent-done-seeding-enabled")
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_seeding_enabled() -> Result<()> {
        semver_600_test(
            [SessionGetField::ScriptTorrentDoneSeedingEnabled],
            "script_torrent_done_seeding_enabled")
    }
    #[test]
    fn request_session_get_legacy_script_torrent_done_seeding_filename() -> Result<()> {
        legacy_test(
            [SessionGetField::ScriptTorrentDoneSeedingFilename],
            "script-torrent-done-seeding-filename")
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_seeding_filename() -> Result<()> {
        semver_600_test(
            [SessionGetField::ScriptTorrentDoneSeedingFilename],
            "script_torrent_done_seeding_filename")
    }

    #[test]
    fn request_session_get_legacy_seed_queue_enabled() -> Result<()> {
        legacy_test([SessionGetField::SeedQueueEnabled], "seed-queue-enabled")
    }

    #[test]
    fn request_session_get_semver_600_seed_queue_enabled() -> Result<()> {
        semver_600_test([SessionGetField::SeedQueueEnabled], "seed_queue_enabled")
    }

    #[test]
    fn request_session_get_legacy_seed_queue_size() -> Result<()> {
        legacy_test([SessionGetField::SeedQueueSize], "seed-queue-size")
    }

    #[test]
    fn request_session_get_semver_600_seed_queue_size() -> Result<()> {
        semver_600_test([SessionGetField::SeedQueueSize], "seed_queue_size")
    }

    #[test]
    fn request_session_get_legacy_seed_ratio_limit() -> Result<()> {
        legacy_test([SessionGetField::SeedRatioLimit], "seedRatioLimit")
    }

    #[test]
    fn request_session_get_semver_600_seed_ratio_limit() -> Result<()> {
        semver_600_test([SessionGetField::SeedRatioLimit], "seed_ratio_limit")
    }

    #[test]
    fn request_session_get_legacy_seed_ratio_limited() -> Result<()> {
        legacy_test([SessionGetField::SeedRatioLimited], "seedRatioLimited")
    }

    #[test]
    fn request_session_get_semver_600_seed_ratio_limited() -> Result<()> {
        semver_600_test([SessionGetField::SeedRatioLimited], "seed_ratio_limited")
    }

    #[test]
    fn request_session_get_legacy_sequential_download() -> Result<()> {
        legacy_test([SessionGetField::SequentialDownload], "sequential_download")
    }

    #[test]
    fn request_session_get_semver_600_sequential_download() -> Result<()> {
        semver_600_test([SessionGetField::SequentialDownload], "sequential_download")
    }

    #[test]
    fn request_session_get_legacy_session_id() -> Result<()> {
        legacy_test([SessionGetField::SessionId], "session-id")
    }

    #[test]
    fn request_session_get_semver_600_session_id() -> Result<()> {
        semver_600_test([SessionGetField::SessionId], "session_id")
    }

    #[test]
    fn request_session_get_legacy_speed_limit_down_enabled() -> Result<()> {
        legacy_test([SessionGetField::SpeedLimitDownEnabled], "speed-limit-down-enabled")
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_down_enabled() -> Result<()> {
        semver_600_test([SessionGetField::SpeedLimitDownEnabled], "speed_limit_down_enabled")
    }

    #[test]
    fn request_session_get_legacy_speed_limit_down() -> Result<()> {
        legacy_test([SessionGetField::SpeedLimitDown], "speed-limit-down")
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_down() -> Result<()> {
        semver_600_test([SessionGetField::SpeedLimitDown], "speed_limit_down")
    }

    #[test]
    fn request_session_get_legacy_speed_limit_up_enabled() -> Result<()> {
        legacy_test([SessionGetField::SpeedLimitUpEnabled], "speed-limit-up-enabled")
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_up_enabled() -> Result<()> {
        semver_600_test([SessionGetField::SpeedLimitUpEnabled], "speed_limit_up_enabled")
    }

    #[test]
    fn request_session_get_legacy_speed_limit_up() -> Result<()> {
        legacy_test([SessionGetField::SpeedLimitUp], "speed-limit-up")
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_up() -> Result<()> {
        semver_600_test([SessionGetField::SpeedLimitUp], "speed_limit_up")
    }

    #[test]
    fn request_session_get_legacy_start_added_torrents() -> Result<()> {
        legacy_test([SessionGetField::StartAddedTorrents], "start-added-torrents")
    }

    #[test]
    fn request_session_get_semver_600_start_added_torrents() -> Result<()> {
        semver_600_test([SessionGetField::StartAddedTorrents], "start_added_torrents")
    }

    #[test]
    fn request_session_get_legacy_trash_original_torrent_files() -> Result<()> {
        legacy_test([SessionGetField::TrashOriginalTorrentFiles], "trash-original-torrent-files")
    }

    #[test]
    fn request_session_get_semver_600_trash_original_torrent_files() -> Result<()> {
        semver_600_test(
            [SessionGetField::TrashOriginalTorrentFiles],
            "trash_original_torrent_files")
    }

    #[test]
    fn request_session_get_legacy_units() -> Result<()> {
        legacy_test([SessionGetField::Units], "units")
    }

    #[test]
    fn request_session_get_semver_600_units() -> Result<()> {
        semver_600_test([SessionGetField::Units], "units")
    }

    #[test]
    fn request_session_get_legacy_utp_enabled() -> Result<()> {
        legacy_test([SessionGetField::UtpEnabled], "utp-enabled")
    }

    #[test]
    fn request_session_get_semver_600_utp_enabled() -> Result<()> {
        semver_600_test([SessionGetField::UtpEnabled], "utp_enabled")
    }

    #[test]
    fn request_session_get_legacy_version() -> Result<()> {
        legacy_test([SessionGetField::Version], "version")
    }

    #[test]
    fn request_session_get_semver_600_version() -> Result<()> {
        semver_600_test([SessionGetField::Version], "version")
    }
}
