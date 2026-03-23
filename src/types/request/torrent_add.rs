use compat_macros::GenerateCompat;
use serde::Serialize;

use crate::types::Priority;

#[derive(GenerateCompat, Serialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TorrentAddArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "download-dir")]
    pub download_dir: Option<String>,
    /// Either "filename" OR "metainfo" MUST be included
    /// semi-optional
    /// filename or URL of the .torrent file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// semi-optional
    /// base64-encoded .torrent content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "peer-limit")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<Priority>,
    /// list of indices of files to be downloaded
    /// to ignore some files, put their indices in files_unwanted, otherwise
    /// they will still be downloaded
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-wanted")]
    pub files_wanted: Option<Vec<i32>>,
    /// list of indices of files not to download
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with high priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-high")]
    pub priority_high: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with low priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-low")]
    pub priority_low: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with normal priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-normal")]
    pub priority_normal: Option<Vec<i32>>,
    /// whether to download torrent pieces sequentially
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_download: Option<bool>,
}

