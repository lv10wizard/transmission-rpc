use compat_macros::GenerateCompat;
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
#[derive(GenerateCompat, Serialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentAddArgs {
    /// A string of one or more [cookies]. These are passed to the request when [`filename`] is a
    /// url.
    ///
    /// The format of the cookies should be `NAME=CONTENTS`, where `NAME` is the cookie name and
    /// `CONTENTS` is what the cookie should contain. Set multiple cookies like this:
    /// `name1=content1; name2=content2;` etc. See [libcurl documentation] for more information.
    ///
    /// [cookies]: <https://en.wikipedia.org/wiki/HTTP_cookie>
    /// [libcurl documentation]: <https://curl.se/rfc/cookie_spec.html>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    /// Path to download the torrent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_dir: Option<String>,
    /// Path to- or URL of the `.torrent` file to add.
    ///
    /// **NOTE:** Either this (`filename`) or [`metainfo`] **MUST** be `Some(_)`.
    ///
    /// [`metainfo`]: Self::metainfo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Arbitrary labels to set on the newly added torrent.
    ///
    /// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// [Base64]-encoded `.torrent` content.
    ///
    /// **NOTE:** Either this (`metainfo`) or [`filename`] **MUST** be `Some(_)`.
    ///
    /// [Base64]: <https://en.wikipedia.org/wiki/Base64>
    /// [`filename`]: Self::filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
    /// If true, don't start the torrent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Maximum number of peers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit: Option<u16>,
    /// The torrent's bandwidth priority.
    ///
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    #[serde(skip_serializing_if = "Option::is_none", rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<Priority>,
    /// List of indices of files to be downloaded.
    ///
    /// To ignore some files, put their indices in [`files_unwanted`], otherwise they will still be
    /// downloaded.
    ///
    /// [`files_unwanted`]: Self::files_unwanted
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of indices of files to not download.
    pub files_wanted: Option<Vec<usize>>,
    /// List of indices of files not to download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_unwanted: Option<Vec<usize>>,
    /// List of indices of files to be downloaded with high priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_high: Option<Vec<usize>>,
    /// List of indices of files to be downloaded with low priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_low: Option<Vec<usize>>,
    /// List of indices of files to be downloaded with normal priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_normal: Option<Vec<usize>>,
    /// Whether to download torrent pieces sequentially.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_download: Option<bool>,
    /// Download from a specific piece when [`sequential_download`] is enabled (`true`).
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [`sequential_download`]: Self::sequential_download
    #[serde(skip_serializing_if = "Option::is_none")]
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
        self.download_dir = Some(filename.as_ref().to_string());
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

    /// Fluently set the bandwidth priority of the [`TorrentAddArgs`] by consuming the
    /// [`TorrentAddArgs`] instance, modifying its `bandwidth_priority` field, and returning the
    /// instance.
    pub fn bandwidth_priority(mut self, bandwidth_priority: Priority) -> Self {
        self.bandwidth_priority = Some(bandwidth_priority);
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
mod setter_tests {
    #[allow(unused_imports)]
    use super::*;
}

#[cfg(test)]
mod serde_tests {
    #[allow(unused_imports)]
    use super::*;
}
