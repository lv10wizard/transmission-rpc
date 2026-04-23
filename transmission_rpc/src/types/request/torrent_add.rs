use compat_macros::SemverCompat;
use serde::Serialize;

use crate::types::Priority;

/// Defines request arguments for the [`torrent_add`] method.
///
/// **NOTE:** Either [`filename`] **or** [`metainfo`] **MUST** be included. (All other fields are
/// optional). The behavior is undefined if both [`filename`] **and** [`metainfo`] are defined (I
/// think [`filename`] takes precedence in this case but that behavior may change in the future).
///
/// # Constructors
///
/// * [`default`] creates a new [`TorrentAddArgs`] instance with all fields set to `None`.
/// * [`new`] is an alias for [`default`]
///
/// # Setters
///
/// Each field has a fluent setter of the same name that returns the [`TorrentAddArgs`] instance
/// with only that field's value modified, leaving all others untouched.
///
/// e.g.
/// ```rust
/// use transmission_rpc::types::TorrentAddArgs;
///
/// let args = TorrentAddArgs::default()
///     .download_dir("/downloads")
///     .filename("/path/to/my.torrent");
///
/// assert_eq!(args, TorrentAddArgs {
///     download_dir: Some("/downloads".to_string()),
///     filename: Some("/path/to/my.torrent".into()),
///     ..Default::default()
/// });
/// ```
///
/// # Examples
///
/// Construct with fluent setters:
/// ```rust
/// use transmission_rpc::types::TorrentAddArgs;
///
/// let args = TorrentAddArgs::new()
///     .filename("/home/user/debian-testing.iso.torrent")
///     .paused(true)
///     .labels(["linux", "testing"]);
/// ```
///
/// Set fields directly:
/// ```rust
/// use transmission_rpc::types::TorrentAddArgs;
///
/// let mut args = TorrentAddArgs::default();
/// args.filename = Some("/home/user/debian-testing.iso.torrent".into());
/// args.paused = Some(true);
/// args.labels = Some(vec!["linux".into(), "testing".into()]);
/// ```
///
/// Instantiate with concrete values:
/// ```rust
/// use transmission_rpc::types::TorrentAddArgs;
///
/// let args = TorrentAddArgs {
///     filename: Some("/home/user/debian-testing.iso.torrent".into()),
///     paused: Some(true),
///     labels: Some(vec!["linux".to_string(), "testing".to_owned()]),
///     ..Default::default()
/// };
/// ```
///
/// [`filename`]: Self::filename
/// [`metainfo`]: Self::metainfo
/// [`torrent_add`]: crate::TransClient::torrent_add
/// [`default`]: Self::default
/// [`new`]: Self::new
#[serde_with::skip_serializing_none]
#[derive(SemverCompat, Serialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentAddArgs {
    /// The torrent's bandwidth priority.
    #[added = "3.1.0"]
    #[serde(rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<Priority>,
    /// A string of one or more [cookies]. These are passed to the request when [`filename`] is a
    /// url.
    ///
    /// The format of the cookies should be `NAME=CONTENTS`, where `NAME` is the cookie name and
    /// `CONTENTS` is what the cookie should contain. Set multiple cookies like this:
    /// `name1=content1; name2=content2;` etc. See [libcurl documentation] for more information.
    ///
    /// [cookies]: <https://en.wikipedia.org/wiki/HTTP_cookie>
    /// [`filename`]: Self::filename
    /// [libcurl documentation]: <https://curl.se/rfc/cookie_spec.html>
    #[added = "4.0.0"]
    pub cookies: Option<String>,
    /// Path to download the torrent to.
    pub download_dir: Option<String>,
    /// Path to- or URL of the `.torrent` file to add.
    ///
    /// **NOTE:** Either this (`filename`) or [`metainfo`] **MUST** be `Some(_)`.
    ///
    /// [`metainfo`]: Self::metainfo
    pub filename: Option<String>,
    /// Arbitrary labels to set on the newly added torrent.
    #[added = "5.3.0"]
    pub labels: Option<Vec<String>>,
    /// [Base64]-encoded `.torrent` content.
    ///
    /// **NOTE:** Either this (`metainfo`) or [`filename`] **MUST** be `Some(_)`.
    ///
    /// [Base64]: <https://en.wikipedia.org/wiki/Base64>
    /// [`filename`]: Self::filename
    pub metainfo: Option<String>,
    /// If true, don't start the torrent.
    pub paused: Option<bool>,
    /// Maximum number of peers.
    pub peer_limit: Option<u16>,
    /// List of indices of files to be downloaded.
    ///
    /// To ignore some files, put their indices in [`files_unwanted`], otherwise they will still be
    /// downloaded.
    ///
    /// [`files_unwanted`]: Self::files_unwanted
    /// List of indices of files to not download.
    #[added = "2.0.0"]
    pub files_wanted: Option<Vec<usize>>,
    /// List of indices of files not to download.
    #[added = "2.0.0"]
    pub files_unwanted: Option<Vec<usize>>,
    /// List of indices of files to be downloaded with high priority.
    #[added = "2.0.0"]
    pub priority_high: Option<Vec<usize>>,
    /// List of indices of files to be downloaded with low priority.
    #[added = "2.0.0"]
    pub priority_low: Option<Vec<usize>>,
    /// List of indices of files to be downloaded with normal priority.
    #[added = "2.0.0"]
    pub priority_normal: Option<Vec<usize>>,
    /// Whether to download torrent pieces sequentially.
    #[added = "6.0.0"]
    pub sequential_download: Option<bool>,
    /// Download from a specific piece when [`sequential_download`] is enabled (`true`).
    ///
    /// [`sequential_download`]: Self::sequential_download
    #[added = "6.0.0"]
    pub sequential_download_from_piece: Option<u64>,
}

impl TorrentAddArgs {
    /// Constructs a new [`TorrentAddArgs`] with all fields set to `None`. This is an alias for
    /// [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether the [`TorrentAddArgs`] includes the required fields to add the torrent.
    pub fn is_valid(&self) -> bool {
        self.filename.is_some() || self.metainfo.is_some()
    }

    /// Fluently set the bandwidth priority of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `bandwidth_priority` field, and returning the
    /// instance.
    pub fn bandwidth_priority(mut self, bandwidth_priority: Priority) -> Self {
        self.bandwidth_priority = Some(bandwidth_priority);
        self
    }

    /// Fluently set the cookies of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `cookies` field, and returning the instance.
    ///
    /// The format should be `NAME=CONTENTS` with each cookie separated by a `;`, eg. `foo=bar;
    /// lorem=ipsum;`.
    ///
    /// This method takes any [`str`]-like type, eg. [`String`] or `&str`.
    pub fn cookies<S: AsRef<str>>(mut self, cookies: S) -> Self {
        self.cookies = Some(cookies.as_ref().to_string());
        self
    }

    /// Fluently set the download directory path of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `download_dir` field, and returning the
    /// instance.
    ///
    /// This method takes any [`str`]-like type, eg. [`String`] or `&str`.
    pub fn download_dir<S: AsRef<str>>(mut self, download_dir: S) -> Self {
        self.download_dir = Some(download_dir.as_ref().to_string());
        self
    }

    /// Fluently set the filename of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `filename` field, and returning the instance.
    ///
    /// The `filename` should be the path to- or the URL of the `.torrent` file to add.
    ///
    /// **NOTE:** Either `filename` or [`metainfo`] **MUST** be included.
    ///
    /// This method takes any [`str`]-like type, eg. [`String`] or `&str`.
    ///
    /// [`metainfo`]: Self::metainfo
    pub fn filename<S: AsRef<str>>(mut self, filename: S) -> Self {
        self.filename = Some(filename.as_ref().to_string());
        self
    }

    /// Fluently set the labels of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `labels` field, and returning the instance.
    ///
    /// This method takes any collection of [`str`]-like instances that implements
    /// [`IntoIterator`], eg. `Vec<String>` or `[&str]`.
    pub fn labels<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let labels = labels
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
        self.labels = Some(labels);
        self
    }

    /// Fluently set the metainfo of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `metainfo` field, and returning the instance.
    ///
    /// The `metainfo` should be a string containing the [base64]-encoded `.torrent` content.
    ///
    /// **NOTE:** Either [`filename`] or `metainfo` **MUST** be included.
    ///
    /// This method takes any [`str`]-like type, eg. [`String`] or `&str`.
    ///
    /// [base64]: <https://en.wikipedia.org/wiki/Base64>
    /// [`filename`]: Self::filename
    pub fn metainfo<S: AsRef<str>>(mut self, metainfo: S) -> Self {
        self.metainfo = Some(metainfo.as_ref().to_string());
        self
    }

    /// Fluently set the paused state of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `paused` field, and returning the instance.
    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = Some(paused);
        self
    }

    /// Fluently set the peer limit of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `peer_limit` field, and returning the instance.
    pub fn peer_limit(mut self, peer_limit: u16) -> Self {
        self.peer_limit = Some(peer_limit);
        self
    }

    /// Fluently set the wanted files of the [`TorrentAddArgs`] by consuming the [`TorrentAddArgs`]
    /// instance, modifying its `files_wanted` field, and returning the instance.
    ///
    /// This method takes any collection of `usize` instances that implements [`IntoIterator`], eg.
    /// `Vec<usize>` or `[usize]`.
    pub fn files_wanted<I>(mut self, files_wanted: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.files_wanted = Some(files_wanted.into_iter().collect());
        self
    }

    /// Fluently set the files to skip downloading of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `files_unwanted` field, and returning the
    /// instance.
    ///
    /// This method takes any collection of `usize` instances that implements [`IntoIterator`], eg.
    /// `Vec<usize>` or `[usize]`.
    pub fn files_unwanted<I>(mut self, files_unwanted: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.files_unwanted = Some(files_unwanted.into_iter().collect());
        self
    }

    /// Fluently set the high-priority files to download of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `priority_high` field, and returning the
    /// instance.
    ///
    /// This method takes any collection of `usize` instances that implements [`IntoIterator`], eg.
    /// `Vec<usize>` or `[usize]`.
    pub fn priority_high<I>(mut self, priority_high: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.priority_high = Some(priority_high.into_iter().collect());
        self
    }

    /// Fluently set the low-priority files to download of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `priority_low` field, and returning the
    /// instance.
    ///
    /// This method takes any collection of `usize` instances that implements [`IntoIterator`], eg.
    /// `Vec<usize>` or `[usize]`.
    pub fn priority_low<I>(mut self, priority_low: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.priority_low = Some(priority_low.into_iter().collect());
        self
    }

    /// Fluently set the normal-priority files to download of the [`TorrentAddArgs`] by consuming
    /// the [`TorrentAddArgs`] instance, modifying its `priority_normal` field, and returning the
    /// instance.
    ///
    /// This method takes any collection of `usize` instances that implements [`IntoIterator`], eg.
    /// `Vec<usize>` or `[usize]`.
    pub fn priority_normal<I>(mut self, priority_normal: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.priority_normal = Some(priority_normal.into_iter().collect());
        self
    }

    /// Fluently set the sequential download flag of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `sequential_download` field, and returning the
    /// instance.
    pub fn sequential_download(mut self, sequential_download: bool) -> Self {
        self.sequential_download = Some(sequential_download);
        self
    }

    /// Fluently set the specific piece to begin downloading from of the [`TorrentAddArgs`] by
    /// consuming the [`TorrentAddArgs`] instance, modifying its `sequential_download_from_piece`
    /// field, and returning the instance.
    ///
    /// This value has no effect if [`sequential_download`] is either `None` or `Some(false)`.
    ///
    /// [`sequential_download`]: Self::sequential_download
    pub fn sequential_download_from_piece(mut self, piece: u64) -> Self {
        self.sequential_download_from_piece = Some(piece);
        self
    }
}

#[cfg(test)]
mod misc_tests {
    use super::*;

    #[test]
    fn torrent_add_args_new_is_valid_false() {
        let args = TorrentAddArgs::new();

        assert_eq!(args.is_valid(), false);
    }

    #[test]
    fn torrent_add_args_default_is_valid_false() {
        let args = TorrentAddArgs::default();

        assert_eq!(args.is_valid(), false);
    }

    #[test]
    fn torrent_add_args_filename_is_valid_true() {
        let args = TorrentAddArgs {
            filename: Some("/foo/bar/baz/".into()),
            ..Default::default()
        };

        assert_eq!(args.is_valid(), true);
    }

    #[test]
    fn torrent_add_args_metainfo_is_valid_true() {
        let args = TorrentAddArgs {
            metainfo: Some("foobar==".into()),
            ..Default::default()
        };

        assert_eq!(args.is_valid(), true);
    }

    #[test]
    fn torrent_add_args_filename_and_metainfo_is_valid_true() {
        let args = TorrentAddArgs {
            filename: Some("/foo/bar/baz/".into()),
            metainfo: Some("foobar==".into()),
            ..Default::default()
        };

        assert_eq!(args.is_valid(), true);
    }
}

#[cfg(test)]
mod setter_tests {
    use super::*;

    #[test]
    fn torrent_add_args_bandwidth_priority() {
        assert_eq!(
            TorrentAddArgs::new().bandwidth_priority(Priority::Low),
            TorrentAddArgs {
                bandwidth_priority: Some(Priority::Low),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_cookies() {
        assert_eq!(
            TorrentAddArgs::new().cookies("foo=bar; abc=def;"),
            TorrentAddArgs {
                cookies: Some("foo=bar; abc=def;".into()),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_download_dir() {
        assert_eq!(
            TorrentAddArgs::new().download_dir("/lorem/ipsum"),
            TorrentAddArgs {
                download_dir: Some("/lorem/ipsum".into()),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_filename() {
        assert_eq!(
            TorrentAddArgs::new().filename("/lorem/ipsum/foo.torrent"),
            TorrentAddArgs {
                filename: Some("/lorem/ipsum/foo.torrent".into()),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_labels() {
        assert_eq!(
            TorrentAddArgs::new().labels(["foo", "bar"]),
            TorrentAddArgs {
                labels: Some(vec!["foo".to_owned(), "bar".to_string()]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_metainfo() {
        assert_eq!(
            TorrentAddArgs::new().metainfo("abcdef="),
            TorrentAddArgs {
                metainfo: Some("abcdef=".into()),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_paused() {
        assert_eq!(
            TorrentAddArgs::new().paused(true),
            TorrentAddArgs {
                paused: Some(true),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_peer_limit() {
        assert_eq!(
            TorrentAddArgs::new().peer_limit(15),
            TorrentAddArgs {
                peer_limit: Some(15),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_files_wanted() {
        assert_eq!(
            TorrentAddArgs::new().files_wanted([1, 4, 8, 15]),
            TorrentAddArgs {
                files_wanted: Some(vec![1, 4, 8, 15]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_files_unwanted() {
        assert_eq!(
            TorrentAddArgs::new().files_unwanted([123, 124, 603]),
            TorrentAddArgs {
                files_unwanted: Some(vec![123, 124, 603]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_priority_high() {
        assert_eq!(
            TorrentAddArgs::new().priority_high([6]),
            TorrentAddArgs {
                priority_high: Some(vec![6]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_priority_low() {
        assert_eq!(
            TorrentAddArgs::new().priority_low([6, 7]),
            TorrentAddArgs {
                priority_low: Some(vec![6, 7]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_priority_normal() {
        assert_eq!(
            TorrentAddArgs::new().priority_normal([6, 7, 10]),
            TorrentAddArgs {
                priority_normal: Some(vec![6, 7, 10]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_sequential_download() {
        assert_eq!(
            TorrentAddArgs::new().sequential_download(true),
            TorrentAddArgs {
                sequential_download: Some(true),
                ..Default::default()
            }
        )
    }

    #[test]
    fn torrent_add_args_sequential_download_from_piece() {
        assert_eq!(
            TorrentAddArgs::new().sequential_download_from_piece(6006),
            TorrentAddArgs {
                sequential_download_from_piece: Some(6006),
                ..Default::default()
            }
        )
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::types::{JSON_RPC_VERSION_2_0, Result, request::test_helper::verify};
    use super::*;

    #[test]
    fn torrent_add_args_legacy_bandwidth_priority() -> Result<()> {
        let args = TorrentAddArgs::default()
            .bandwidth_priority(Priority::High);
        verify(args, None, "\"bandwidthPriority\":1")
    }

    #[test]
    fn torrent_add_args_semver_600_bandwidth_priority() -> Result<()> {
        let args = TorrentAddArgs::default()
            .bandwidth_priority(Priority::Normal);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"bandwidth_priority\":0")
    }

    #[test]
    fn torrent_add_args_legacy_cookies() -> Result<()> {
        let args = TorrentAddArgs::default()
            .cookies("abc=123");
        verify(args, None, "\"cookies\":\"abc=123\"")
    }

    #[test]
    fn torrent_add_args_semver_600_cookies() -> Result<()> {
        let args = TorrentAddArgs::default()
            .cookies("foo=bar; token=123456;");
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"cookies\":\"foo=bar; token=123456;\"")
    }

    #[test]
    fn torrent_add_args_legacy_download_dir() -> Result<()> {
        let args = TorrentAddArgs::default()
            .download_dir("/home/user/downloads");
        verify(args, None, "\"download-dir\":\"/home/user/downloads\"")
    }

    #[test]
    fn torrent_add_args_semver_600_download_dir() -> Result<()> {
        let args = TorrentAddArgs::default()
            .download_dir("/tmp");
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"download_dir\":\"/tmp\"")
    }

    #[test]
    fn torrent_add_args_legacy_filename() -> Result<()> {
        let args = TorrentAddArgs::default()
            .filename("magnet:?xt=urn:btih:1234567890&dn=foo.iso");
        verify(args, None, "\"filename\":\"magnet:?xt=urn:btih:1234567890&dn=foo.iso\"")
    }

    #[test]
    fn torrent_add_args_semver_600_filename() -> Result<()> {
        let args = TorrentAddArgs::default()
            .filename("/tmp/test.iso.torrent");
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"filename\":\"/tmp/test.iso.torrent\"")
    }

    #[test]
    fn torrent_add_args_legacy_labels() -> Result<()> {
        let args = TorrentAddArgs::default()
            .labels(["test", "DELETE"]);
        verify(args, None, "\"labels\":[\"test\",\"DELETE\"]")
    }

    #[test]
    fn torrent_add_args_semver_600_labels() -> Result<()> {
        let args = TorrentAddArgs::default()
            .labels(vec!["foo", "bar", "123"]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"labels\":[\"foo\",\"bar\",\"123\"]")
    }

    #[test]
    fn torrent_add_args_legacy_metainfo() -> Result<()> {
        let args = TorrentAddArgs::default()
            .metainfo("fOoB/Ar+/");
        verify(args, None, "\"metainfo\":\"fOoB/Ar+/\"")
    }

    #[test]
    fn torrent_add_args_semver_600_metainfo() -> Result<()> {
        let args = TorrentAddArgs::default()
            .metainfo("L+/oR3m+/Ip5UM//");
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"metainfo\":\"L+/oR3m+/Ip5UM//\"")
    }

    #[test]
    fn torrent_add_args_legacy_paused() -> Result<()> {
        let args = TorrentAddArgs::default()
            .paused(false);
        verify(args, None, "\"paused\":false")
    }

    #[test]
    fn torrent_add_args_semver_600_paused() -> Result<()> {
        let args = TorrentAddArgs::default()
            .paused(true);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"paused\":true")
    }

    #[test]
    fn torrent_add_args_legacy_peer_limit() -> Result<()> {
        let args = TorrentAddArgs::default()
            .peer_limit(4);
        verify(args, None, "\"peer-limit\":4")
    }

    #[test]
    fn torrent_add_args_semver_600_peer_limit() -> Result<()> {
        let args = TorrentAddArgs::default()
            .peer_limit(69);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"peer_limit\":69")
    }

    #[test]
    fn torrent_add_args_legacy_files_wanted() -> Result<()> {
        let args = TorrentAddArgs::default()
            .files_wanted([1, 2, 3]);
        verify(args, None, "\"files-wanted\":[1,2,3]")
    }

    #[test]
    fn torrent_add_args_semver_600_files_wanted() -> Result<()> {
        let args = TorrentAddArgs::default()
            .files_wanted([606, 1002, 15392]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"files_wanted\":[606,1002,15392]")
    }

    #[test]
    fn torrent_add_args_legacy_files_unwanted() -> Result<()> {
        let args = TorrentAddArgs::default()
            .files_unwanted([6, 7, 8]);
        verify(args, None, "\"files-unwanted\":[6,7,8]")
    }

    #[test]
    fn torrent_add_args_semver_600_files_unwanted() -> Result<()> {
        let args = TorrentAddArgs::default()
            .files_unwanted([42, 420]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"files_unwanted\":[42,420]")
    }

    #[test]
    fn torrent_add_args_legacy_priority_high() -> Result<()> {
        let args = TorrentAddArgs::default()
            .priority_high([5, 55, 100]);
        verify(args, None, "\"priority-high\":[5,55,100]")
    }

    #[test]
    fn torrent_add_args_semver_600_priority_high() -> Result<()> {
        let args = TorrentAddArgs::default()
            .priority_high([33]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"priority_high\":[33]")
    }

    #[test]
    fn torrent_add_args_legacy_priority_low() -> Result<()> {
        let args = TorrentAddArgs::default()
            .priority_low([10, 12, 20]);
        verify(args, None, "\"priority-low\":[10,12,20]")
    }

    #[test]
    fn torrent_add_args_semver_600_priority_low() -> Result<()> {
        let args = TorrentAddArgs::default()
            .priority_low([2, 5]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"priority_low\":[2,5]")
    }

    #[test]
    fn torrent_add_args_legacy_priority_normal() -> Result<()> {
        let args = TorrentAddArgs::default()
            .priority_normal([100, 101, 110]);
        verify(args, None, "\"priority-normal\":[100,101,110]")
    }

    #[test]
    fn torrent_add_args_semver_600_priority_normal() -> Result<()> {
        let args = TorrentAddArgs::default()
            .priority_normal([20, 51]);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"priority_normal\":[20,51]")
    }

    #[test]
    fn torrent_add_args_legacy_sequential_download() -> Result<()> {
        let args = TorrentAddArgs::default()
            .sequential_download(true);
        verify(args, None, "")
    }

    #[test]
    fn torrent_add_args_semver_600_sequential_download() -> Result<()> {
        let args = TorrentAddArgs::default()
            .sequential_download(false);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"sequential_download\":false")
    }

    #[test]
    fn torrent_add_args_legacy_sequential_download_from_piece() -> Result<()> {
        let args = TorrentAddArgs::default()
            .sequential_download_from_piece(39);
        verify(args, None, "")
    }

    #[test]
    fn torrent_add_args_semver_600_sequential_download_from_piece() -> Result<()> {
        let args = TorrentAddArgs::default()
            .sequential_download_from_piece(40);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"sequential_download_from_piece\":40")
    }
}
