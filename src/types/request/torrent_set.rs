use compat_macros::GenerateCompat;
use serde::Serialize;

use crate::types::{Id, IdleMode, Priority, RatioMode};

use super::TrackerList;

/// Defines request arguments for the [`torrent_set`] method.
///
/// # Constructors
///
/// * [`TorrentSetArgs::default`] creates a new [`TorrentSetArgs`] instance with all fields set to
/// their default value (`None`).
/// * [`TorrentSetArgs::new`] is an alias for [`TorrentSetArgs::default`].
///
/// # Setters
///
/// These methods are fluent setters, returning a new [`TorrentSetArgs`] instance modifying only
/// the corresponding field while leaving all other fields untouched.
///
/// * [`TorrentSetArgs::bandwidth_priority`]: The torrents' bandwidth [`Priority`].
/// * [`TorrentSetArgs::download_limit`]: Maximum download speed (`KBps`).
/// * [`TorrentSetArgs::download_limited`]: `true` to honor `download_limit`.
/// * [`TorrentSetArgs::files_wanted`]: Indices of file(s) to download.
/// * [`TorrentSetArgs::files_unwanted`]: Indices of file(s) to skip (ie, not download).
/// * [`TorrentSetArgs::group`]: The name of the torrents' bandwidth group.
///     > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17).
/// * [`TorrentSetArgs::honors_session_limits`]: `true` to honor the session's upload limits.
/// * [`TorrentSetArgs::labels`]: A `Vec` of `String` labels to set on the torrent(s).
///     > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16).
/// * [`TorrentSetArgs::location`]: The new location of the torrents' content.
/// * [`TorrentSetArgs::peer_limit`]: Maximum number of peers.
/// * [`TorrentSetArgs::priority_high`]: Indices of [`Priority::High`] file(s).
/// * [`TorrentSetArgs::priority_low`]: Indices of [`Priority::Low`] file(s).
/// * [`TorrentSetArgs::priority_normal`]: Indices of [`Priority::Normal`] file(s).
/// * [`TorrentSetArgs::queue_position`]: The new queue position of this torrent `[0..n)`.
/// * [`TorrentSetArgs::seed_idle_limit`]: Torrent-level number of minutes of seeding inactivity
/// before it considered `stalled`.
/// * [`TorrentSetArgs::seed_idle_mode`]: Which seeding inactivity mode ([`IdleMode`]) to use.
/// * [`TorrentSetArgs::seed_ratio_limit`]: Torrent-level seeding ratio.
/// * [`TorrentSetArgs::seed_ratio_mode`]: Which [`RatioMode`] to use.
/// * [`TorrentSetArgs::sequential_download`]: `true` to download the torrent pieces sequentially.
///     > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18).
/// * [`TorrentSetArgs::tracker_add`]: Add a new tracker url in its own new tier.
///     * *NOTE:* This documentation may be incorrect. The rpc-spec itself is unclear.
///     > ⚠ Deprecated in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17);
///     > prefer `tracker_list` if possible.
/// * [`TorrentSetArgs::tracker_list`]: `TrackerList` of announce urls with an empty element
/// between [tiers](https://www.bittorrent.org/beps/bep_0012.html).
///     > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17).
/// * [`TorrentSetArgs::tracker_remove`]: [`Trackers::id`] of trackers to remove.
///     * *NOTE:* This documentation may be incorrect. The rpc-spec itself is unclear.
///     > ⚠ Deprecated in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17);
///     > prefer `tracker_list` if possible.
/// * [`TorrentSetArgs::tracker_replace`]: Pairs of <[`Trackers::id`]/new announce urls>.
///     * *NOTE:* This documentation may be incorrect. The rpc-spec itself is unclear.
///     * See: transmission/transmission
///     [#3226](https://github.com/transmission/transmission/issues/3226#issuecomment-1411899883).
///     > ⚠ Deprecated in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17);
///     > prefer `tracker_list` if possible.
/// * [`TorrentSetArgs::upload_limit`]: Maximum upload speed (`KBps`).
/// * [`TorrentSetArgs::upload_limited`]: `true` to honor `upload_limit`.
///
/// # Examples
///
/// With fluent setters:
/// ```
/// use transmission_rpc::types::TorrentSetArgs;
///
/// let args = TorrentSetArgs()::default()
///                .seed_ratio_limit(12.34)
///                .labels(vec!["foo", "bar"])
///                .locations("/a/b/c/d");
/// ```
///
/// Directly setting struct fields:
/// ```
/// use transmission_rpc::types::TorrentSetArgs;
///
/// let mut args = TorrentSetArgs()::default();
/// args.seed_ratio_limit = Some(12.34);
/// args.labels = Some(vec!["foo", "bar"]);
/// args.locations = Some("/a/b/c/d");
/// ```
///
/// [`torrent_set`]: crate::TransClient::torrent_set
/// [`Trackers::id`]: super::Trackers::id
#[derive(GenerateCompat, Serialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TorrentSetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files-wanted")]
    pub files_wanted: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honors_session_limits: Option<bool>,

    // Don't expose the `ids` field as it is blindly overwritten by `torrent_set`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ids: Option<Vec<Id>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-high")]
    pub priority_high: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-low")]
    pub priority_low: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-normal")]
    pub priority_normal: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_mode: Option<IdleMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_limit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_mode: Option<RatioMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_download: Option<bool>,
    /// Download from a specific piece when [sequential download] is enabled.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [sequential download]: Self::sequential_download
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_download_from_piece: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_list: Option<TrackerList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_remove: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_replace: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limited: Option<bool>,
}

impl TorrentSetArgs {
    /// Creates a new [`TorrentSetArgs`] with all fields set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bandwidth_priority(mut self, bandwidth_priority: Priority) -> Self {
        self.bandwidth_priority = Some(bandwidth_priority);
        self
    }
    pub fn download_limit(mut self, download_limit: usize) -> Self {
        self.download_limit = Some(download_limit);
        self
    }
    pub fn download_limited(mut self, download_limited: bool) -> Self {
        self.download_limited = Some(download_limited);
        self
    }
    pub fn files_wanted(mut self, files_wanted: Vec<usize>) -> Self {
        self.files_wanted = Some(files_wanted);
        self
    }
    pub fn files_unwanted(mut self, files_unwanted: Vec<usize>) -> Self {
        self.files_unwanted = Some(files_unwanted);
        self
    }
    pub fn group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }
    pub fn honors_session_limits(mut self, honors_session_limits: bool) -> Self {
        self.honors_session_limits = Some(honors_session_limits);
        self
    }
    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.labels = Some(labels);
        self
    }
    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
    pub fn peer_limit(mut self, peer_limit: u16) -> Self {
        self.peer_limit = Some(peer_limit);
        self
    }
    pub fn priority_high(mut self, priority_high: Vec<usize>) -> Self {
        self.priority_high = Some(priority_high);
        self
    }
    pub fn priority_low(mut self, priority_low: Vec<usize>) -> Self {
        self.priority_low = Some(priority_low);
        self
    }
    pub fn priority_normal(mut self, priority_normal: Vec<usize>) -> Self {
        self.priority_normal = Some(priority_normal);
        self
    }
    pub fn queue_position(mut self, queue_position: usize) -> Self {
        self.queue_position = Some(queue_position);
        self
    }
    pub fn seed_idle_limit(mut self, seed_idle_limit: u16) -> Self {
        self.seed_idle_limit = Some(seed_idle_limit);
        self
    }
    pub fn seed_idle_mode(mut self, seed_idle_mode: IdleMode) -> Self {
        self.seed_idle_mode = Some(seed_idle_mode);
        self
    }
    pub fn seed_ratio_limit(mut self, seed_ratio_limit: f64) -> Self {
        self.seed_ratio_limit = Some(seed_ratio_limit.into());
        self
    }
    pub fn seed_ratio_mode(mut self, seed_ratio_mode: RatioMode) -> Self {
        self.seed_ratio_mode = Some(seed_ratio_mode);
        self
    }
    pub fn sequential_download(mut self, sequential_download: bool) -> Self {
        self.sequential_download = Some(sequential_download);
        self
    }
    pub fn sequential_download_from_piece(mut self, piece: u64) -> Self {
        self.sequential_download_from_piece = Some(piece);
        self
    }
    pub fn tracker_add(mut self, tracker_add: Vec<String>) -> Self {
        self.tracker_add = Some(tracker_add);
        self
    }
    pub fn tracker_list(mut self, tracker_list: TrackerList) -> Self {
        self.tracker_list = Some(tracker_list);
        self
    }
    pub fn tracker_remove(mut self, tracker_remove: Vec<String>) -> Self {
        self.tracker_remove = Some(tracker_remove);
        self
    }
    pub fn tracker_replace(mut self, tracker_replace: Vec<String>) -> Self {
        self.tracker_replace = Some(tracker_replace);
        self
    }
    pub fn upload_limit(mut self, upload_limit: usize) -> Self {
        self.upload_limit = Some(upload_limit);
        self
    }
    pub fn upload_limited(mut self, upload_limited: bool) -> Self {
        self.upload_limited = Some(upload_limited);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::TorrentSetArgs;

    #[test]
    fn torrent_set_args_setter_group() {
        let args = TorrentSetArgs::default();
        assert_eq!(args.bandwidth_priority, None);
        assert_eq!(args.group, None);
        let args = args.group("test group".to_string());
        assert_eq!(args.bandwidth_priority, None);
        assert_eq!(args.group, Some("test group".to_string()));
    }

    #[test]
    fn torrent_set_args_setter_sequential_download() {
        let args = TorrentSetArgs::default();
        assert_eq!(args.group, None);
        assert_eq!(args.sequential_download, None);
        let args = args.sequential_download(false);
        assert_eq!(args.group, None);
        assert_eq!(args.sequential_download, Some(false));
        let args = args.sequential_download(true);
        assert_eq!(args.group, None);
        assert_eq!(args.sequential_download, Some(true));
    }
}
