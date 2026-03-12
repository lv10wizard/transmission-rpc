use convert_case::ccase;
use enum_iterator::{all, Sequence};
use serde::{Serialize, Serializer};
use serde_with::skip_serializing_none;

use jsonrpc_macros::{compat_with, generate_semver_600_compat};

use crate::json_rpc::{JsonRpcId, JsonRpcRequest};
use super::{
    AltSpeedDay, Encryption, EncryptionCompat, Id, IdleMode, Priority, RatioMode, Tag, Transport,
};

pub(crate) use session_get::*; // SessionGetArgs, __semver_600_compat_SessionGetArgs
pub use session_get::SessionGetField;

#[cfg(feature = "tor-get-serde")]
use serde::Deserialize; 

mod group_set;
mod session_get;
mod torrent_set;

#[cfg(test)]
mod json_rpc_tests;
#[cfg(test)]
mod session_set_serde_tests;

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
                    // Always ask the rpc server for a response so that users can decide what they
                    // want to do with it.
                    id: Some(JsonRpcId::default()),
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
        let string_fields = fields
            .unwrap_or_else(|| all::<TorrentGetField>().collect())
            .into_iter()
            .map(|f| TorrentGetField::to_str(&f))
            .collect();
        let ids = ids.map(|ids| ids.into_iter().collect());
        RpcRequest {
            method: Method::TorrentGet,
            arguments: Some(Args::TorrentGet(TorrentGetArgs {
                fields: Some(string_fields),
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
        move_from: Option<bool>,
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
                move_from,
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

pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField {}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Args {
    FreeSpace(FreeSpaceArgs),
    GroupGet(GroupGetArgs),
    GroupSet(GroupSetArgs),
    SessionGet(SessionGetArgs),
    SessionSet(SessionSetArgs),
    QueueMove(QueueMoveArgs),
    TorrentGet(TorrentGetArgs),
    TorrentAction(TorrentActionArgs),
    TorrentRemove(TorrentRemoveArgs),
    TorrentAdd(TorrentAddArgs),
    TorrentSet(TorrentSetArgs),
    TorrentSetLocation(TorrentSetLocationArgs),
    TorrentRenamePath(TorrentRenamePathArgs),
}

// TODO: refactor to #[use_compat] (or something)
/// [`Args`] semver-6.0.0 compatibility helper to facilitate legacy request to JSON-RPC/snake_case
/// request serialization.
///
/// The inner `__semver_600_compat_[...]` names are generated by the
/// `#[generate_semver_600_compat]` helper attribute proc-macro.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
enum ArgsCompat {
    FreeSpace(__semver_600_compat_FreeSpaceArgs),
    GroupGet(__semver_600_compat_GroupGetArgs),
    GroupSet(__semver_600_compat_GroupSetArgs),
    SessionGet(__semver_600_compat_SessionGetArgs),
    SessionSet(__semver_600_compat_SessionSetArgs),
    QueueMove(__semver_600_compat_QueueMoveArgs),
    TorrentGet(__semver_600_compat_TorrentGetArgs),
    TorrentAction(__semver_600_compat_TorrentActionArgs),
    TorrentRemove(__semver_600_compat_TorrentRemoveArgs),
    TorrentAdd(__semver_600_compat_TorrentAddArgs),
    TorrentSet(__semver_600_compat_TorrentSetArgs),
    TorrentSetLocation(__semver_600_compat_TorrentSetLocationArgs),
    TorrentRenamePath(__semver_600_compat_TorrentRenamePathArgs),
} // --- TODO

impl Args {
    fn into_compat(self) -> ArgsCompat {
        match self {
            Args::FreeSpace(x) => ArgsCompat::FreeSpace(x.into()),
            Args::GroupGet(x) => ArgsCompat::GroupGet(x.into()),
            Args::GroupSet(x) => ArgsCompat::GroupSet(x.into()),
            Args::SessionGet(x) => ArgsCompat::SessionGet(x.into()),
            Args::SessionSet(x) => ArgsCompat::SessionSet(x.into()),
            Args::QueueMove(x) => ArgsCompat::QueueMove(x.into()),
            Args::TorrentGet(x) => {
                let snake_case = TorrentGetArgs {
                    fields: x.fields
                        .map(|fields| fields
                            .into_iter()
                            .map(|f| ccase!(snake, f))
                            .collect()),
                    ids: x.ids,
                };
                ArgsCompat::TorrentGet(snake_case.into())
            },
            Args::TorrentAction(x) => ArgsCompat::TorrentAction(x.into()),
            Args::TorrentRemove(x) => ArgsCompat::TorrentRemove(x.into()),
            Args::TorrentAdd(x) => ArgsCompat::TorrentAdd(x.into()),
            Args::TorrentSet(x) => ArgsCompat::TorrentSet(x.into()),
            Args::TorrentSetLocation(x) => ArgsCompat::TorrentSetLocation(x.into()),
            Args::TorrentRenamePath(x) => ArgsCompat::TorrentRenamePath(x.into()),
        }
    }
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct FreeSpaceArgs {
    path: String,
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub(crate) struct GroupGetArgs {
    groups: Option<Vec<String>>,
}

impl From<Vec<String>> for GroupGetArgs {
    fn from(value: Vec<String>) -> Self {
        Some(value).into()
    }
}

impl From<Option<Vec<String>>> for GroupGetArgs {
    fn from(value: Option<Vec<String>>) -> Self {
        Self {
            groups: value,
        }
    }
}

/// Defines request arguments for the [`group_set`] method.
///
/// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17).
///
/// # Constructor
///
/// * [`GroupSetArgs::new`] creates an empty `GroupSetArgs` object with the given group name.
///
/// # Setters
///
/// The following methods are fluent setters, returning a new `GroupSetArgs` instance modifying
/// only the corresponding field while leaving all other fields untouched.
///
/// * [`GroupSetArgs::honors_session_limits`]: Whether the session's upload limits are honored.
/// * [`GroupSetArgs::speed_limit_down_enabled`]: Whether the bandwidth group limits download
/// speed.
/// * [`GroupSetArgs::speed_limit_down`]: Maximum download speed (`KBps`).
/// * [`GroupSetArgs::speed_limit_up_enabled`]: Whether the bandwidth group limits upload speed.
/// * [`GroupSetArgs::speed_limit_up`]: Maximum upload speed (`KBps`).
///
/// # Examples
///
/// With fluent setters:
/// ```
/// use transmission_rpc::types::GroupSetArgs;
///
/// let args = GroupSetArgs()::new("my-bandwidth-group".to_owned())
///                .honors_session_limits(false)
///                .speed_limit_up_enabled(true)
///                .speed_limit_up(500);
/// ```
///
/// Directly setting struct fields:
/// ```
/// use transmission_rpc::types::GroupSetArgs;
///
/// let mut args = GroupSetArgs()::new("my-bandwidth-group".to_owned());
/// args.honors_session_limits = Some(false);
/// args.speed_limit_up_enabled = Some(true);
/// args.speed_limit_up = Some(500);
/// ```
///
/// [`group_set`]: crate::TransClient::group_set
#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GroupSetArgs {
    #[serde(skip_serializing_if = "Option::is_none", rename = "honorsSessionLimits")]
    pub honors_session_limits: Option<bool>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up: Option<u64>,
}

#[generate_semver_600_compat]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SessionSetArgs {
    pub alt_speed_down: Option<i32>,
    pub alt_speed_enabled: Option<bool>,
    pub alt_speed_time_begin: Option<i32>,
    pub alt_speed_time_day: Option<AltSpeedDay>,
    pub alt_speed_time_enabled: Option<bool>,
    pub alt_speed_time_end: Option<i32>,
    pub alt_speed_up: Option<i32>,

    /// "Enable a very basic brute force protection for the RPC server. See
    ///  [anti_brute_force_threshold] below."
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    ///
    /// [anti_brute_force_threshold]: Self::anti_brute_force_threshold
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub anti_brute_force_enabled: Option<bool>,

    /// "After this amount of failed authentication attempts is surpassed, the RPC server will deny
    ///  any further authentication attempts until it is restarted. This is not tracked per IP but
    ///  in total."
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub anti_brute_force_threshold: Option<i32>,

    pub blocklist_enabled: Option<bool>,
    pub blocklist_url: Option<String>,

    /// Legacy (pre-semver-6.0.0) version of `cache_size_mib`. Use this if the rpc server you are
    /// communicating with is running a Transmission version less than 4.1.0 (before semver 6.0.0).
    ///
    /// > ⚠ Deprecated in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?)
    ///
    /// [`cache_size_mib`]: Self::cache_size_mib
    pub cache_size_mb: Option<i32>,

    /// Transmission 4.1.0 (`rpc_version_semver` 6.0.0) version of [`cache_size_mb`].
    ///
    /// > ⚠ Deprecated in Transmission 4.2.0 (`rpc_version_semver` 6.1.0, `rpc_version`: ?)
    ///
    /// [`cache_size_mb`]: Self::cache_size_mb
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub cache_size_mib: Option<i32>,

    pub default_trackers: Option<String>,
    pub dht_enabled: Option<bool>,
    pub download_dir: Option<String>,
    pub download_queue_enabled: Option<bool>,
    pub download_queue_size: Option<i32>,
    #[compat_with(EncryptionCompat)]
    pub encryption: Option<Encryption>, // TODO: #[use_compat(Encryption600Compat)]
    pub idle_seeding_limit_enabled: Option<bool>,
    pub idle_seeding_limit: Option<i32>,
    pub incomplete_dir_enabled: Option<bool>,
    pub incomplete_dir: Option<String>,
    pub lpd_enabled: Option<bool>,
    pub peer_limit_global: Option<i32>,
    pub peer_limit_per_torrent: Option<i32>,
    pub peer_port_random_on_start: Option<bool>,
    pub peer_port: Option<u16>,
    pub pex_enabled: Option<bool>,
    pub port_forwarding_enabled: Option<bool>,

    /// "List your preference of transport protocols in the order of preferred-first. Omitting the
    ///  transport protocol from the list will disable it. *Note: Never disable TCP when you also
    ///  disable µTP, because then your client would not be able to communicate. Disabling TCP
    ///  might also break webseeds.*"
    ///
    ///  > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub preferred_transports: Option<Vec<Transport>>,

    pub queue_stalled_enabled: Option<bool>,
    pub queue_stalled_minutes: Option<i32>,
    pub rename_partial_files: Option<bool>,
    pub reqq: Option<i32>,
    pub script_torrent_added_enabled: Option<bool>,
    pub script_torrent_added_filename: Option<String>,
    pub script_torrent_done_enabled: Option<bool>,
    pub script_torrent_done_filename: Option<String>,
    pub script_torrent_done_seeding_enabled: Option<bool>,
    pub script_torrent_done_seeding_filename: Option<String>,
    pub seed_queue_enabled: Option<bool>,
    pub seed_queue_size: Option<i32>,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,

    /// "true means sequential download is enabled by default for added torrents"
    ///
    /// > Added in Transmission 4.1.0 (`rpc-version-semver` 5.4.0, `rpc-version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub sequential_download: Option<bool>,

    pub speed_limit_down_enabled: Option<bool>,
    pub speed_limit_down: Option<i32>,
    pub speed_limit_up_enabled: Option<bool>,
    pub speed_limit_up: Option<i32>,
    pub start_added_torrents: Option<bool>,
    pub trash_original_torrent_files: Option<bool>,
    pub utp_enabled: Option<bool>,
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct QueueMoveArgs {
    ids: Vec<Id>,
}

impl From<Vec<Id>> for QueueMoveArgs {
    fn from(ids: Vec<Id>) -> Self {
        Self { ids }
    }
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentGetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<Id>>,
}

impl Default for TorrentGetArgs {
    fn default() -> Self {
        let all_fields = all::<TorrentGetField>().map(|it| it.to_str()).collect();
        TorrentGetArgs {
            fields: Some(all_fields),
            ids: None,
        }
    }
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentActionArgs {
    ids: Vec<Id>,
}
#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    #[serde(rename = "delete-local-data")]
    delete_local_data: bool,
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "move")]
    move_from: Option<bool>,
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentRenamePathArgs {
    ids: Vec<Id>,
    path: String,
    name: String,
}

#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
#[cfg_attr(feature = "tor-get-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "tor-get-serde", serde(rename_all = "camelCase"))]
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
    #[cfg_attr(feature = "tor-get-serde", serde(rename = "file-count"))]
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
    #[cfg_attr(feature = "tor-get-serde", serde(rename = "peer-limit"))]
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
    #[cfg_attr(feature = "tor-get-serde", serde(rename = "primary-mime-type"))]
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

impl TorrentGetField {
    #[must_use]
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::ActivityDate => "activityDate",
            TorrentGetField::AddedDate => "addedDate",
            TorrentGetField::Availability => "availability",
            TorrentGetField::BandwidthPriority => "bandwidthPriority",
            TorrentGetField::Comment => "comment",
            TorrentGetField::CorruptEver => "corruptEver",
            TorrentGetField::Creator => "creator",
            TorrentGetField::DateCreated => "dateCreated",
            TorrentGetField::DesiredAvailable => "desiredAvailable",
            TorrentGetField::DoneDate => "doneDate",
            TorrentGetField::DownloadDir => "downloadDir",
            TorrentGetField::DownloadedEver => "downloadedEver",
            TorrentGetField::DownloadLimit => "downloadLimit",
            TorrentGetField::DownloadLimited => "downloadLimited",
            TorrentGetField::EditDate => "editDate",
            TorrentGetField::Error => "error",
            TorrentGetField::ErrorString => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::EtaIdle => "etaIdle",
            TorrentGetField::FileCount => "file-count",
            TorrentGetField::FileStats => "fileStats",
            TorrentGetField::Files => "files",
            TorrentGetField::Group => "group",
            TorrentGetField::HashString => "hashString",
            TorrentGetField::HaveUnchecked => "haveUnchecked",
            TorrentGetField::HaveValid => "haveValid",
            TorrentGetField::HonorsSessionLimits => "honorsSessionLimits",
            TorrentGetField::Id => "id",
            TorrentGetField::IsFinished => "isFinished",
            TorrentGetField::IsPrivate => "isPrivate",
            TorrentGetField::IsStalled => "isStalled",
            TorrentGetField::Labels => "labels",
            TorrentGetField::LeftUntilDone => "leftUntilDone",
            TorrentGetField::MagnetLink => "magnetLink",
            TorrentGetField::ManualAnnounceTime => "manualAnnounceTime",
            TorrentGetField::MaxConnectedPeers => "maxConnectedPeers",
            TorrentGetField::MetadataPercentComplete => "metadataPercentComplete",
            TorrentGetField::Name => "name",
            TorrentGetField::PeerLimit => "peer-limit",
            TorrentGetField::Peers => "peers",
            TorrentGetField::PeersConnected => "peersConnected",
            TorrentGetField::PeersFrom => "peersFrom",
            TorrentGetField::PeersGettingFromUs => "peersGettingFromUs",
            TorrentGetField::PeersSendingToUs => "peersSendingToUs",
            TorrentGetField::PercentComplete => "percentComplete",
            TorrentGetField::PercentDone => "percentDone",
            TorrentGetField::Pieces => "pieces",
            TorrentGetField::PieceCount => "pieceCount",
            TorrentGetField::PieceSize => "pieceSize",
            TorrentGetField::Priorities => "priorities",
            TorrentGetField::PrimaryMimeType => "primary-mime-type",
            TorrentGetField::QueuePosition => "queuePosition",
            TorrentGetField::RateDownload => "rateDownload",
            TorrentGetField::RateUpload => "rateUpload",
            TorrentGetField::RecheckProgress => "recheckProgress",
            TorrentGetField::SecondsDownloading => "secondsDownloading",
            TorrentGetField::SecondsSeeding => "secondsSeeding",
            TorrentGetField::SeedIdleLimit => "seedIdleLimit",
            TorrentGetField::SeedIdleMode => "seedIdleMode",
            TorrentGetField::SeedRatioLimit => "seedRatioLimit",
            TorrentGetField::SeedRatioMode => "seedRatioMode",
            TorrentGetField::SequentialDownload => "sequential_download",
            TorrentGetField::SizeWhenDone => "sizeWhenDone",
            TorrentGetField::StartDate => "startDate",
            TorrentGetField::Status => "status",
            TorrentGetField::TorrentFile => "torrentFile",
            TorrentGetField::TotalSize => "totalSize",
            TorrentGetField::Trackers => "trackers",
            TorrentGetField::TrackerList => "trackerList",
            TorrentGetField::TrackerStats => "trackerStats",
            TorrentGetField::UploadRatio => "uploadRatio",
            TorrentGetField::UploadedEver => "uploadedEver",
            TorrentGetField::UploadLimit => "uploadLimit",
            TorrentGetField::UploadLimited => "uploadLimited",
            TorrentGetField::Wanted => "wanted",
            TorrentGetField::Webseeds => "webseeds",
            TorrentGetField::WebseedsSendingToUs => "webseedsSendingToUs",
        }
        .to_string()
    }
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackerList(pub Vec<String>);

impl Serialize for TrackerList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.join("\n").serialize(serializer)
    }
}

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
///     > Added in Transmission 4.1.0 (`rpc-version-semver` 5.4.0, `rpc-version`: 18).
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
#[generate_semver_600_compat]
#[derive(Serialize, Debug, Clone, Default, PartialEq)]
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
    ids: Option<Vec<Id>>,

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
