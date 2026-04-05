use compat_macros::GenerateCompat;
use serde::Serialize;

use super::{Args, RpcRequest, map_vec};

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
#[compat(placeholder = P)]
pub(crate) struct SessionGetArgs {
    #[compat(type = Vec<P>, map = map_vec)]
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

/// Defines valid [`session_get`] request fields to limit what [Transmission] session data to
/// respond with.
///
/// [`session_get`]: crate::TransClient::session_get
/// [Transmission]: <https://transmissionbt.com/>
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
    /// > ⚠ **DEPRECATED** in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?):
    /// The memory cache is being removed, making this setting moot. The setting will still be
    /// gettable and settable via RPC session_get and session_set until Transmission
    /// 5.0.0 to avoid client breakage, but it will be otherwise unused in libtransmission. Clients
    ///   should stop using this key.
    #[compat(name = CacheSizeMib)]
    /*
    #[added(semver = "3.4.0")]
    #[changed(semver = "6.0.0", name = CacheSizeMib)]
    #[deprecated(semver = "6.1.0", reason = "The memory cache is being removed, making this setting moot. The setting will still be gettable and settable via RPC session_get and session_set until Transmission 5.0.0 to avoid client breakage, but it will be otherwise unused in libtransmission. Clients should stop using this key.")]
    */
    CacheSizeMb,
    ConfigDir,
    DefaultTrackers,
    DhtEnabled,
    DownloadDir,
    /// > ⚠ **DEPRECATED** in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17):
    /// Use [`free_space`] instead.
    ///
    /// [`free_space`]: crate::TransClient::free_space
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
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use [`RpcVersionSemver`] instead.
    ///
    /// [`RpcVersionSemver`]: Self::RpcVersionSemver
    RpcVersionMinimum,
    RpcVersionSemver,
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use [`RpcVersionSemver`] instead.
    ///
    /// [`RpcVersionSemver`]: Self::RpcVersionSemver
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
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use [`RpcVersionSemver`] instead.
    ///
    /// [`RpcVersionSemver`]: Self::RpcVersionSemver
    TcpEnabled,
    TrashOriginalTorrentFiles,
    Units,
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use [`PreferredTransports`] instead.
    ///
    /// [`PreferredTransports`]: Self::PreferredTransports
    UtpEnabled,
    Version,
}

#[cfg(test)]
mod serde_tests {
    use serde_json;

    use crate::types::{
        JSON_RPC_VERSION_2_0,
        Result,
        request::test_helper::verify,
    };
    use super::*;

    /// Verifies that the [`RpcRequest`] serialized with `args` produces json with the
    /// `expected_fields`, ie. that the `"arguments"` (or `"params"` for semver-6.0.0+) contains a
    /// `"fields"` key with `expected_fields` as an array of strings).
    ///
    /// Like with [`verify`], `jsonrpc` controls the version of the serialized request.
    fn verify_fields<'f, T, F>(args: T, jsonrpc: Option<&str>, expected_fields: F)
        -> Result<()>
    where
        T: Into<RpcRequest>,
        F: IntoIterator<Item = &'f str>,
    {
        let fields: Vec<_> = expected_fields.into_iter().collect();
        let expected_fields = format!("\"fields\":{}", serde_json::to_string(&fields)?);
        verify(args, jsonrpc, &expected_fields)
    }

    // -----------------------------------------------------------------

    #[test]
    fn request_session_get_legacy_alt_speed_down() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedDown], None, ["alt-speed-down"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_down() -> Result<()> {
        verify_fields(
            [SessionGetField::AltSpeedDown],
            Some(JSON_RPC_VERSION_2_0),
            ["alt_speed_down"])
    }

    #[test]
    fn request_session_get_legacy_alt_speed_enabled() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedEnabled], None, ["alt-speed-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::AltSpeedEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["alt_speed_enabled"])
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_begin() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedTimeBegin], None, ["alt-speed-time-begin"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_begin() -> Result<()> {
        verify_fields(
            [SessionGetField::AltSpeedTimeBegin],
            Some(JSON_RPC_VERSION_2_0),
            ["alt_speed_time_begin"])
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_day() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedTimeDay], None, ["alt-speed-time-day"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_day() -> Result<()> {
        verify_fields(
            [SessionGetField::AltSpeedTimeDay],
            Some(JSON_RPC_VERSION_2_0),
            ["alt_speed_time_day"])
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_enabled() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedTimeEnabled], None, ["alt-speed-time-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::AltSpeedTimeEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["alt_speed_time_enabled"])
    }

    #[test]
    fn request_session_get_legacy_alt_speed_time_end() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedTimeEnd], None, ["alt-speed-time-end"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_time_end() -> Result<()> {
        verify_fields(
            [SessionGetField::AltSpeedTimeEnd],
            Some(JSON_RPC_VERSION_2_0),
            ["alt_speed_time_end"])
    }

    #[test]
    fn request_session_get_legacy_alt_speed_up() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedUp], None, ["alt-speed-up"])
    }

    #[test]
    fn request_session_get_semver_600_alt_speed_up() -> Result<()> {
        verify_fields([SessionGetField::AltSpeedUp], Some(JSON_RPC_VERSION_2_0), ["alt_speed_up"])
    }

    #[test]
    fn request_session_get_legacy_anti_brute_force_enabled() -> Result<()> {
        verify_fields([SessionGetField::AntiBruteForceEnabled], None, ["anti-brute-force-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_anti_brute_force_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::AntiBruteForceEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["anti_brute_force_enabled"])
    }

    #[test]
    fn request_session_get_legacy_anti_brute_force_threshold() -> Result<()> {
        verify_fields(
            [SessionGetField::AntiBruteForceThreshold],
            None,
            ["anti-brute-force-threshold"])
    }

    #[test]
    fn request_session_get_semver_600_anti_brute_force_threshold() -> Result<()> {
        verify_fields(
            [SessionGetField::AntiBruteForceThreshold],
            Some(JSON_RPC_VERSION_2_0),
            ["anti_brute_force_threshold"])
    }

    #[test]
    fn request_session_get_legacy_blocklist_enabled() -> Result<()> {
        verify_fields([SessionGetField::BlocklistEnabled], None, ["blocklist-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_blocklist_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::BlocklistEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["blocklist_enabled"])
    }

    #[test]
    fn request_session_get_legacy_blocklist_size() -> Result<()> {
        verify_fields([SessionGetField::BlocklistSize], None, ["blocklist-size"])
    }

    #[test]
    fn request_session_get_semver_600_blocklist_size() -> Result<()> {
        verify_fields(
            [SessionGetField::BlocklistSize],
            Some(JSON_RPC_VERSION_2_0),
            ["blocklist_size"])
    }

    #[test]
    fn request_session_get_legacy_blocklist_url() -> Result<()> {
        verify_fields([SessionGetField::BlocklistUrl], None, ["blocklist-url"])
    }

    #[test]
    fn request_session_get_semver_600_blocklist_url() -> Result<()> {
        verify_fields(
            [SessionGetField::BlocklistUrl],
            Some(JSON_RPC_VERSION_2_0),
            ["blocklist_url"])
    }

    #[test]
    fn request_session_get_legacy_cache_size_mb() -> Result<()> {
        verify_fields([SessionGetField::CacheSizeMb], None, ["cache-size-mb"])
    }

    #[test]
    fn request_session_get_semver_600_cache_size_mb() -> Result<()> {
        verify_fields(
            [SessionGetField::CacheSizeMb],
            Some(JSON_RPC_VERSION_2_0),
            ["cache_size_mib"])
    }

    #[test]
    fn request_session_get_legacy_config_dir() -> Result<()> {
        verify_fields([SessionGetField::ConfigDir], None, ["config-dir"])
    }

    #[test]
    fn request_session_get_semver_600_config_dir() -> Result<()> {
        verify_fields([SessionGetField::ConfigDir], Some(JSON_RPC_VERSION_2_0), ["config_dir"])
    }

    #[test]
    fn request_session_get_legacy_default_trackers() -> Result<()> {
        verify_fields([SessionGetField::DefaultTrackers], None, ["default-trackers"])
    }

    #[test]
    fn request_session_get_semver_600_default_trackers() -> Result<()> {
        verify_fields(
            [SessionGetField::DefaultTrackers],
            Some(JSON_RPC_VERSION_2_0),
            ["default_trackers"])
    }

    #[test]
    fn request_session_get_legacy_dht_enabled() -> Result<()> {
        verify_fields([SessionGetField::DhtEnabled], None, ["dht-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_dht_enabled() -> Result<()> {
        verify_fields([SessionGetField::DhtEnabled], Some(JSON_RPC_VERSION_2_0), ["dht_enabled"])
    }

    #[test]
    fn request_session_get_legacy_download_dir() -> Result<()> {
        verify_fields([SessionGetField::DownloadDir], None, ["download-dir"])
    }

    #[test]
    fn request_session_get_semver_600_download_dir() -> Result<()> {
        verify_fields([SessionGetField::DownloadDir], Some(JSON_RPC_VERSION_2_0), ["download_dir"])
    }

    #[test]
    fn request_session_get_legacy_download_dir_free_space() -> Result<()> {
        verify_fields([SessionGetField::DownloadDirFreeSpace], None, ["download-dir-free-space"])
    }

    #[test]
    fn request_session_get_semver_600_download_dir_free_space() -> Result<()> {
        verify_fields(
            [SessionGetField::DownloadDirFreeSpace],
            Some(JSON_RPC_VERSION_2_0),
            ["download_dir_free_space"])
    }

    #[test]
    fn request_session_get_legacy_download_queue_enabled() -> Result<()> {
        verify_fields([SessionGetField::DownloadQueueEnabled], None, ["download-queue-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_download_queue_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::DownloadQueueEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["download_queue_enabled"])
    }

    #[test]
    fn request_session_get_legacy_download_queue_size() -> Result<()> {
        verify_fields([SessionGetField::DownloadQueueSize], None, ["download-queue-size"])
    }

    #[test]
    fn request_session_get_semver_600_download_queue_size() -> Result<()> {
        verify_fields(
            [SessionGetField::DownloadQueueSize],
            Some(JSON_RPC_VERSION_2_0),
            ["download_queue_size"])
    }

    #[test]
    fn request_session_get_legacy_encryption() -> Result<()> {
        verify_fields([SessionGetField::Encryption], None, ["encryption"])
    }

    #[test]
    fn request_session_get_semver_600_encryption() -> Result<()> {
        verify_fields([SessionGetField::Encryption], Some(JSON_RPC_VERSION_2_0), ["encryption"])
    }

    #[test]
    fn request_session_get_legacy_idle_seeding_limit() -> Result<()> {
        verify_fields([SessionGetField::IdleSeedingLimit], None, ["idle-seeding-limit"])
    }

    #[test]
    fn request_session_get_semver_600_idle_seeding_limit() -> Result<()> {
        verify_fields(
            [SessionGetField::IdleSeedingLimit],
            Some(JSON_RPC_VERSION_2_0),
            ["idle_seeding_limit"])
    }

    #[test]
    fn request_session_get_legacy_idle_seeding_limit_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::IdleSeedingLimitEnabled],
            None,
            ["idle-seeding-limit-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_idle_seeding_limit_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::IdleSeedingLimitEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["idle_seeding_limit_enabled"])
    }

    #[test]
    fn request_session_get_legacy_incomplete_dir() -> Result<()> {
        verify_fields([SessionGetField::IncompleteDir], None, ["incomplete-dir"])
    }

    #[test]
    fn request_session_get_semver_600_incomplete_dir() -> Result<()> {
        verify_fields(
            [SessionGetField::IncompleteDir],
            Some(JSON_RPC_VERSION_2_0),
            ["incomplete_dir"])
    }

    #[test]
    fn request_session_get_legacy_incomplete_dir_enabled() -> Result<()> {
        verify_fields([SessionGetField::IncompleteDirEnabled], None, ["incomplete-dir-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_incomplete_dir_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::IncompleteDirEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["incomplete_dir_enabled"])
    }

    #[test]
    fn request_session_get_legacy_lpd_enabled() -> Result<()> {
        verify_fields([SessionGetField::LpdEnabled], None, ["lpd-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_lpd_enabled() -> Result<()> {
        verify_fields([SessionGetField::LpdEnabled], Some(JSON_RPC_VERSION_2_0), ["lpd_enabled"])
    }

    #[test]
    fn request_session_get_legacy_peer_limit_global() -> Result<()> {
        verify_fields([SessionGetField::PeerLimitGlobal], None, ["peer-limit-global"])
    }

    #[test]
    fn request_session_get_semver_600_peer_limit_global() -> Result<()> {
        verify_fields(
            [SessionGetField::PeerLimitGlobal],
            Some(JSON_RPC_VERSION_2_0),
            ["peer_limit_global"])
    }

    #[test]
    fn request_session_get_legacy_peer_limit_per_torrent() -> Result<()> {
        verify_fields([SessionGetField::PeerLimitPerTorrent], None, ["peer-limit-per-torrent"])
    }

    #[test]
    fn request_session_get_semver_600_peer_limit_per_torrent() -> Result<()> {
        verify_fields(
            [SessionGetField::PeerLimitPerTorrent],
            Some(JSON_RPC_VERSION_2_0),
            ["peer_limit_per_torrent"])
    }

    #[test]
    fn request_session_get_legacy_peer_port_random_on_start() -> Result<()> {
        verify_fields(
            [SessionGetField::PeerPortRandomOnStart],
            None,
            ["peer-port-random-on-start"])
    }

    #[test]
    fn request_session_get_semver_600_peer_port_random_on_start() -> Result<()> {
        verify_fields(
            [SessionGetField::PeerPortRandomOnStart],
            Some(JSON_RPC_VERSION_2_0),
            ["peer_port_random_on_start"])
    }

    #[test]
    fn request_session_get_legacy_peer_port() -> Result<()> {
        verify_fields([SessionGetField::PeerPort], None, ["peer-port"])
    }

    #[test]
    fn request_session_get_semver_600_peer_port() -> Result<()> {
        verify_fields([SessionGetField::PeerPort], Some(JSON_RPC_VERSION_2_0), ["peer_port"])
    }

    #[test]
    fn request_session_get_legacy_pex_enabled() -> Result<()> {
        verify_fields([SessionGetField::PexEnabled], None, ["pex-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_pex_enabled() -> Result<()> {
        verify_fields([SessionGetField::PexEnabled], Some(JSON_RPC_VERSION_2_0), ["pex_enabled"])
    }

    #[test]
    fn request_session_get_legacy_port_forwarding_enabled() -> Result<()> {
        verify_fields([SessionGetField::PortForwardingEnabled], None, ["port-forwarding-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_port_forwarding_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::PortForwardingEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["port_forwarding_enabled"])
    }

    #[test]
    fn request_session_get_legacy_preferred_transports() -> Result<()> {
        verify_fields([SessionGetField::PreferredTransports], None, ["preferred-transports"])
    }

    #[test]
    fn request_session_get_semver_600_preferred_transports() -> Result<()> {
        verify_fields(
            [SessionGetField::PreferredTransports],
            Some(JSON_RPC_VERSION_2_0),
            ["preferred_transports"])
    }

    #[test]
    fn request_session_get_legacy_queue_stalled_enabled() -> Result<()> {
        verify_fields([SessionGetField::QueueStalledEnabled], None, ["queue-stalled-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_queue_stalled_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::QueueStalledEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["queue_stalled_enabled"])
    }

    #[test]
    fn request_session_get_legacy_queue_stalled_minutes() -> Result<()> {
        verify_fields([SessionGetField::QueueStalledMinutes], None, ["queue-stalled-minutes"])
    }

    #[test]
    fn request_session_get_semver_600_queue_stalled_minutes() -> Result<()> {
        verify_fields(
            [SessionGetField::QueueStalledMinutes],
            Some(JSON_RPC_VERSION_2_0),
            ["queue_stalled_minutes"])
    }

    #[test]
    fn request_session_get_legacy_rename_partial_files() -> Result<()> {
        verify_fields([SessionGetField::RenamePartialFiles], None, ["rename-partial-files"])
    }

    #[test]
    fn request_session_get_semver_600_rename_partial_files() -> Result<()> {
        verify_fields(
            [SessionGetField::RenamePartialFiles],
            Some(JSON_RPC_VERSION_2_0),
            ["rename_partial_files"])
    }

    #[test]
    fn request_session_get_legacy_reqq() -> Result<()> {
        verify_fields([SessionGetField::Reqq], None, ["reqq"])
    }

    #[test]
    fn request_session_get_semver_600_reqq() -> Result<()> {
        verify_fields([SessionGetField::Reqq], Some(JSON_RPC_VERSION_2_0), ["reqq"])
    }

    #[test]
    fn request_session_get_legacy_rpc_version_minimum() -> Result<()> {
        verify_fields([SessionGetField::RpcVersionMinimum], None, ["rpc-version-minimum"])
    }

    #[test]
    fn request_session_get_semver_600_rpc_version_minimum() -> Result<()> {
        verify_fields(
            [SessionGetField::RpcVersionMinimum],
            Some(JSON_RPC_VERSION_2_0),
            ["rpc_version_minimum"])
    }

    #[test]
    fn request_session_get_legacy_rpc_version_semver() -> Result<()> {
        verify_fields([SessionGetField::RpcVersionSemver], None, ["rpc-version-semver"])
    }

    #[test]
    fn request_session_get_semver_600_rpc_version_semver() -> Result<()> {
        verify_fields(
            [SessionGetField::RpcVersionSemver],
            Some(JSON_RPC_VERSION_2_0),
            ["rpc_version_semver"])
    }

    #[test]
    fn request_session_get_legacy_rpc_version() -> Result<()> {
        verify_fields([SessionGetField::RpcVersion], None, ["rpc-version"])
    }

    #[test]
    fn request_session_get_semver_600_rpc_version() -> Result<()> {
        verify_fields([SessionGetField::RpcVersion], Some(JSON_RPC_VERSION_2_0), ["rpc_version"])
    }

    #[test]
    fn request_session_get_legacy_script_torrent_added_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentAddedEnabled],
            None,
            ["script-torrent-added-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_added_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentAddedEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["script_torrent_added_enabled"])
    }

    #[test]
    fn request_session_get_legacy_script_torrent_added_filename() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentAddedFilename],
            None,
            ["script-torrent-added-filename"])
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_added_filename() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentAddedFilename],
            Some(JSON_RPC_VERSION_2_0),
            ["script_torrent_added_filename"])
    }

    #[test]
    fn request_session_get_legacy_script_torrent_done_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneEnabled],
            None,
            ["script-torrent-done-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["script_torrent_done_enabled"])
    }

    #[test]
    fn request_session_get_legacy_script_torrent_done_filename() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneFilename],
            None,
            ["script-torrent-done-filename"])
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_filename() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneFilename],
            Some(JSON_RPC_VERSION_2_0),
            ["script_torrent_done_filename"])
    }

    #[test]
    fn request_session_get_legacy_script_torrent_done_seeding_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneSeedingEnabled],
            None,
            ["script-torrent-done-seeding-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_seeding_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneSeedingEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["script_torrent_done_seeding_enabled"])
    }
    #[test]
    fn request_session_get_legacy_script_torrent_done_seeding_filename() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneSeedingFilename],
            None,
            ["script-torrent-done-seeding-filename"])
    }

    #[test]
    fn request_session_get_semver_600_script_torrent_done_seeding_filename() -> Result<()> {
        verify_fields(
            [SessionGetField::ScriptTorrentDoneSeedingFilename],
            Some(JSON_RPC_VERSION_2_0),
            ["script_torrent_done_seeding_filename"])
    }

    #[test]
    fn request_session_get_legacy_seed_queue_enabled() -> Result<()> {
        verify_fields([SessionGetField::SeedQueueEnabled], None, ["seed-queue-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_seed_queue_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::SeedQueueEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_queue_enabled"])
    }

    #[test]
    fn request_session_get_legacy_seed_queue_size() -> Result<()> {
        verify_fields([SessionGetField::SeedQueueSize], None, ["seed-queue-size"])
    }

    #[test]
    fn request_session_get_semver_600_seed_queue_size() -> Result<()> {
        verify_fields(
            [SessionGetField::SeedQueueSize],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_queue_size"])
    }

    #[test]
    fn request_session_get_legacy_seed_ratio_limit() -> Result<()> {
        verify_fields([SessionGetField::SeedRatioLimit], None, ["seedRatioLimit"])
    }

    #[test]
    fn request_session_get_semver_600_seed_ratio_limit() -> Result<()> {
        verify_fields(
            [SessionGetField::SeedRatioLimit],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_ratio_limit"])
    }

    #[test]
    fn request_session_get_legacy_seed_ratio_limited() -> Result<()> {
        verify_fields([SessionGetField::SeedRatioLimited], None, ["seedRatioLimited"])
    }

    #[test]
    fn request_session_get_semver_600_seed_ratio_limited() -> Result<()> {
        verify_fields(
            [SessionGetField::SeedRatioLimited],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_ratio_limited"])
    }

    #[test]
    fn request_session_get_legacy_sequential_download() -> Result<()> {
        verify_fields([SessionGetField::SequentialDownload], None, ["sequential_download"])
    }

    #[test]
    fn request_session_get_semver_600_sequential_download() -> Result<()> {
        verify_fields(
            [SessionGetField::SequentialDownload],
            Some(JSON_RPC_VERSION_2_0),
            ["sequential_download"])
    }

    #[test]
    fn request_session_get_legacy_session_id() -> Result<()> {
        verify_fields([SessionGetField::SessionId], None, ["session-id"])
    }

    #[test]
    fn request_session_get_semver_600_session_id() -> Result<()> {
        verify_fields([SessionGetField::SessionId], Some(JSON_RPC_VERSION_2_0), ["session_id"])
    }

    #[test]
    fn request_session_get_legacy_speed_limit_down_enabled() -> Result<()> {
        verify_fields([SessionGetField::SpeedLimitDownEnabled], None, ["speed-limit-down-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_down_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::SpeedLimitDownEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["speed_limit_down_enabled"])
    }

    #[test]
    fn request_session_get_legacy_speed_limit_down() -> Result<()> {
        verify_fields([SessionGetField::SpeedLimitDown], None, ["speed-limit-down"])
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_down() -> Result<()> {
        verify_fields(
            [SessionGetField::SpeedLimitDown],
            Some(JSON_RPC_VERSION_2_0),
            ["speed_limit_down"])
    }

    #[test]
    fn request_session_get_legacy_speed_limit_up_enabled() -> Result<()> {
        verify_fields([SessionGetField::SpeedLimitUpEnabled], None, ["speed-limit-up-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_up_enabled() -> Result<()> {
        verify_fields(
            [SessionGetField::SpeedLimitUpEnabled],
            Some(JSON_RPC_VERSION_2_0),
            ["speed_limit_up_enabled"])
    }

    #[test]
    fn request_session_get_legacy_speed_limit_up() -> Result<()> {
        verify_fields([SessionGetField::SpeedLimitUp], None, ["speed-limit-up"])
    }

    #[test]
    fn request_session_get_semver_600_speed_limit_up() -> Result<()> {
        verify_fields(
            [SessionGetField::SpeedLimitUp],
            Some(JSON_RPC_VERSION_2_0),
            ["speed_limit_up"])
    }

    #[test]
    fn request_session_get_legacy_start_added_torrents() -> Result<()> {
        verify_fields([SessionGetField::StartAddedTorrents], None, ["start-added-torrents"])
    }

    #[test]
    fn request_session_get_semver_600_start_added_torrents() -> Result<()> {
        verify_fields(
            [SessionGetField::StartAddedTorrents],
            Some(JSON_RPC_VERSION_2_0),
            ["start_added_torrents"])
    }

    #[test]
    fn request_session_get_legacy_tcp_enabled() -> Result<()> {
        verify_fields([SessionGetField::TcpEnabled], None, ["tcp-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_tcp_enabled() -> Result<()> {
        verify_fields([SessionGetField::TcpEnabled], Some(JSON_RPC_VERSION_2_0), ["tcp_enabled"])
    }

    #[test]
    fn request_session_get_legacy_trash_original_torrent_files() -> Result<()> {
        verify_fields(
            [SessionGetField::TrashOriginalTorrentFiles],
            None,
            ["trash-original-torrent-files"])
    }

    #[test]
    fn request_session_get_semver_600_trash_original_torrent_files() -> Result<()> {
        verify_fields(
            [SessionGetField::TrashOriginalTorrentFiles],
            Some(JSON_RPC_VERSION_2_0),
            ["trash_original_torrent_files"])
    }

    #[test]
    fn request_session_get_legacy_units() -> Result<()> {
        verify_fields([SessionGetField::Units], None, ["units"])
    }

    #[test]
    fn request_session_get_semver_600_units() -> Result<()> {
        verify_fields([SessionGetField::Units], Some(JSON_RPC_VERSION_2_0), ["units"])
    }

    #[test]
    fn request_session_get_legacy_utp_enabled() -> Result<()> {
        verify_fields([SessionGetField::UtpEnabled], None, ["utp-enabled"])
    }

    #[test]
    fn request_session_get_semver_600_utp_enabled() -> Result<()> {
        verify_fields([SessionGetField::UtpEnabled], Some(JSON_RPC_VERSION_2_0), ["utp_enabled"])
    }

    #[test]
    fn request_session_get_legacy_version() -> Result<()> {
        verify_fields([SessionGetField::Version], None, ["version"])
    }

    #[test]
    fn request_session_get_semver_600_version() -> Result<()> {
        verify_fields([SessionGetField::Version], Some(JSON_RPC_VERSION_2_0), ["version"])
    }
}
