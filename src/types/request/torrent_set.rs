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
    /// The torrent's bandwidth priority.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_priority: Option<Priority>,
    /// Maximum download speed (kB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limit: Option<usize>,
    /// True if [`download_limit`] is honored.
    ///
    /// [`download_limit`]: Self::download_limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limited: Option<bool>,
    /// Indices of file(s) to not download.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<usize>>,
    /// Indices of file(s) to download.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files-wanted")]
    pub files_wanted: Option<Vec<usize>>,
    /// The name of this torrent's bandwidth group.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// True if session upload limits are honored.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honors_session_limits: Option<bool>,

    // Don't expose the `ids` field as it is blindly overwritten by `torrent_set`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ids: Option<Vec<Id>>,

    /// Array of user-created string labels.
    ///
    /// > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// New location of the torrent's content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Maximum number of peers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<u16>,
    /// Indices of high-priority file(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-high")]
    pub priority_high: Option<Vec<usize>>,
    /// Indices of low-priority file(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-low")]
    pub priority_low: Option<Vec<usize>>,
    /// Indices of normal-priority file(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-normal")]
    pub priority_normal: Option<Vec<usize>>,
    /// Position of this torrent in the queue [0, n).
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<usize>,
    /// Torrent-level number of minutes of seeding inactivity.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_limit: Option<u16>,
    /// Which seeding inactivity to use.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_mode: Option<IdleMode>,

    /// Torrent-level seeding ratio.
    ///
    /// > Added in Transmission 1.60 (`rpc-version-semver` 2.0.0, `rpc-version`: 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_limit: Option<f64>,

    /// Which ratio to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_mode: Option<RatioMode>,
    /// Download torrent pieces sequentially.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download: Option<bool>,
    /// Download from a specific piece when [sequential download] is enabled.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [sequential download]: Self::sequential_download
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download_from_piece: Option<u64>,
    /// Strings of announce URLs to add.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17):
    /// Use [`tracker_list`] instead.
    ///
    /// [`tracker_list`]: Self::tracker_list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_add: Option<Vec<String>>,
    /// String of announce URLs, one per line, and a blank line between tiers.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_list: Option<TrackerList>,
    /// Ids of trackers to remove.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17):
    /// Use [`tracker_list`] instead.
    ///
    /// [`tracker_list`]: Self::tracker_list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_remove: Option<Vec<String>>,
    /// Pairs of <trackerId/new announce URLs>.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17):
    /// Use [`tracker_list`] instead.
    ///
    /// [`tracker_list`]: Self::tracker_list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_replace: Option<Vec<String>>,
    /// Maximum upload speed (KBps).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<usize>,
    /// True if [`upload_limit`] is honored.
    ///
    /// [`upload_limit`]: Self::upload_limit
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

#[cfg(test)]
mod serde_tests {
    use crate::types::{JSON_RPC_VERSION_2_0, Result, request::test_helper::verify};
    use super::*;

    #[test]
    fn request_torrent_set_legacy_bandwidth_priority() -> Result<()> {
        let args = TorrentSetArgs::new()
            .bandwidth_priority(Priority::High);
        verify(args, None, "\"bandwidthPriority\":1")
    }

    #[test]
    fn request_torrent_set_semver_600_bandwidth_priority() -> Result<()> {
        let args = TorrentSetArgs::new()
            .bandwidth_priority(Priority::Low);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"bandwidth_priority\":-1")
    }

    #[test]
    fn request_torrent_set_legacy_download_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .download_limit(1000);
        verify(args, None, "\"downloadLimit\":1000")
    }

    #[test]
    fn request_torrent_set_semver_600_download_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .download_limit(500);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"download_limit\":500")
    }

    #[test]
    fn request_torrent_set_legacy_download_limited() -> Result<()> {
        let args = TorrentSetArgs::new()
            .download_limited(true);
        verify(args, None, "\"downloadLimited\":true")
    }

    #[test]
    fn request_torrent_set_semver_600_download_limited() -> Result<()> {
        let args = TorrentSetArgs::new()
            .download_limited(false);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"download_limited\":false")
    }

    #[test]
    fn request_torrent_set_legacy_files_unwanted() -> Result<()> {
        let args = TorrentSetArgs::new()
            .files_unwanted(vec![1,23,4]);
        verify(args, None, "\"files-unwanted\":[1,23,4]")
    }

    #[test]
    fn request_torrent_set_semver_600_files_unwanted() -> Result<()> {
        let args = TorrentSetArgs::new()
            .files_unwanted(vec![5]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"files_unwanted\":[5]")
    }

    #[test]
    fn request_torrent_set_legacy_files_wanted() -> Result<()> {
        let args = TorrentSetArgs::new()
            .files_wanted(vec![99]);
        verify(args, None, "\"files-wanted\":[99]")
    }

    #[test]
    fn request_torrent_set_semver_600_files_wanted() -> Result<()> {
        let args = TorrentSetArgs::new()
            .files_wanted(vec![]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"files_wanted\":[]")
    }

    #[test]
    fn request_torrent_set_legacy_group() -> Result<()> {
        let args = TorrentSetArgs::new()
            .group("gogogo".into());
        verify(args, None, "\"group\":\"gogogo\"")
    }

    #[test]
    fn request_torrent_set_semver_600_group() -> Result<()> {
        let args = TorrentSetArgs::new()
            .group("foobar!".into());
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"group\":\"foobar!\"")
    }

    #[test]
    fn request_torrent_set_legacy_honors_session_limits() -> Result<()> {
        let args = TorrentSetArgs::new()
            .honors_session_limits(true);
        verify(args, None, "\"honorsSessionLimits\":true")
    }

    #[test]
    fn request_torrent_set_semver_600_honors_session_limits() -> Result<()> {
        let args = TorrentSetArgs::new()
            .honors_session_limits(false);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"honors_session_limits\":false")
    }

    #[test]
    fn request_torrent_set_legacy_labels() -> Result<()> {
        let args = TorrentSetArgs::new()
            .labels(vec![]);
        verify(args, None, "\"labels\":[]")
    }

    #[test]
    fn request_torrent_set_semver_600_labels() -> Result<()> {
        let args = TorrentSetArgs::new()
            .labels(vec!["foo".into(), "bar".into()]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"labels\":[\"foo\",\"bar\"]")
    }

    #[test]
    fn request_torrent_set_legacy_location() -> Result<()> {
        let args = TorrentSetArgs::new()
            .location("/incomplete".into());
        verify(args, None, "\"location\":\"/incomplete\"")
    }

    #[test]
    fn request_torrent_set_semver_600_location() -> Result<()> {
        let args = TorrentSetArgs::new()
            .location("/tmp/foo".into());
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"location\":\"/tmp/foo\"")
    }

    #[test]
    fn request_torrent_set_legacy_peer_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .peer_limit(500);
        verify(args, None, "\"peer-limit\":500")
    }

    #[test]
    fn request_torrent_set_semver_600_peer_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .peer_limit(6);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"peer_limit\":6")
    }

    #[test]
    fn request_torrent_set_legacy_priority_high() -> Result<()> {
        let args = TorrentSetArgs::new()
            .priority_high(vec![1, 2, 3]);
        verify(args, None, "\"priority-high\":[1,2,3]")
    }

    #[test]
    fn request_torrent_set_semver_600_priority_high() -> Result<()> {
        let args = TorrentSetArgs::new()
            .priority_high(vec![]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"priority_high\":[]")
    }

    #[test]
    fn request_torrent_set_legacy_priority_low() -> Result<()> {
        let args = TorrentSetArgs::new()
            .priority_low(vec![]);
        verify(args, None, "\"priority-low\":[]")
    }

    #[test]
    fn request_torrent_set_semver_600_priority_low() -> Result<()> {
        let args = TorrentSetArgs::new()
            .priority_low(vec![6, 50, 99]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"priority_low\":[6,50,99]")
    }

    #[test]
    fn request_torrent_set_legacy_priority_normal() -> Result<()> {
        let args = TorrentSetArgs::new()
            .priority_normal(vec![101, 200, 444, 445, 600]);
        verify(args, None, "\"priority-normal\":[101,200,444,445,600]")
    }

    #[test]
    fn request_torrent_set_semver_600_priority_normal() -> Result<()> {
        let args = TorrentSetArgs::new()
            .priority_normal(vec![6, 50, 99]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"priority_normal\":[6,50,99]")
    }

    #[test]
    fn request_torrent_set_legacy_queue_position() -> Result<()> {
        let args = TorrentSetArgs::new()
            .queue_position(0);
        verify(args, None, "\"queuePosition\":0")
    }

    #[test]
    fn request_torrent_set_semver_600_queue_position() -> Result<()> {
        let args = TorrentSetArgs::new()
            .queue_position(4);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"queue_position\":4")
    }

    #[test]
    fn request_torrent_set_legacy_seed_idle_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_idle_limit(10);
        verify(args, None, "\"seedIdleLimit\":10")
    }

    #[test]
    fn request_torrent_set_semver_600_seed_idle_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_idle_limit(300);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"seed_idle_limit\":300")
    }

    #[test]
    fn request_torrent_set_legacy_seed_idle_mode() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_idle_mode(IdleMode::Global);
        verify(args, None, "\"seedIdleMode\":0")
    }

    #[test]
    fn request_torrent_set_semver_600_seed_idle_mode() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_idle_mode(IdleMode::Unlimited);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"seed_idle_mode\":2")
    }

    #[test]
    fn request_torrent_set_legacy_seed_ratio_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_ratio_limit(0.5);
        verify(args, None, "\"seedRatioLimit\":0.5")
    }

    #[test]
    fn request_torrent_set_semver_600_seed_ratio_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_ratio_limit(2.25);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"seed_ratio_limit\":2.25")
    }

    #[test]
    fn request_torrent_set_legacy_seed_ratio_mode() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_ratio_mode(RatioMode::Single);
        verify(args, None, "\"seedRatioMode\":1")
    }

    #[test]
    fn request_torrent_set_semver_600_seed_ratio_mode() -> Result<()> {
        let args = TorrentSetArgs::new()
            .seed_ratio_mode(RatioMode::Global);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"seed_ratio_mode\":0")
    }

    #[test]
    fn request_torrent_set_legacy_sequential_download() -> Result<()> {
        let args = TorrentSetArgs::new()
            .sequential_download(true);
        verify(args, None, "")
    }

    #[test]
    fn request_torrent_set_semver_600_sequential_download() -> Result<()> {
        let args = TorrentSetArgs::new()
            .sequential_download(false);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"sequential_download\":false")
    }

    #[test]
    fn request_torrent_set_legacy_sequential_download_from_piece() -> Result<()> {
        let args = TorrentSetArgs::new()
            .sequential_download_from_piece(123);
        verify(args, None, "")
    }

    #[test]
    fn request_torrent_set_semver_600_sequential_download_from_piece() -> Result<()> {
        let args = TorrentSetArgs::new()
            .sequential_download_from_piece(606);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"sequential_download_from_piece\":606")
    }

    #[test]
    fn request_torrent_set_legacy_tracker_add() -> Result<()> {
        let args = TorrentSetArgs::new()
            .tracker_add(vec![
                "https://example.com:6969/a".into(),
                "https://example.com:2001/a".into(),
            ]);
        verify(args, None, 
            "\"trackerAdd\":[\
                \"https://example.com:6969/a\",\
                \"https://example.com:2001/a\"\
            ]")
    }

    #[test]
    fn request_torrent_set_semver_600_tracker_add() -> Result<()> {
        let args = TorrentSetArgs::new()
            .tracker_add(vec![]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"tracker_add\":[]")
    }

    /* TODO: tracker_list tests
    #[test]
    fn request_torrent_set_legacy_tracker_list() -> Result<()> {
        let args = TorrentSetArgs::new()
            // FIXME: tiers, trailing '\n'
            .tracker_list(TrackerList(vec![
                "https://a.example.org:1111/foo".into(),
                "https://b.example.org:6012/bar".into(),
            ]));
        verify(args, None, 
            "\"trackerList\":\"\
                \"https://a.example.org:1111/foo\"\n\
                \"https://b.example.org:6012/bar\"\n\
            \"")
    }

    #[test]
    fn request_torrent_set_semver_600_tracker_list() -> Result<()> {
        let args = TorrentSetArgs::new()
            // FIXME: tiers, trailing '\n'
            .tracker_list(TrackerList(vec![]));
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"tracker_list\":\"\n\"")
    }
    */

    /* TODO: tracker_remove tests
    #[test]
    fn request_torrent_set_legacy_tracker_remove() -> Result<()> {
        let args = TorrentSetArgs::new()
            .tracker_remove(todo!());
        verify(args, None, 
            "\"trackerRemove\":\"\
                \"https://a.example.org:1111/foo\"\n\
                \"https://b.example.org:6012/bar\"\n\
            \"")
    }

    #[test]
    fn request_torrent_set_semver_600_tracker_remove() -> Result<()> {
        let args = TorrentSetArgs::new()
            .tracker_remove(todo!());
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"tracker_remove\":\"\n\"")
    }
    */

    /* TODO: tracker_replace tests
    #[test]
    fn request_torrent_set_legacy_tracker_replace() -> Result<()> {
        let args = TorrentSetArgs::new()
            .tracker_replace(todo!());
        verify(args, None, 
            "\"trackerReplace\":\"\
            \"")
    }

    #[test]
    fn request_torrent_set_semver_600_tracker_replace() -> Result<()> {
        let args = TorrentSetArgs::new()
            .tracker_replace(todo!());
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"tracker_replace\":\"\n\"")
    }
    */

    #[test]
    fn request_torrent_set_legacy_upload_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .upload_limit(5000);
        verify(args, None, "\"uploadLimit\":5000")
    }

    #[test]
    fn request_torrent_set_semver_600_upload_limit() -> Result<()> {
        let args = TorrentSetArgs::new()
            .upload_limit(50);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"upload_limit\":50")
    }

    #[test]
    fn request_torrent_set_legacy_upload_limited() -> Result<()> {
        let args = TorrentSetArgs::new()
            .upload_limited(false);
        verify(args, None, "\"uploadLimited\":false")
    }

    #[test]
    fn request_torrent_set_semver_600_upload_limited() -> Result<()> {
        let args = TorrentSetArgs::new()
            .upload_limited(true);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"upload_limited\":true")
    }
}
