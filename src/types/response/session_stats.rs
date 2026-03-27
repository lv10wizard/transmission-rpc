use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    #[serde(alias = "torrent_count")]
    pub torrent_count: i32,
    #[serde(alias = "active_torrent_count")]
    pub active_torrent_count: i32,
    #[serde(alias = "paused_torrent_count")]
    pub paused_torrent_count: i32,
    #[serde(alias = "download_speed")]
    pub download_speed: i64,
    #[serde(alias = "upload_speed")]
    pub upload_speed: i64,
    #[serde(alias = "current-stats")]
    #[serde(alias = "current_stats")]
    pub current_stats: Stats,
    #[serde(alias = "cumulative-stats")]
    #[serde(alias = "cumulative_stats")]
    pub cumulative_stats: Stats,
}

#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(alias = "files_added")]
    pub files_added: i32,
    #[serde(alias = "downloaded_bytes")]
    pub downloaded_bytes: i64,
    #[serde(alias = "uploaded_bytes")]
    pub uploaded_bytes: i64,
    #[serde(alias = "seconds_active")]
    pub seconds_active: i64,
    #[serde(alias = "session_count")]
    pub session_count: Option<i32>,
}
