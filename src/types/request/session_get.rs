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
    QueueStalledEnabled,
    QueueStalledMinutes,
    RenamePartialFiles,
    Reqq,
    RpcVersionMinimum,
    RpcVersionSemver,
    RpcVersion,
    Scripttorrentaddedenabled,
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
    #[serde(rename = "sequential_download")]
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
}
