use std::fmt::{self, Display};

use enum_iterator::all;
use serde::{Serialize, Serializer};

use compat_macros::SemverCompat;

use crate::json_rpc::{JsonRpcId, JsonRpcRequest};
use super::{
    AltSpeedDay, Encryption, EncryptionCompat, Id, IpProtocol, MinutesAfterMidnight, Tag,
    Transport,
};

pub(crate) use group_set::*; // GroupSetArgs
pub(crate) use session_get::*; // SessionGetArgs
pub(crate) use session_set::*; // SessionSetArgs
pub(crate) use torrent_add::*; // TorrentAddArgs
pub(crate) use torrent_get::*; // TorrentGetArgs
pub(crate) use torrent_set::*; // TorrentSetArgs

pub use group_set::GroupSetArgs;
pub use session_get::SessionGetField;
pub use session_set::SessionSetArgs;
pub use torrent_add::TorrentAddArgs;
pub use torrent_get::TorrentGetField;
pub use torrent_set::{TorrentSetArgs, TrackerReplaceArgs, TrackerReplacePair};

mod group_set;
mod into;
mod session_get;
mod session_set;
mod torrent_add;
mod torrent_get;
mod torrent_set;

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
                    method: self.method.into_compat(),
                    params: self.arguments.as_ref()
                        // Cloning the request arguments shouldn't be too costly...
                        // Not ideal, but maybe this gets optimized away anyway?
                        .cloned()
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

    pub fn port_test(args: PortTestArgs, tag: Option<Tag>) -> RpcRequest {
        RpcRequest {
            method: Method::PortTest,
            arguments: Some(args.into()),
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
            method: action.into(),
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

#[derive(SemverCompat, Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Method {
    BlocklistUpdate,
    FreeSpace,
    GroupGet,
    GroupSet,
    PortTest,
    QueueMoveBottom,
    QueueMoveDown,
    QueueMoveTop,
    QueueMoveUp,
    SessionClose,
    SessionGet,
    SessionSet,
    SessionStats,
    TorrentAdd,
    TorrentGet,
    TorrentReannounce,
    TorrentRemove,
    TorrentRenamePath,
    TorrentSet,
    TorrentSetLocation,
    TorrentStart,
    TorrentStartNow,
    TorrentStop,
    TorrentVerify,
}

impl From<TorrentAction> for Method {
    fn from(action: TorrentAction) -> Self {
        match action {
            TorrentAction::Start => Method::TorrentStart,
            TorrentAction::StartNow => Method::TorrentStartNow,
            TorrentAction::Stop => Method::TorrentStop,
            TorrentAction::Verify => Method::TorrentVerify,
            TorrentAction::Reannounce => Method::TorrentReannounce,
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = serde_json::to_string(self)
            .map_err(|_| fmt::Error)?;
        write!(f, "{as_str}")
    }
}

pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField {}

#[derive(SemverCompat, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Args {
    #[compat(type = "_")]
    FreeSpace(FreeSpaceArgs),
    #[compat(type = _)]
    GroupGet(GroupGetArgs),
    #[compat(type = _)]
    GroupSet(GroupSetArgs),
    #[compat(type = _)]
    PortTest(PortTestArgs),
    #[compat(type = _)]
    SessionGet(SessionGetArgs),
    #[compat(type = _)]
    SessionSet(SessionSetArgs),
    #[compat(type = _)]
    QueueMove(QueueMoveArgs),
    #[compat(type = _)]
    TorrentGet(TorrentGetArgs),
    #[compat(type = _)]
    TorrentAction(TorrentActionArgs),
    #[compat(type = _)]
    TorrentRemove(TorrentRemoveArgs),
    #[compat(type = _)]
    TorrentAdd(TorrentAddArgs),
    #[compat(type = _)]
    TorrentSet(TorrentSetArgs),
    #[compat(type = _)]
    TorrentSetLocation(TorrentSetLocationArgs),
    #[compat(type = _)]
    TorrentRenamePath(TorrentRenamePathArgs),
}

#[derive(SemverCompat, Serialize, Debug, Clone)]
pub struct FreeSpaceArgs {
    path: String,
}

#[derive(SemverCompat, Serialize, Debug, Clone)]
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

/// Request arguments of a [`port_test`] query.
///
/// [`port_test`]: crate::TransClient::port_test
#[derive(SemverCompat, Serialize, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PortTestArgs {
    /// Specifies the IP version to use for the port test. For backwards compatibility, it is
    /// allowed to omit this parameter to get the behavior before Transmission `4.1.0`
    /// (`rpc-version-semver` 6.0.0), which is to check whichever IP version the OS happened to use
    /// to connect to the port test service.
    ///
    /// > Added in Transmission 4.1.0 (`rpc_version_semver` 6.0.0, `rpc_version`: 18)
    #[serde(skip_serializing)] // Doesn't exist pre- semver-6.0.0
    pub ip_protocol: Option<IpProtocol>,
}

impl PortTestArgs {
    /// Constructs a new `PortTestArgs`. This is an alias for [`Default::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Fluently sets the `ip_protocol` [`port_test`] argument leaving all other fields untouched.
    ///
    /// [`port_test`]: crate::TransClient::port_test
    pub fn ip_protocol(mut self, proto: IpProtocol) -> Self {
        self.ip_protocol = Some(proto);
        self
    }
}

#[derive(SemverCompat, Serialize, Debug, Clone)]
pub struct QueueMoveArgs {
    ids: Vec<Id>,
}

impl<I: IntoIterator<Item = Id>> From<I> for QueueMoveArgs {
    fn from(ids: I) -> Self {
        Self { ids: ids.into_iter().collect() }
    }
}

#[derive(SemverCompat, Serialize, Debug, Clone)]
pub struct TorrentActionArgs {
    ids: Vec<Id>,
}

impl<I: IntoIterator<Item = Id>> From<I> for TorrentActionArgs {
    fn from(ids: I) -> Self {
        Self { ids: ids.into_iter().collect() }
    }
}

#[derive(SemverCompat, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    delete_local_data: bool,
}

#[derive(SemverCompat, Serialize, Debug, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    r#move: bool,
}

/// Request arguments of a [`torrent_rename_path`] query.
///
/// [`torrent_rename_path`]: crate::TransClient::torrent_rename_path
#[derive(SemverCompat, Serialize, Debug, Clone)]
pub struct TorrentRenamePathArgs {
    ids: Vec<Id>,
    path: String,
    name: String,
}

/// The kind of [`torrent_action`] request.
///
/// [`torrent_action`]: crate::TransClient::torrent_action
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TorrentAction {
    Start,
    StartNow,
    Stop,
    Verify,
    Reannounce,
}

impl Display for TorrentAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: This will only emit pre- semver-6.0.0 names.
        write!(f, "{}", match self {
            Self::Start => "torrent-start",
            Self::StartNow => "torrent-start-now",
            Self::Stop => "torrent-stop",
            Self::Verify => "torrent-verify",
            Self::Reannounce => "torrent-reannounce",
        })
    }
}

impl TorrentAction {
    /// This is an alias for [`ToString::to_string`].
    ///
    /// This method exists so that application code written prior to [`TorrentAction`] implementing
    /// [`Display`] does not break.
    #[must_use]
    pub fn to_str(&self) -> String {
        self.to_string()
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
    fn request_port_test_legacy() -> Result<()> {
        let args = PortTestArgs { ip_protocol: Some(IpProtocol::Ipv4) };
        verify(args, None, "")
    }

    #[test]
    fn request_port_test_semver_600() -> Result<()> {
        let args = PortTestArgs { ip_protocol: Some(IpProtocol::Ipv6) };
        verify(args, Some(JSON_RPC_VERSION_2_0), "\"ip_protocol\":\"ipv6\"")
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

    // ---------------------------------------------------------------------------------------------

    fn serialize_method_legacy(method: Method) -> Result<String> {
        serde_json::to_string(&method)
            .map_err(Into::into)
    }

    fn serialize_method_semver_600(method: Method) -> Result<String> {
        serde_json::to_string(&method.into_compat())
            .map_err(Into::into)
    }

    #[test]
    fn request_method_legacy_blocklist_update() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::BlocklistUpdate)?, "\"blocklist-update\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_blocklist_update() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::BlocklistUpdate)?, "\"blocklist_update\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_free_space() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::FreeSpace)?, "\"free-space\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_free_space() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::FreeSpace)?, "\"free_space\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_group_get() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::GroupGet)?, "\"group-get\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_group_get() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::GroupGet)?, "\"group_get\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_group_set() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::GroupSet)?, "\"group-set\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_group_set() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::GroupSet)?, "\"group_set\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_port_test() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::PortTest)?, "\"port-test\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_port_test() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::PortTest)?, "\"port_test\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_queue_move_bottom() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::QueueMoveBottom)?, "\"queue-move-bottom\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_queue_move_bottom() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::QueueMoveBottom)?, "\"queue_move_bottom\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_queue_move_down() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::QueueMoveDown)?, "\"queue-move-down\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_queue_move_down() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::QueueMoveDown)?, "\"queue_move_down\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_queue_move_top() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::QueueMoveTop)?, "\"queue-move-top\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_queue_move_top() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::QueueMoveTop)?, "\"queue_move_top\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_queue_move_up() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::QueueMoveUp)?, "\"queue-move-up\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_queue_move_up() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::QueueMoveUp)?, "\"queue_move_up\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_session_close() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::SessionClose)?, "\"session-close\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_session_close() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::SessionClose)?, "\"session_close\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_session_get() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::SessionGet)?, "\"session-get\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_session_get() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::SessionGet)?, "\"session_get\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_session_set() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::SessionSet)?, "\"session-set\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_session_set() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::SessionSet)?, "\"session_set\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_session_stats() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::SessionStats)?, "\"session-stats\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_session_stats() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::SessionStats)?, "\"session_stats\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_add() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentAdd)?, "\"torrent-add\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_add() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentAdd)?, "\"torrent_add\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_get() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentGet)?, "\"torrent-get\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_get() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentGet)?, "\"torrent_get\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_reannounce() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentReannounce)?, "\"torrent-reannounce\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_reannounce() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentReannounce)?,
            "\"torrent_reannounce\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_remove() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentRemove)?, "\"torrent-remove\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_remove() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentRemove)?, "\"torrent_remove\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_rename_path() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentRenamePath)?, "\"torrent-rename-path\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_rename_path() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentRenamePath)?,
            "\"torrent_rename_path\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_set() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentSet)?, "\"torrent-set\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_set() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentSet)?, "\"torrent_set\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_set_location() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentSetLocation)?,
            "\"torrent-set-location\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_set_location() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentSetLocation)?,
            "\"torrent_set_location\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_start() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentStart)?, "\"torrent-start\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_start() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentStart)?, "\"torrent_start\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_start_now() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentStartNow)?, "\"torrent-start-now\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_start_now() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentStartNow)?, "\"torrent_start_now\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_stop() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentStop)?, "\"torrent-stop\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_stop() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentStop)?, "\"torrent_stop\"");
        Ok(())
    }

    #[test]
    fn request_method_legacy_torrent_verify() -> Result<()> {
        assert_eq!(serialize_method_legacy(Method::TorrentVerify)?, "\"torrent-verify\"");
        Ok(())
    }

    #[test]
    fn request_method_semver_600_torrent_verify() -> Result<()> {
        assert_eq!(serialize_method_semver_600(Method::TorrentVerify)?, "\"torrent_verify\"");
        Ok(())
    }
}
