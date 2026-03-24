use compat_macros::GenerateCompat;
use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Id, map_option_vec};

#[skip_serializing_none]
#[derive(GenerateCompat, Serialize, Debug, Clone)]
#[compat(placeholder = P)]
pub struct TorrentGetArgs {
    #[compat(type = Option<Vec<P>>, map = map_option_vec)]
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
    #[serde(rename = "bytes_completed")] // (?) Doesn't exist pre- semver-6.0.0
    BytesCompleted,
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
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// "it never worked".
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
    #[serde(rename = "sequential_download")] // Doesn't exist pre- semver-6.0.0
    SequentialDownload,
    #[serde(rename = "sequential_download_from_piece")] // Doesn't exist pre- semver-6.0.0
    SequentialDownloadFromPiece,
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
    /// > ⚠ **DEPRECATED** in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?):
    /// Use [`WebseedsEx`] instead.
    ///
    /// [`WebseedsEx`]: Self::WebseedsEx
    Webseeds,
    #[serde(rename = "webseeds_ex")] // Doesn't exist pre- semver-6.1.0
    WebseedsEx,
    WebseedsSendingToUs,
}

#[cfg(test)]
mod serde_tests {
    use serde_json;

    use crate::types::{
        JSON_RPC_VERSION_2_0,
 Result,
 RpcRequest, request::test_helper::verify
    };
    use super::*;

    fn verify_fields<'x, F, I, X, Y>(
        fields: F,
        ids: I,
        jsonrpc: Option<&str>,
        expected_fields: X,
        expected_ids: Y,
    ) -> Result<()>
    where
        F: IntoIterator<Item = TorrentGetField>,
        I: IntoIterator<Item = Id>,
        X: IntoIterator<Item = &'x str>,
        Y: IntoIterator<Item = i32>,
    {
        let request = {
            let fields: Vec<_> = fields.into_iter().collect();
            let fields = (!fields.is_empty())
                .then_some(fields);
            let ids: Vec<_> = ids.into_iter().collect();
            let ids = (!ids.is_empty())
                .then_some(ids);

            RpcRequest::torrent_get(fields, ids, None)
        };
        let expected_fields: Vec<_> = expected_fields.into_iter().collect();
        let expected_ids = {
            let ids: Vec<_>  = expected_ids.into_iter().collect();
            match ids.is_empty() {
                true => "".to_string(),
                false => format!(",\"ids\":{}", serde_json::to_string(&ids)?),
            }
        };
        let expected = format!("\
            \"fields\":{}\
            {expected_ids}\
        ", serde_json::to_string(&expected_fields)?);
        verify(request, jsonrpc, &expected)
    }

    // -----------------------------------------------------------------

    #[test]
    fn request_torrent_get_legacy_activity_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::ActivityDate], [],
            None,
            ["activityDate"], [])
    }

    #[test]
    fn request_torrent_get_semver_600_activity_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::ActivityDate], [],
            Some(JSON_RPC_VERSION_2_0),
            ["activity_date"], [])
    }

    #[test]
    fn request_torrent_get_legacy_added_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::AddedDate], [Id::Id(123), Id::Id(42), Id::Id(6)],
            None,
            ["addedDate"], [123, 42, 6])
    }

    #[test]
    fn request_torrent_get_semver_600_added_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::AddedDate], [Id::Id(6), Id::Id(123), Id::Id(42)],
            Some(JSON_RPC_VERSION_2_0),
            ["added_date"], [6, 123, 42])
    }

    #[test]
    fn request_torrent_get_legacy_availability() -> Result<()> {
        verify_fields(
            [TorrentGetField::Availability], [Id::Id(3)],
            None,
            ["availability"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_availability() -> Result<()> {
        verify_fields(
            [TorrentGetField::Availability], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["availability"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_bandwidth_priority() -> Result<()> {
        verify_fields(
            [TorrentGetField::BandwidthPriority], [Id::Id(3)],
            None,
            ["bandwidthPriority"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_bandwidth_priority() -> Result<()> {
        verify_fields(
            [TorrentGetField::BandwidthPriority], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["bandwidth_priority"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_bytes_completed() -> Result<()> {
        verify_fields(
            [TorrentGetField::BytesCompleted], [Id::Id(3)],
            None,
            ["bytes_completed"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_bytes_completed() -> Result<()> {
        verify_fields(
            [TorrentGetField::BytesCompleted], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["bytes_completed"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_comment() -> Result<()> {
        verify_fields(
            [TorrentGetField::Comment], [Id::Id(3)],
            None,
            ["comment"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_comment() -> Result<()> {
        verify_fields(
            [TorrentGetField::Comment], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["comment"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_corrupt_ever() -> Result<()> {
        verify_fields(
            [TorrentGetField::CorruptEver], [Id::Id(3)],
            None,
            ["corruptEver"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_corrupt_ever() -> Result<()> {
        verify_fields(
            [TorrentGetField::CorruptEver], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["corrupt_ever"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_creator() -> Result<()> {
        verify_fields(
            [TorrentGetField::Creator], [Id::Id(3)],
            None,
            ["creator"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_creator() -> Result<()> {
        verify_fields(
            [TorrentGetField::Creator], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["creator"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_date_created() -> Result<()> {
        verify_fields(
            [TorrentGetField::DateCreated], [Id::Id(3)],
            None,
            ["dateCreated"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_date_created() -> Result<()> {
        verify_fields(
            [TorrentGetField::DateCreated], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["date_created"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_desired_available() -> Result<()> {
        verify_fields(
            [TorrentGetField::DesiredAvailable], [Id::Id(3)],
            None,
            ["desiredAvailable"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_desired_available() -> Result<()> {
        verify_fields(
            [TorrentGetField::DesiredAvailable], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["desired_available"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_done_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::DoneDate], [Id::Id(3)],
            None,
            ["doneDate"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_done_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::DoneDate], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["done_date"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_download_dir() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadDir], [Id::Id(3)],
            None,
            ["downloadDir"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_download_dir() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadDir], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["download_dir"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_downloaded_ever() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadedEver], [Id::Id(3)],
            None,
            ["downloadedEver"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_downloaded_ever() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadedEver], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["downloaded_ever"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_download_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadLimit], [Id::Id(3)],
            None,
            ["downloadLimit"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_download_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadLimit], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["download_limit"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_download_limited() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadLimited], [Id::Id(3)],
            None,
            ["downloadLimited"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_download_limited() -> Result<()> {
        verify_fields(
            [TorrentGetField::DownloadLimited], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["download_limited"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_edit_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::EditDate], [Id::Id(3)],
            None,
            ["editDate"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_edit_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::EditDate], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["edit_date"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_error() -> Result<()> {
        verify_fields(
            [TorrentGetField::Error], [Id::Id(3)],
            None,
            ["error"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_error() -> Result<()> {
        verify_fields(
            [TorrentGetField::Error], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["error"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_error_string() -> Result<()> {
        verify_fields(
            [TorrentGetField::ErrorString], [Id::Id(3)],
            None,
            ["errorString"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_error_string() -> Result<()> {
        verify_fields(
            [TorrentGetField::ErrorString], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["error_string"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_eta() -> Result<()> {
        verify_fields(
            [TorrentGetField::Eta], [Id::Id(3)],
            None,
            ["eta"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_eta() -> Result<()> {
        verify_fields(
            [TorrentGetField::Eta], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["eta"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_eta_idle() -> Result<()> {
        verify_fields(
            [TorrentGetField::EtaIdle], [Id::Id(3)],
            None,
            ["etaIdle"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_eta_idle() -> Result<()> {
        verify_fields(
            [TorrentGetField::EtaIdle], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["eta_idle"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_file_count() -> Result<()> {
        verify_fields(
            [TorrentGetField::FileCount], [Id::Id(3)],
            None,
            ["file-count"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_file_count() -> Result<()> {
        verify_fields(
            [TorrentGetField::FileCount], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["file_count"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_files() -> Result<()> {
        verify_fields(
            [TorrentGetField::Files], [Id::Id(3)],
            None,
            ["files"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_files() -> Result<()> {
        verify_fields(
            [TorrentGetField::Files], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["files"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_file_stats() -> Result<()> {
        verify_fields(
            [TorrentGetField::FileStats], [Id::Id(3)],
            None,
            ["fileStats"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_file_stats() -> Result<()> {
        verify_fields(
            [TorrentGetField::FileStats], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["file_stats"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_group() -> Result<()> {
        verify_fields(
            [TorrentGetField::Group], [Id::Id(3)],
            None,
            ["group"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_group() -> Result<()> {
        verify_fields(
            [TorrentGetField::Group], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["group"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_hash_string() -> Result<()> {
        verify_fields(
            [TorrentGetField::HashString], [Id::Id(3)],
            None,
            ["hashString"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_hash_string() -> Result<()> {
        verify_fields(
            [TorrentGetField::HashString], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["hash_string"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_have_unchecked() -> Result<()> {
        verify_fields(
            [TorrentGetField::HaveUnchecked], [Id::Id(3)],
            None,
            ["haveUnchecked"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_have_unchecked() -> Result<()> {
        verify_fields(
            [TorrentGetField::HaveUnchecked], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["have_unchecked"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_have_valid() -> Result<()> {
        verify_fields(
            [TorrentGetField::HaveValid], [Id::Id(3)],
            None,
            ["haveValid"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_have_valid() -> Result<()> {
        verify_fields(
            [TorrentGetField::HaveValid], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["have_valid"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_honors_session_limits() -> Result<()> {
        verify_fields(
            [TorrentGetField::HonorsSessionLimits], [Id::Id(3)],
            None,
            ["honorsSessionLimits"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_honors_session_limits() -> Result<()> {
        verify_fields(
            [TorrentGetField::HonorsSessionLimits], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["honors_session_limits"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_id() -> Result<()> {
        verify_fields(
            [TorrentGetField::Id], [Id::Id(3)],
            None,
            ["id"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_id() -> Result<()> {
        verify_fields(
            [TorrentGetField::Id], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["id"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_is_finished() -> Result<()> {
        verify_fields(
            [TorrentGetField::IsFinished], [Id::Id(3)],
            None,
            ["isFinished"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_is_finished() -> Result<()> {
        verify_fields(
            [TorrentGetField::IsFinished], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["is_finished"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_is_private() -> Result<()> {
        verify_fields(
            [TorrentGetField::IsPrivate], [Id::Id(3)],
            None,
            ["isPrivate"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_is_private() -> Result<()> {
        verify_fields(
            [TorrentGetField::IsPrivate], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["is_private"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_is_stalled() -> Result<()> {
        verify_fields(
            [TorrentGetField::IsStalled], [Id::Id(3)],
            None,
            ["isStalled"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_is_stalled() -> Result<()> {
        verify_fields(
            [TorrentGetField::IsStalled], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["is_stalled"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_labels() -> Result<()> {
        verify_fields(
            [TorrentGetField::Labels], [Id::Id(3)],
            None,
            ["labels"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_labels() -> Result<()> {
        verify_fields(
            [TorrentGetField::Labels], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["labels"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_left_until_done() -> Result<()> {
        verify_fields(
            [TorrentGetField::LeftUntilDone], [Id::Id(3)],
            None,
            ["leftUntilDone"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_left_until_done() -> Result<()> {
        verify_fields(
            [TorrentGetField::LeftUntilDone], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["left_until_done"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_magnet_link() -> Result<()> {
        verify_fields(
            [TorrentGetField::MagnetLink], [Id::Id(3)],
            None,
            ["magnetLink"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_magnet_link() -> Result<()> {
        verify_fields(
            [TorrentGetField::MagnetLink], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["magnet_link"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_manual_announce_time() -> Result<()> {
        verify_fields(
            [TorrentGetField::ManualAnnounceTime], [Id::Id(3)],
            None,
            ["manualAnnounceTime"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_manual_announce_time() -> Result<()> {
        verify_fields(
            [TorrentGetField::ManualAnnounceTime], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["manual_announce_time"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_max_connected_peers() -> Result<()> {
        verify_fields(
            [TorrentGetField::MaxConnectedPeers], [Id::Id(3)],
            None,
            ["maxConnectedPeers"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_max_connected_peers() -> Result<()> {
        verify_fields(
            [TorrentGetField::MaxConnectedPeers], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["max_connected_peers"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_metadata_percent_complete() -> Result<()> {
        verify_fields(
            [TorrentGetField::MetadataPercentComplete], [Id::Id(3)],
            None,
            ["metadataPercentComplete"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_metadata_percent_complete() -> Result<()> {
        verify_fields(
            [TorrentGetField::MetadataPercentComplete], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["metadata_percent_complete"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_name() -> Result<()> {
        verify_fields(
            [TorrentGetField::Name], [Id::Id(3)],
            None,
            ["name"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_name() -> Result<()> {
        verify_fields(
            [TorrentGetField::Name], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["name"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_peer_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeerLimit], [Id::Id(3)],
            None,
            ["peer-limit"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_peer_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeerLimit], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["peer_limit"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_peers() -> Result<()> {
        verify_fields(
            [TorrentGetField::Peers], [Id::Id(3)],
            None,
            ["peers"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_peers() -> Result<()> {
        verify_fields(
            [TorrentGetField::Peers], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["peers"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_peers_connected() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersConnected], [Id::Id(3)],
            None,
            ["peersConnected"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_peers_connected() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersConnected], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["peers_connected"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_peers_from() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersFrom], [Id::Id(3)],
            None,
            ["peersFrom"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_peers_from() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersFrom], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["peers_from"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_peers_getting_from_us() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersGettingFromUs], [Id::Id(3)],
            None,
            ["peersGettingFromUs"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_peers_getting_from_us() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersGettingFromUs], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["peers_getting_from_us"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_peers_sending_to_us() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersSendingToUs], [Id::Id(3)],
            None,
            ["peersSendingToUs"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_peers_sending_to_us() -> Result<()> {
        verify_fields(
            [TorrentGetField::PeersSendingToUs], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["peers_sending_to_us"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_percent_complete() -> Result<()> {
        verify_fields(
            [TorrentGetField::PercentComplete], [Id::Id(3)],
            None,
            ["percentComplete"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_percent_complete() -> Result<()> {
        verify_fields(
            [TorrentGetField::PercentComplete], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["percent_complete"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_percent_done() -> Result<()> {
        verify_fields(
            [TorrentGetField::PercentDone], [Id::Id(3)],
            None,
            ["percentDone"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_percent_done() -> Result<()> {
        verify_fields(
            [TorrentGetField::PercentDone], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["percent_done"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_pieces() -> Result<()> {
        verify_fields(
            [TorrentGetField::Pieces], [Id::Id(3)],
            None,
            ["pieces"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_pieces() -> Result<()> {
        verify_fields(
            [TorrentGetField::Pieces], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["pieces"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_piece_count() -> Result<()> {
        verify_fields(
            [TorrentGetField::PieceCount], [Id::Id(3)],
            None,
            ["pieceCount"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_piece_count() -> Result<()> {
        verify_fields(
            [TorrentGetField::PieceCount], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["piece_count"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_piece_size() -> Result<()> {
        verify_fields(
            [TorrentGetField::PieceSize], [Id::Id(3)],
            None,
            ["pieceSize"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_piece_size() -> Result<()> {
        verify_fields(
            [TorrentGetField::PieceSize], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["piece_size"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_priorities() -> Result<()> {
        verify_fields(
            [TorrentGetField::Priorities], [Id::Id(3)],
            None,
            ["priorities"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_priorities() -> Result<()> {
        verify_fields(
            [TorrentGetField::Priorities], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["priorities"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_primary_mime_type() -> Result<()> {
        verify_fields(
            [TorrentGetField::PrimaryMimeType], [Id::Id(3)],
            None,
            ["primary-mime-type"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_primary_mime_type() -> Result<()> {
        verify_fields(
            [TorrentGetField::PrimaryMimeType], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["primary_mime_type"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_queue_position() -> Result<()> {
        verify_fields(
            [TorrentGetField::QueuePosition], [Id::Id(3)],
            None,
            ["queuePosition"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_queue_position() -> Result<()> {
        verify_fields(
            [TorrentGetField::QueuePosition], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["queue_position"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_rate_download() -> Result<()> {
        verify_fields(
            [TorrentGetField::RateDownload], [Id::Id(3)],
            None,
            ["rateDownload"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_rate_download() -> Result<()> {
        verify_fields(
            [TorrentGetField::RateDownload], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["rate_download"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_rate_upload() -> Result<()> {
        verify_fields(
            [TorrentGetField::RateUpload], [Id::Id(3)],
            None,
            ["rateUpload"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_rate_upload() -> Result<()> {
        verify_fields(
            [TorrentGetField::RateUpload], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["rate_upload"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_recheck_progress() -> Result<()> {
        verify_fields(
            [TorrentGetField::RecheckProgress], [Id::Id(3)],
            None,
            ["recheckProgress"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_recheck_progress() -> Result<()> {
        verify_fields(
            [TorrentGetField::RecheckProgress], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["recheck_progress"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_seconds_downloading() -> Result<()> {
        verify_fields(
            [TorrentGetField::SecondsDownloading], [Id::Id(3)],
            None,
            ["secondsDownloading"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_seconds_downloading() -> Result<()> {
        verify_fields(
            [TorrentGetField::SecondsDownloading], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["seconds_downloading"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_seconds_seeding() -> Result<()> {
        verify_fields(
            [TorrentGetField::SecondsSeeding], [Id::Id(3)],
            None,
            ["secondsSeeding"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_seconds_seeding() -> Result<()> {
        verify_fields(
            [TorrentGetField::SecondsSeeding], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["seconds_seeding"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_seed_idle_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedIdleLimit], [Id::Id(3)],
            None,
            ["seedIdleLimit"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_seed_idle_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedIdleLimit], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_idle_limit"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_seed_idle_mode() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedIdleMode], [Id::Id(3)],
            None,
            ["seedIdleMode"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_seed_idle_mode() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedIdleMode], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_idle_mode"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_seed_ratio_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedRatioLimit], [Id::Id(3)],
            None,
            ["seedRatioLimit"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_seed_ratio_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedRatioLimit], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_ratio_limit"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_seed_ratio_mode() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedRatioMode], [Id::Id(3)],
            None,
            ["seedRatioMode"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_seed_ratio_mode() -> Result<()> {
        verify_fields(
            [TorrentGetField::SeedRatioMode], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["seed_ratio_mode"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_sequential_download() -> Result<()> {
        verify_fields(
            [TorrentGetField::SequentialDownload], [Id::Id(3)],
            None,
            ["sequential_download"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_sequential_download() -> Result<()> {
        verify_fields(
            [TorrentGetField::SequentialDownload], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["sequential_download"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_sequential_download_from_piece() -> Result<()> {
        verify_fields(
            [TorrentGetField::SequentialDownloadFromPiece], [Id::Id(3)],
            None,
            ["sequential_download_from_piece"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_sequential_download_from_piece() -> Result<()> {
        verify_fields(
            [TorrentGetField::SequentialDownloadFromPiece], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["sequential_download_from_piece"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_size_when_done() -> Result<()> {
        verify_fields(
            [TorrentGetField::SizeWhenDone], [Id::Id(3)],
            None,
            ["sizeWhenDone"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_size_when_done() -> Result<()> {
        verify_fields(
            [TorrentGetField::SizeWhenDone], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["size_when_done"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_start_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::StartDate], [Id::Id(3)],
            None,
            ["startDate"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_start_date() -> Result<()> {
        verify_fields(
            [TorrentGetField::StartDate], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["start_date"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_status() -> Result<()> {
        verify_fields(
            [TorrentGetField::Status], [Id::Id(3)],
            None,
            ["status"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_status() -> Result<()> {
        verify_fields(
            [TorrentGetField::Status], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["status"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_torrent_file() -> Result<()> {
        verify_fields(
            [TorrentGetField::TorrentFile], [Id::Id(3)],
            None,
            ["torrentFile"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_torrent_file() -> Result<()> {
        verify_fields(
            [TorrentGetField::TorrentFile], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["torrent_file"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_total_size() -> Result<()> {
        verify_fields(
            [TorrentGetField::TotalSize], [Id::Id(3)],
            None,
            ["totalSize"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_total_size() -> Result<()> {
        verify_fields(
            [TorrentGetField::TotalSize], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["total_size"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_trackers() -> Result<()> {
        verify_fields(
            [TorrentGetField::Trackers], [Id::Id(3)],
            None,
            ["trackers"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_trackers() -> Result<()> {
        verify_fields(
            [TorrentGetField::Trackers], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["trackers"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_tracker_list() -> Result<()> {
        verify_fields(
            [TorrentGetField::TrackerList], [Id::Id(3)],
            None,
            ["trackerList"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_tracker_list() -> Result<()> {
        verify_fields(
            [TorrentGetField::TrackerList], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["tracker_list"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_tracker_stats() -> Result<()> {
        verify_fields(
            [TorrentGetField::TrackerStats], [Id::Id(3)],
            None,
            ["trackerStats"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_tracker_stats() -> Result<()> {
        verify_fields(
            [TorrentGetField::TrackerStats], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["tracker_stats"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_uploaded_ever() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadedEver], [Id::Id(3)],
            None,
            ["uploadedEver"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_uploaded_ever() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadedEver], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["uploaded_ever"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_upload_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadLimit], [Id::Id(3)],
            None,
            ["uploadLimit"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_upload_limit() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadLimit], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["upload_limit"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_upload_limited() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadLimited], [Id::Id(3)],
            None,
            ["uploadLimited"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_upload_limited() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadLimited], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["upload_limited"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_upload_ratio() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadRatio], [Id::Id(3)],
            None,
            ["uploadRatio"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_upload_ratio() -> Result<()> {
        verify_fields(
            [TorrentGetField::UploadRatio], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["upload_ratio"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_wanted() -> Result<()> {
        verify_fields(
            [TorrentGetField::Wanted], [Id::Id(3)],
            None,
            ["wanted"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_wanted() -> Result<()> {
        verify_fields(
            [TorrentGetField::Wanted], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["wanted"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_webseeds() -> Result<()> {
        verify_fields(
            [TorrentGetField::Webseeds], [Id::Id(3)],
            None,
            ["webseeds"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_webseeds() -> Result<()> {
        verify_fields(
            [TorrentGetField::Webseeds], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["webseeds"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_webseeds_ex() -> Result<()> {
        verify_fields(
            [TorrentGetField::WebseedsEx], [Id::Id(3)],
            None,
            ["webseeds_ex"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_webseeds_ex() -> Result<()> {
        verify_fields(
            [TorrentGetField::WebseedsEx], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["webseeds_ex"], [3])
    }

    #[test]
    fn request_torrent_get_legacy_webseeds_sending_to_us() -> Result<()> {
        verify_fields(
            [TorrentGetField::WebseedsSendingToUs], [Id::Id(3)],
            None,
            ["webseedsSendingToUs"], [3])
    }

    #[test]
    fn request_torrent_get_semver_600_webseeds_sending_to_us() -> Result<()> {
        verify_fields(
            [TorrentGetField::WebseedsSendingToUs], [Id::Id(3)],
            Some(JSON_RPC_VERSION_2_0),
            ["webseeds_sending_to_us"], [3])
    }
}
