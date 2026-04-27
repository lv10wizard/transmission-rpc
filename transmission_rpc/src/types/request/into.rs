use super::{
    Args,
    FreeSpaceArgs,
    GroupGetArgs,
    GroupSetArgs,
    Method,
    PortTestArgs,
    QueueMoveArgs,
    RpcRequest,
    SessionGetArgs,
    SessionSetArgs,
    TorrentActionArgs,
    TorrentAddArgs,
    TorrentGetArgs,
    TorrentRemoveArgs,
    TorrentRenamePathArgs,
    TorrentSetArgs,
    TorrentSetLocationArgs,
};

impl From<FreeSpaceArgs> for Args {
    fn from(value: FreeSpaceArgs) -> Self {
        Self::FreeSpace(value)
    }
}

impl From<FreeSpaceArgs> for RpcRequest {
    fn from(value: FreeSpaceArgs) -> Self {
        Self {
            method: Method::FreeSpace,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<GroupGetArgs> for Args {
    fn from(value: GroupGetArgs) -> Self {
        Self::GroupGet(value)
    }
}

impl From<GroupGetArgs> for RpcRequest {
    fn from(value: GroupGetArgs) -> Self {
        Self {
            method: Method::GroupGet,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<GroupSetArgs> for Args {
    fn from(value: GroupSetArgs) -> Self {
        Self::GroupSet(value)
    }
}

impl From<GroupSetArgs> for RpcRequest {
    fn from(value: GroupSetArgs) -> Self {
        Self {
            method: Method::GroupSet,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<PortTestArgs> for Args {
    fn from(value: PortTestArgs) -> Self {
        Self::PortTest(value)
    }
}

impl From<PortTestArgs> for RpcRequest {
    fn from(value: PortTestArgs) -> Self {
        RpcRequest::port_test(value, None)
    }
}

impl From<QueueMoveArgs> for Args {
    fn from(value: QueueMoveArgs) -> Self {
        Self::QueueMove(value)
    }
}

impl From<SessionGetArgs> for Args {
    fn from(value: SessionGetArgs) -> Self {
        Self::SessionGet(value)
    }
}

impl From<SessionGetArgs> for RpcRequest {
    fn from(value: SessionGetArgs) -> Self {
        Self {
            method: Method::SessionGet,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<SessionSetArgs> for Args {
    fn from(value: SessionSetArgs) -> Self {
        Self::SessionSet(value)
    }
}

impl From<SessionSetArgs> for RpcRequest {
    fn from(value: SessionSetArgs) -> Self {
        Self {
            method: Method::SessionSet,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<TorrentActionArgs> for Args {
    fn from(value: TorrentActionArgs) -> Self {
        Self::TorrentAction(value)
    }
}

impl From<TorrentAddArgs> for Args {
    fn from(value: TorrentAddArgs) -> Self {
        Self::TorrentAdd(value)
    }
}

impl From<TorrentAddArgs> for RpcRequest {
    fn from(value: TorrentAddArgs) -> Self {
        Self {
            method: Method::TorrentAdd,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<TorrentGetArgs> for Args {
    fn from(value: TorrentGetArgs) -> Self {
        Self::TorrentGet(value)
    }
}

impl From<TorrentGetArgs> for RpcRequest {
    fn from(value: TorrentGetArgs) -> Self {
        Self {
            method: Method::TorrentGet,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<TorrentRemoveArgs> for Args {
    fn from(value: TorrentRemoveArgs) -> Self {
        Self::TorrentRemove(value)
    }
}

impl From<TorrentRemoveArgs> for RpcRequest {
    fn from(value: TorrentRemoveArgs) -> Self {
        Self {
            method: Method::TorrentRemove,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<TorrentRenamePathArgs> for Args {
    fn from(value: TorrentRenamePathArgs) -> Self {
        Self::TorrentRenamePath(value)
    }
}

impl From<TorrentRenamePathArgs> for RpcRequest {
    fn from(value: TorrentRenamePathArgs) -> Self {
        Self {
            method: Method::TorrentRenamePath,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<TorrentSetArgs> for Args {
    fn from(value: TorrentSetArgs) -> Self {
        Self::TorrentSet(value)
    }
}

impl From<TorrentSetArgs> for RpcRequest {
    fn from(value: TorrentSetArgs) -> Self {
        Self {
            method: Method::TorrentSet,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}

impl From<TorrentSetLocationArgs> for Args {
    fn from(value: TorrentSetLocationArgs) -> Self {
        Self::TorrentSetLocation(value)
    }
}

impl From<TorrentSetLocationArgs> for RpcRequest {
    fn from(value: TorrentSetLocationArgs) -> Self {
        Self {
            method: Method::TorrentSetLocation,
            arguments: Some(value.into()),
            tag: None,
        }
    }
}
