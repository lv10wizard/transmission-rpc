use compat_macros::GenerateCompat;
use serde::Serialize;

use super::Args;

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
/// [`session_get`]: crate::types::request::RpcRequest::session_get
/// [`SessionGet`]: crate::types::SessionGet
/// [`HashSet`]: std::collections::HashSet
#[derive(GenerateCompat, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SessionGetArgs {
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

impl From<SessionGetArgs> for Args {
    fn from(value: SessionGetArgs) -> Self {
        Args::SessionGet(value)
    }
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SessionGetField {
    AltSpeedDown,
    AltSpeedEnabled,
    AltSpeedTimeBegin,
    AltSpeedTimeDay,
    AltSpeedTimeEnabled,
    AltSpeedTimeEnd,
    AltSpeedUp,
    BlocklistEnabled,
    BlocklistSize,
    BlocklistUrl,
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
