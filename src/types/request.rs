use std::fmt::{self, Display};

use enum_iterator::all;
use serde::{Serialize, Serializer};
use serde_with::skip_serializing_none;

use compat_macros::GenerateCompat;

use crate::json_rpc::{JsonRpcId, JsonRpcRequest};
use super::{AltSpeedDay, Encryption, EncryptionCompat, Id, MinutesAfterMidnight, Tag, Transport};

pub(crate) use group_set::*; // GroupSetArgs
pub(crate) use session_get::*; // SessionGetArgs
pub(crate) use torrent_add::*; // TorrentAddArgs
pub(crate) use torrent_get::*; // TorrentGetArgs
pub(crate) use torrent_set::*; // TorrentSetArgs

pub use group_set::GroupSetArgs;
pub use session_get::SessionGetField;
pub use torrent_add::TorrentAddArgs;
pub use torrent_get::TorrentGetField;
pub use torrent_set::{TorrentSetArgs, TrackerReplaceArgs, TrackerReplacePair};

mod group_set;
mod into;
mod session_get;
mod torrent_add;
mod torrent_get;
mod torrent_set;

#[cfg(test)]
mod json_rpc_tests;
#[cfg(test)]
mod session_set_serde_tests;
#[cfg(test)]
mod test_helper;

/// Represents a transmission rpc method.
#[derive(Debug)]
pub(crate) struct RpcRequest {
    method: Method,
    arguments: Option<Args>,
    /// "An optional `tag` number used by clients to track responses. If provided by a request, the
    /// response MUST include the same tag." <sup>[1][2]</sup>
    ///
    /// This struct also doubles as the JSON-RPC "id" request field for Transmission 4.1.0
    /// (rpc_version_semver 6.0.0, rpc_version: 18) and later. "id" defaults to `Tag(0)` if `None`.
    ///
    /// [1]: <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#21-requests>
    /// [2]: <https://github.com/transmission/transmission/blob/4.0.6/libtransmission/rpcimpl.cc#L2520>
    tag: Option<Tag>,

    pub(crate) jsonrpc: Option<String>,
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.jsonrpc.as_ref() {
            Some(jsonrpc) => {
                JsonRpcRequest {
                    jsonrpc: jsonrpc,
                    // All method names were converted to snake_case in Transmission 4.1.0 (when
                    // the RPC server switched to the JSON-RPC 2.0 protocol).
                    method: &self.method
                        .as_str()
                        .replace("-", "_"),
                    params: self.arguments
                        // Cloning the request arguments shouldn't be too costly...
                        .clone()
                        .map(|args| args.into_compat()),
                    id: self.tag
                        // Try to use the provided tag, if one exists.
                        .map(Into::into)
                        // Always ask the rpc server for a response so that users can decide what
                        // they want to do with it.
                        .or_else(|| Some(JsonRpcId::default()))
                }
                .serialize(serializer)
            },

            None => {
                /// Serialization helper for legacy requests (pre- Transmission 4.1.0).
                #[derive(Serialize)]
                struct LegacyRequest<'a> {
                    method: &'a Method,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    arguments: &'a Option<Args>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    tag: Option<Tag>,
                }

                LegacyRequest {
                    method: &self.method,
                    arguments: &self.arguments,
                    tag: self.tag,
                }
                .serialize(serializer)
            },
        }
    }
}

impl RpcRequest {
    /// Fluent setter to assign an arbitrary `tag` to the `RpcRequest`.
    #[allow(dead_code)]
    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.set_tag(tag);
        self
    }

    #[allow(dead_code)]
    pub fn set_tag(&mut self, tag: Tag) {
        self.tag = Some(tag);
    }

    pub fn session_set(args: SessionSetArgs, tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::SessionSet,
            arguments: Some(Args::SessionSet(args)),
            tag,
            jsonrpc: None,
        }
    }

    pub fn session_get(args: Option<SessionGetArgs>, tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::SessionGet,
            arguments: args.map(Into::into),
            tag,
            jsonrpc: None,
        }
    }

    pub fn session_stats(tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::SessionStats,
            arguments: None,
            tag,
            jsonrpc: None,
        }
    }

    pub fn session_close(tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::SessionClose,
            arguments: None,
            tag,
            jsonrpc: None,
        }
    }

    pub fn blocklist_update(tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::BlocklistUpdate,
            arguments: None,
            tag,
            jsonrpc: None,
        }
    }

    pub fn free_space(path: String, tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::FreeSpace,
            arguments: Some(Args::FreeSpace(FreeSpaceArgs { path })),
            tag,
            jsonrpc: None,
        }
    }

    pub fn port_test(tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::PortTest,
            arguments: None,
            tag,
            jsonrpc: None,
        }
    }

    pub fn queue_move_top<I>(ids: I, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        RpcRequest {
            method: Method::QueueMoveTop,
            arguments: Args::QueueMove(Vec::from_iter(ids).into()).into(),
            tag,
            jsonrpc: None,
        }
    }

    pub fn queue_move_up<I>(ids: I, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        RpcRequest {
            method: Method::QueueMoveUp,
            arguments: Args::QueueMove(Vec::from_iter(ids).into()).into(),
            tag,
            jsonrpc: None,
        }
    }

    pub fn queue_move_down<I>(ids: I, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        RpcRequest {
            method: Method::QueueMoveDown,
            arguments: Args::QueueMove(Vec::from_iter(ids).into()).into(),
            tag,
            jsonrpc: None,
        }
    }

    pub fn queue_move_bottom<I>(ids: I, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        RpcRequest {
            method: Method::QueueMoveBottom,
            arguments: Args::QueueMove(Vec::from_iter(ids).into()).into(),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_get<FIELDS, IDS>(
        fields: Option<FIELDS>,
        ids: Option<IDS>,
        tag: Option<Tag>,
    ) -> RpcRequest
    where
        FIELDS: IntoIterator<Item = TorrentGetField> + FromIterator<TorrentGetField>,
        IDS: IntoIterator<Item = Id>,
    {
        let fields = fields
            .unwrap_or_else(|| all::<TorrentGetField>().collect())
            .into_iter()
            .collect();
        let ids = ids.map(|ids| ids.into_iter().collect());
        RpcRequest {
            method: Method::TorrentGet,
            arguments: Some(Args::TorrentGet(TorrentGetArgs {
                fields: Some(fields),
                ids,
            })),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_set<I>(mut args: TorrentSetArgs, ids: Option<I>, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        args.ids = ids.map(|ids| ids.into_iter().collect());
        RpcRequest {
            method: Method::TorrentSet,
            arguments: Some(Args::TorrentSet(args)),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_remove<I>(ids: I, delete_local_data: bool, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        let ids = ids.into_iter().collect();
        RpcRequest {
            method: Method::TorrentRemove,
            arguments: Some(Args::TorrentRemove(TorrentRemoveArgs {
                ids,
                delete_local_data,
            })),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_add(add: TorrentAddArgs, tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::TorrentAdd,
            arguments: Some(Args::TorrentAdd(add)),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_action<I>(action: TorrentAction, ids: I, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        let ids = ids.into_iter().collect();
        RpcRequest {
            method: Method::TorrentAction(action),
            arguments: Some(Args::TorrentAction(TorrentActionArgs { ids })),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_set_location<I>(
        ids: I,
        location: String,
        move_from: bool,
        tag: Option<Tag>,
    ) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        let ids = ids.into_iter().collect();
        RpcRequest {
            method: Method::TorrentSetLocation,
            arguments: Some(Args::TorrentSetLocation(TorrentSetLocationArgs {
                ids,
                location,
                r#move: move_from,
            })),
            tag,
            jsonrpc: None,
        }
    }

    pub fn torrent_rename_path<I>(
        ids: I,
        path: String,
        name: String,
        tag: Option<Tag>,
    ) -> RpcRequest
    where
        I: IntoIterator<Item = Id>,
    {
        let ids = ids.into_iter().collect();
        RpcRequest {
            method: Method::TorrentRenamePath,
            arguments: Some(Args::TorrentRenamePath(TorrentRenamePathArgs {
                ids,
                path,
                name,
            })),
            tag,
            jsonrpc: None,
        }
    }

    pub fn group_get<I>(groups: Option<I>, tag: Option<Tag>) -> RpcRequest
    where
        I: IntoIterator<Item = String>,
    {
        RpcRequest {
            method: Method::GroupGet,
            arguments: Some(Args::GroupGet(groups.map(Vec::from_iter).into())),
            tag,
            jsonrpc: None,
        }
    }

    pub fn group_set(args: GroupSetArgs, tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::GroupSet,
            arguments: Some(Args::GroupSet(args)),
            tag,
            jsonrpc: None,
        }
    }
}

/// Converts a `Vec<T>` into a `Vec<U>` by iterating over all of `vec`'s items and calling `func`
/// on each.
///
/// The function signature was created to conform with [`Option::map`] arguments, specifically for
/// [`GenerateCompat`] usage.
pub(crate) fn map_vec<F, T, U>(vec: Vec<T>, func: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    vec.into_iter()
        .map(func)
        .collect()
}

/// Converts a `Option<Vec<T>>` into a `Option<Vec<U>>` by iterating over all of `vec`'s items and
/// calling `func` on each.
///
/// The function signature was created to conform with [`Option::map`] arguments, specifically for
/// [`GenerateCompat`] usage.
pub(crate) fn map_option_vec<F, T, U>(vec: Option<Vec<T>>, func: F) -> Option<Vec<U>>
where
    F: Fn(T) -> U,
{
    vec.map(|vec| {
        vec.into_iter()
            .map(func)
            .collect()
    })
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Method {
    SessionSet,
    SessionGet,
    SessionStats,
    SessionClose,
    BlocklistUpdate,
    FreeSpace,
    GroupGet,
    GroupSet,
    PortTest,
    TorrentGet,
    TorrentSet,
    TorrentRemove,
    TorrentAdd,
    TorrentAction(TorrentAction),
    TorrentSetLocation,
    TorrentRenamePath,
    QueueMoveUp,
    QueueMoveDown,
    QueueMoveTop,
    QueueMoveBottom,
}

impl Method {
    fn as_str(&self) -> &'static str {
        use Method as M;

        match self {
            M::SessionSet => "session-set",
            M::SessionGet => "session-get",
            M::SessionStats => "session-stats",
            M::SessionClose => "session-close",
            M::BlocklistUpdate => "blocklist-update",
            M::FreeSpace => "free-space",
            M::GroupGet => "group-get",
            M::GroupSet => "group-set",
            M::PortTest => "port-test",
            M::TorrentGet => "torrent-get",
            M::TorrentSet => "torrent-set",
            M::TorrentRemove => "torrent-remove",
            M::TorrentAdd => "torrent-add",
            M::TorrentAction(action) => action.as_str(),
            M::TorrentSetLocation => "torrent-set-location",
            M::TorrentRenamePath => "torrent-rename-path",
            M::QueueMoveUp => "queue-move-up",
            M::QueueMoveDown => "queue-move-down",
            M::QueueMoveTop => "queue-move-top",
            M::QueueMoveBottom => "queue-move-bottom",
        }
    }
}

// Manually implement [Serialize] for [Method] because serde doesn't support flattening of
// tuple struct variant of enums, [Method::TorrentAction(_)] in this case.
impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField {}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
#[compat(placeholder = P)]
#[serde(untagged)]
pub enum Args {
    #[compat(type = P)]
    FreeSpace(FreeSpaceArgs),
    #[compat(type = P)]
    GroupGet(GroupGetArgs),
    #[compat(type = P)]
    GroupSet(GroupSetArgs),
    #[compat(type = P)]
    SessionGet(SessionGetArgs),
    #[compat(type = P)]
    SessionSet(SessionSetArgs),
    #[compat(type = P)]
    QueueMove(QueueMoveArgs),
    #[compat(type = P)]
    TorrentGet(TorrentGetArgs),
    #[compat(type = P)]
    TorrentAction(TorrentActionArgs),
    #[compat(type = P)]
    TorrentRemove(TorrentRemoveArgs),
    #[compat(type = P)]
    TorrentAdd(TorrentAddArgs),
    #[compat(type = P)]
    TorrentSet(TorrentSetArgs),
    #[compat(type = P)]
    TorrentSetLocation(TorrentSetLocationArgs),
    #[compat(type = P)]
    TorrentRenamePath(TorrentRenamePathArgs),
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
pub struct FreeSpaceArgs {
    path: String,
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
pub(crate) struct GroupGetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<String>>,
}

impl From<Vec<String>> for GroupGetArgs {
    fn from(value: Vec<String>) -> Self {
        (!value.is_empty())
            .then_some(value)
            .into()
    }
}

impl<I: IntoIterator<Item = String>> From<Option<I>> for GroupGetArgs {
    fn from(value: Option<I>) -> Self {
        Self {
            groups: value
                .map(|val| val.into_iter().collect()),
        }
    }
}

#[skip_serializing_none]
#[derive(GenerateCompat, Serialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionSetArgs {
    /// Max global download speed (kB/s).
    pub alt_speed_down: Option<u64>,
    /// True means use the alt speeds.
    pub alt_speed_enabled: Option<bool>,
    /// When to turn on alt speeds (units: minutes after midnight).
    pub alt_speed_time_begin: Option<MinutesAfterMidnight>,
    /// What day(s) to turn on alt speeds.
    pub alt_speed_time_day: Option<AltSpeedDay>,
    /// True means the scheduled on/off times are used.
    pub alt_speed_time_enabled: Option<bool>,
    /// When to turn off alt speeds (units: minutes after midnight).
    pub alt_speed_time_end: Option<MinutesAfterMidnight>,
    /// Max global upload speed (kB/s).
    pub alt_speed_up: Option<u64>,

    /// Enable a very basic brute force protection for the RPC server. See
    /// [`anti_brute_force_threshold`] below.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [`anti_brute_force_threshold`]: Self::anti_brute_force_threshold
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub anti_brute_force_enabled: Option<bool>,

    /// After this amount of failed authentication attempts is surpassed, the RPC server will deny
    /// any further authentication attempts until it is restarted. This is not tracked per IP but
    /// in total.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub anti_brute_force_threshold: Option<u64>,

    /// True means block peers based on [`blocklist_url`]. See also: [blocklists.md].
    ///
    /// [`blocklist_url`]: Self::blocklist_url
    /// [blocklists.md]:
    /// <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    pub blocklist_enabled: Option<bool>,
    /// Location of the blocklist to use. See: [blocklists.md].
    ///
    /// [blocklists.md]:
    /// <https://github.com/transmission/transmission/blob/main/docs/Blocklists.md>
    pub blocklist_url: Option<String>,

    /// Number in MiB to allocate for Transmission's memory cache. The cache is used to help batch
    /// disk IO together, so increasing the cache size can be used to reduce the number of disk
    /// reads and writes. The value is the total available to the Transmission instance. Set it to
    /// the smallest value tolerable by the random access performance of your storage medium to
    /// minimize data loss in case Transmission quit unexpectedly. Setting this to 0 bypasses the
    /// cache, which may be useful if your filesystem already has a cache layer that aggregates
    /// transactions. Pieces are guaranteed to be written to filesystem if sequential download is
    /// enabled. Otherwise, data might still be in cache only.
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?):
    /// The memory cache is being removed, making this setting moot. The setting will still be
    /// gettable and settable via RPC `session_get` and `session_set` until Transmission 5.0.0 to
    /// avoid client breakage, but it will be otherwise unused in libtransmission. Clients should
    /// stop using this key.
    #[compat(name = cache_size_mib)]
    pub cache_size_mb: Option<i32>,

    /// Announce URLs, one per line, and a blank line between [tiers].
    ///
    /// eg. `"http://bt1.archive.org:6969/announce\n\nhttp://bt2.archive.org:6969/announce\n"`
    /// 
    /// [tiers]: <https://www.bittorrent.org/beps/bep_0012.html>
    pub default_trackers: Option<String>,
    /// True means allow [Distrubted Hash Table] in public torrents.
    ///
    /// [Distrubted Hash Table]: <https://wikipedia.org/wiki/Distributed_hash_table>
    pub dht_enabled: Option<bool>,
    /// Default path to download torrents.
    pub download_dir: Option<String>,
    /// If true, limit how many torrents can be downloaded at once.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    pub download_queue_enabled: Option<bool>,
    /// Max number of torrents to download at once (see [`download_queue_enabled`])
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`download_queue_enabled`]: Self::download_queue_enabled
    pub download_queue_size: Option<u64>,
    /// Encryption preference. Encryption may help get around some ISP filtering, but at the cost
    /// of slightly higher CPU use.
    #[compat(type = Option<EncryptionCompat>, map = Option::map)]
    pub encryption: Option<Encryption>,
    /// Torrents we're seeding will be stopped if they're idle for this long.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    pub idle_seeding_limit: Option<u64>,
    /// True if the [seeding inactivity limit] is honored by default.
    ///
    /// > Added in Transmission 2.10 (`rpc-version-semver` 3.4.0, `rpc-version`: 10)
    ///
    /// [seeding inactivity limit]: Self::idle_seeding_limit
    pub idle_seeding_limit_enabled: Option<bool>,
    /// Path for incomplete torrents, when enabled.
    pub incomplete_dir: Option<String>,
    /// True means keep torrents in [`incomplete_dir`] until done.
    ///
    /// [`incomplete_dir`]: Self::incomplete_dir
    pub incomplete_dir_enabled: Option<bool>,
    /// True means allow [Local Peer Discovery] in public torrents.
    ///
    /// [Local Peer Discovery]: <https://en.wikipedia.org/wiki/Local_Peer_Discovery>
    pub lpd_enabled: Option<bool>,
    /// Maximum global number of peers.
    pub peer_limit_global: Option<u64>,
    /// Maximum number of peers per torrent.
    pub peer_limit_per_torrent: Option<u64>,
    /// True means pick a random peer port on launch.
    pub peer_port_random_on_start: Option<bool>,
    /// The daemon's port number.
    pub peer_port: Option<u16>,
    /// True means allow [Peer Exchange] in public torrents.
    ///
    /// [Peer Exchange]: <https://wikipedia.org/wiki/Peer_exchange>
    pub pex_enabled: Option<bool>,
    /// True means ask upstream router to forward the configured peer port to transmission using
    /// [UPnP] or [NAT-PMP].
    ///
    /// [UPnP]: <https://en.wikipedia.org/wiki/Universal_Plug_and_Play>
    /// [NAT-PMP]: <https://en.wikipedia.org/wiki/NAT_Port_Mapping_Protocol>
    pub port_forwarding_enabled: Option<bool>,

    /// List your preference of transport protocols in the order of preferred-first. Omitting the
    /// transport protocol from the list will disable it. *Note: Never disable TCP when you also
    /// disable µTP, because then your client would not be able to communicate. Disabling TCP might
    /// also break webseeds.*
    ///
    ///  > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub preferred_transports: Option<Vec<Transport>>,

    /// Whether or not to consider [idle torrents as stalled].
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [idle torrents as stalled]: Self::queue_stalled_minutes
    pub queue_stalled_enabled: Option<bool>,
    /// Torrents that are idle for N minuets aren't counted toward [`seed_queue_size`] or
    /// [`download_queue_size`].
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_size`]: Self::seed_queue_size
    /// [`download_queue_size`]: Self::download_queue_size
    pub queue_stalled_minutes: Option<u64>,
    /// True means append `.part` to incomplete files.
    ///
    /// > Added in Transmission 1.90 (`rpc-version-semver` 3.1.0, `rpc-version`: 8)
    pub rename_partial_files: Option<bool>,
    /// The number of outstanding block requests a peer is allowed to queue in the client. The
    /// higher this number, the higher the max possible upload speed towards each peer.
    ///
    /// > (?) Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18) [in
    /// [transmission:62240393e]]
    ///
    /// [transmission:62240393e]:
    /// <https://github.com/transmission/transmission/commit/62240393ed056099a6a2ee60d778ac19928ef451>
    pub reqq: Option<u64>,
    /// Run a script when a torrent is added to Transmission. See: [scripts.md].
    ///
    /// [scripts.md]: <https://github.com/transmission/transmission/blob/main/docs/Scripts.md>
    pub script_torrent_added_enabled: Option<bool>,
    /// Path to script.
    pub script_torrent_added_filename: Option<String>,
    /// Run a script when a torrent is done downloading. See: [scripts.md].
    ///
    /// [scripts.md]: <https://github.com/transmission/transmission/blob/main/docs/Scripts.md>
    pub script_torrent_done_enabled: Option<bool>,
    /// Path to script.
    pub script_torrent_done_filename: Option<String>,
    /// Run a script when a torrent is done seeding. See: [scripts.md].
    ///
    /// > (?) Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17) [in
    /// [transmission:9f9b6cdaa]]
    ///
    /// [scripts.md]: <https://github.com/transmission/transmission/blob/main/docs/Scripts.md>
    /// [transmission:9f9b6cdaa]:
    /// <https://github.com/transmission/transmission/commit/9f9b6cdaa2e02727ee62b0f63a34d297e2246650>
    pub script_torrent_done_seeding_enabled: Option<bool>,
    /// Path to script.
    ///
    /// > (?) Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17) [in
    /// [transmission:9f9b6cdaa]]
    ///
    /// [transmission:9f9b6cdaa]:
    /// <https://github.com/transmission/transmission/commit/9f9b6cdaa2e02727ee62b0f63a34d297e2246650>
    pub script_torrent_done_seeding_filename: Option<String>,
    /// When true, Transmission will only seed [`seed_queue_size`] non-stalled torrents at once.
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_size`]: Self::seed_queue_size
    pub seed_queue_enabled: Option<bool>,
    /// Max number of torrents to uploaded at once (see [`seed_queue_enabled`]).
    ///
    /// > Added in Transmission 2.40 (`rpc-version-semver` 5.0.0, `rpc-version`: 14)
    ///
    /// [`seed_queue_enabled`]: Self::seed_queue_enabled
    pub seed_queue_size: Option<u64>,
    /// The default seed ratio for torrents to use.
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    /// True if [`seed_ratio_limit`] is honored by default.
    ///
    /// [`seed_ratio_limit`]: Self::seed_ratio_limit
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,

    /// True means sequential download is enabled by default for added torrents.
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download: Option<bool>,
    /// Download from a specific piece when [sequential download] is enabled.
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18)
    ///
    /// [sequential download]: Self::sequential_download
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download_from_piece: Option<u64>,

    /// Max global download speed (kB/s).
    pub speed_limit_down: Option<u64>,
    /// Whether [`speed_limit_down`] is respected.
    ///
    /// [`speed_limit_down`]: Self::speed_limit_down
    pub speed_limit_down_enabled: Option<bool>,
    /// Max global upload speed (kB/s).
    pub speed_limit_up: Option<i32>,
    /// Whether [`speed_limit_up`] is respected.
    ///
    /// [`speed_limit_up`]: Self::speed_limit_up
    pub speed_limit_up_enabled: Option<bool>,
    /// Start torrents as soon as they are added.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    pub start_added_torrents: Option<bool>,
    /// Delete torrents added from the watch directory.
    ///
    /// > Added in Transmission 2.00 (`rpc-version-semver` 3.3.0, `rpc-version`: 9)
    pub trash_original_torrent_files: Option<bool>,
    /// True means allow [uTP].
    ///
    /// > ⚠ **DEPRECATED** in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18):
    /// Use [`preferred_transports`] instead.
    ///
    /// [uTP]: <https://wikipedia.org/wiki/Micro_Transport_Protocol>
    /// [`preferred_transports`]: Self::preferred_transports
    pub utp_enabled: Option<bool>,
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
pub struct QueueMoveArgs {
    ids: Vec<Id>,
}

impl<I: IntoIterator<Item = Id>> From<I> for QueueMoveArgs {
    fn from(ids: I) -> Self {
        Self { ids: ids.into_iter().collect() }
    }
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
pub struct TorrentActionArgs {
    ids: Vec<Id>,
}

impl<I: IntoIterator<Item = Id>> From<I> for TorrentActionArgs {
    fn from(ids: I) -> Self {
        Self { ids: ids.into_iter().collect() }
    }
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    delete_local_data: bool,
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    r#move: bool,
}

#[derive(GenerateCompat, Serialize, Debug, Clone)]
pub struct TorrentRenamePathArgs {
    ids: Vec<Id>,
    path: String,
    name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TorrentAction {
    Start,
    Stop,
    StartNow,
    Verify,
    Reannounce,
}

impl TorrentAction {
    #[must_use]
    pub fn to_str(&self) -> String {
        self.as_str().to_string()
    }

    fn as_str(&self) -> &'static str {
        use TorrentAction as A;

        match self {
            A::Start => "torrent-start",
            A::Stop => "torrent-stop",
            A::StartNow => "torrent-start-now",
            A::Verify => "torrent-verify",
            A::Reannounce => "torrent-reannounce",
        }
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::types::{JSON_RPC_VERSION_2_0, Result};
    use super::{*, test_helper::verify};

    #[test]
    fn request_free_space_legacy() -> Result<()> {
        let args = FreeSpaceArgs { path: "/downloads".into() };
        verify(args, None, "\"path\":\"/downloads\"")
    }

    #[test]
    fn request_free_space_semver_600() -> Result<()> {
        let args = FreeSpaceArgs { path: "/foo/bar".into() };
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"path\":\"/foo/bar\"")
    }

    #[test]
    fn request_queue_move_top_legacy() -> Result<()> {
        let req = RpcRequest::queue_move_top([Id::Id(123)], None);
        verify(req, None, "\"ids\":[123]")
    }

    #[test]
    fn request_queue_move_top_semver_600() -> Result<()> {
        let req = RpcRequest::queue_move_top([Id::Id(654)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[654]")
    }

    #[test]
    fn request_queue_move_up_legacy() -> Result<()> {
        let req = RpcRequest::queue_move_up([Id::Id(123)], None);
        verify(req, None, "\"ids\":[123]")
    }

    #[test]
    fn request_queue_move_up_semver_600() -> Result<()> {
        let req = RpcRequest::queue_move_up([Id::Id(654)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[654]")
    }

    #[test]
    fn request_queue_move_down_legacy() -> Result<()> {
        let req = RpcRequest::queue_move_down([Id::Id(123)], None);
        verify(req, None, "\"ids\":[123]")
    }

    #[test]
    fn request_queue_move_down_semver_600() -> Result<()> {
        let req = RpcRequest::queue_move_down([Id::Id(654)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[654]")
    }

    #[test]
    fn request_queue_move_bottom_legacy() -> Result<()> {
        let req = RpcRequest::queue_move_bottom([Id::Id(123)], None);
        verify(req, None, "\"ids\":[123]")
    }

    #[test]
    fn request_queue_move_bottom_semver_600() -> Result<()> {
        let req = RpcRequest::queue_move_bottom([Id::Id(654)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[654]")
    }

    #[test]
    fn request_torrent_remove_legacy() -> Result<()> {
        let req = RpcRequest::torrent_remove([Id::Id(2)], true, None);
        verify(req, None,
            "\"ids\":[2],\
            \"delete-local-data\":true")
    }

    #[test]
    fn request_torrent_remove_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_remove([Id::Id(13)], false, None);
        verify(req, Some(JSON_RPC_VERSION_2_0), 
            "\"ids\":[13],\
            \"delete_local_data\":false")
    }

    #[test]
    fn request_torrent_action_start_legacy() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Start, [Id::Id(6)], None);
        verify(req, None, "\"ids\":[6]")
    }

    #[test]
    fn request_torrent_action_start_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Start, [Id::Id(111)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[111]")
    }

    #[test]
    fn request_torrent_action_stop_legacy() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Stop, [Id::Id(7)], None);
        verify(req, None, "\"ids\":[7]")
    }

    #[test]
    fn request_torrent_action_stop_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Stop, [Id::Id(222)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[222]")
    }

    #[test]
    fn request_torrent_action_start_now_legacy() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::StartNow, [Id::Id(8)], None);
        verify(req, None, "\"ids\":[8]")
    }

    #[test]
    fn request_torrent_action_start_now_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::StartNow, [Id::Id(333)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[333]")
    }

    #[test]
    fn request_torrent_action_verify_legacy() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Verify, [Id::Id(9)], None);
        verify(req, None, "\"ids\":[9]")
    }

    #[test]
    fn request_torrent_action_verify_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Verify, [Id::Id(444)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[444]")
    }

    #[test]
    fn request_torrent_action_reannounce_legacy() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Reannounce, [Id::Id(10)], None);
        verify(req, None, "\"ids\":[10]")
    }

    #[test]
    fn request_torrent_action_reannounce_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_action(TorrentAction::Reannounce, [Id::Id(555)], None);
        verify(req, Some(JSON_RPC_VERSION_2_0), "\"ids\":[555]")
    }

    #[test]
    fn request_torrent_set_location_legacy() -> Result<()> {
        let req = RpcRequest::torrent_set_location(
            [Id::Id(105)],
            "/complete".into(),
            false,
            None);
        verify(req, None,
            "\"ids\":[105],\
            \"location\":\"/complete\",\
            \"move\":false")
    }

    #[test]
    fn request_torrent_set_location_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_set_location(
            [Id::Id(205)],
            "/iso".into(),
            true,
            None);
        verify(req, Some(JSON_RPC_VERSION_2_0),
            "\"ids\":[205],\
            \"location\":\"/iso\",\
            \"move\":true")
    }

    #[test]
    fn request_torrent_rename_path_legacy() -> Result<()> {
        let req = RpcRequest::torrent_rename_path(
            [Id::Id(313)],
            "/downloads/debian-13.3.0-amd64-netinst.iso".into(),
            "foo.bar.iso".into(),
            None);
        verify(req, None,
            "\"ids\":[313],\
            \"path\":\"/downloads/debian-13.3.0-amd64-netinst.iso\",\
            \"name\":\"foo.bar.iso\"")
    }

    #[test]
    fn request_torrent_rename_path_semver_600() -> Result<()> {
        let req = RpcRequest::torrent_rename_path(
            [Id::Id(315)],
            "/downloads/cachyos-desktop-linux.iso".into(),
            "cachyos-desktop-linux-260101.iso".into(),
            None);
        verify(req, Some(JSON_RPC_VERSION_2_0),
            "\"ids\":[315],\
            \"path\":\"/downloads/cachyos-desktop-linux.iso\",\
            \"name\":\"cachyos-desktop-linux-260101.iso\"")
    }

    #[test]
    fn request_group_get_legacy() -> Result<()> {
        let args = GroupGetArgs { groups: None };
        verify(args, None, "")?;

        let args = GroupGetArgs { groups: Some(vec!["slow".into(), "fast".into()]) };
        verify(args, None, "\"groups\":[\"slow\",\"fast\"]")
    }

    #[test]
    fn request_group_get_semver_600() -> Result<()> {
        let args = GroupGetArgs { groups: None };
        verify(args, Some(JSON_RPC_VERSION_2_0), "")?;

        let args = GroupGetArgs { groups: Some(vec!["foo".into(), "bar".into()]) };
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"groups\":[\"foo\",\"bar\"]")
    }
}
