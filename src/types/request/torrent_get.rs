use compat_macros::GenerateCompat;
use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Id, map_option_vec};

#[derive(GenerateCompat, Serialize, Debug, Clone)]
#[skip_serializing_none]
pub struct TorrentGetArgs {
    #[compat(type = Option<Vec<__semver_600_compat_TorrentGetField>>, map = map_option_vec)]
    pub(crate) fields: Option<Vec<TorrentGetField>>,
    pub(crate) ids: Option<Vec<Id>>,
}

impl Default for TorrentGetArgs {
    fn default() -> Self {
        let all_fields = all::<TorrentGetField>().collect();
        TorrentGetArgs {
            fields: Some(all_fields),
            ids: None,
        }
    }
}

impl<F, I> From<(F, I)> for TorrentGetArgs
where
    F: IntoIterator<Item = TorrentGetField>,
    I: IntoIterator<Item = Id>,
{
    fn from(args: (F, I)) -> Self {
        let fields: Vec<_> = args.0.into_iter().collect();
        Self {
            fields: match fields.is_empty() {
                true => None,
                false => Some(fields),
            },
            ids: Some(args.1.into_iter().collect()),
        }
    }
}

#[derive(
    GenerateCompat,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Sequence
)]
#[serde(rename_all = "camelCase")]
pub enum TorrentGetField {
    ActivityDate,
    AddedDate,
    Availability,
    BandwidthPriority,
    Comment,
    CorruptEver,
    Creator,
    DateCreated,
    DesiredAvailable,
    DoneDate,
    DownloadDir,
    DownloadedEver,
    DownloadLimit,
    DownloadLimited,
    EditDate,
    Error,
    ErrorString,
    Eta,
    EtaIdle,
    #[serde(rename = "file-count")]
    FileCount,
    FileStats,
    Files,
    Group,
    HashString,
    HaveUnchecked,
    HaveValid,
    HonorsSessionLimits,
    Id,
    IsFinished,
    IsPrivate,
    IsStalled,
    Labels,
    LeftUntilDone,
    MagnetLink,
    ManualAnnounceTime,
    MaxConnectedPeers,
    MetadataPercentComplete,
    Name,
    #[serde(rename = "peer-limit")]
    PeerLimit,
    Peers,
    PeersConnected,
    PeersFrom,
    PeersGettingFromUs,
    PeersSendingToUs,
    PercentComplete,
    PercentDone,
    Pieces,
    PieceCount,
    PieceSize,
    Priorities,
    #[serde(rename = "primary-mime-type")]
    PrimaryMimeType,
    QueuePosition,
    RateDownload,
    RateUpload,
    RecheckProgress,
    SecondsDownloading,
    SecondsSeeding,
    SeedIdleLimit,
    SeedIdleMode,
    SeedRatioLimit,
    SeedRatioMode,
    SequentialDownload,
    SizeWhenDone,
    StartDate,
    Status,
    TorrentFile,
    TotalSize,
    Trackers,
    TrackerList,
    TrackerStats,
    UploadRatio,
    UploadedEver,
    UploadLimit,
    UploadLimited,
    Wanted,
    Webseeds,
    WebseedsSendingToUs,
}
