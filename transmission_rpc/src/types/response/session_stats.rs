use serde::Deserialize;

/// Request arguments of a [`session_stats`] query.
/// 
/// [`session_stats`]: crate::TransClient::session_stats
#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    /// The number of started torrents.
    #[serde(alias = "active_torrent_count")]
    pub active_torrent_count: u64,
    /// [`Stats`] counts that the transmission instance has ever tracked.
    ///
    /// > Added in Transmission 1.50 (`rpc-version-semver` 1.3.0, `rpc-version`: 4)
    #[serde(alias = "cumulative-stats")]
    #[serde(alias = "cumulative_stats")]
    pub cumulative_stats: Stats,
    /// [`Stats`] counts that the transmission instance has tracked since it was started.
    ///
    /// > Added in Transmission 1.50 (`rpc-version-semver` 1.3.0, `rpc-version`: 4)
    #[serde(alias = "current-stats")]
    #[serde(alias = "current_stats")]
    pub current_stats: Stats,
    /// The current overall download speed.
    #[serde(alias = "download_speed")]
    pub download_speed: u64,
    /// The number of paused (stopped) torrents.
    #[serde(alias = "paused_torrent_count")]
    pub paused_torrent_count: u64,
    /// The total number of torrents.
    #[serde(alias = "torrent_count")]
    pub torrent_count: u64,
    /// The current overall upload speed.
    #[serde(alias = "upload_speed")]
    pub upload_speed: u64,
}

#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// Total number of downloaded bytes.
    #[serde(alias = "downloaded_bytes")]
    pub downloaded_bytes: u64,
    /// Number of files added.
    #[serde(alias = "files_added")]
    pub files_added: u64,
    /// Number of seconds transmission has been running.
    #[serde(alias = "seconds_active")]
    pub seconds_active: u64,
    /// Number of times the transmission program has been started.
    #[serde(alias = "session_count")]
    pub session_count: u64,
    /// Total number of uploaded bytes.
    #[serde(alias = "uploaded_bytes")]
    pub uploaded_bytes: u64,
}

#[cfg(test)]
mod legacy_deser_tests {
    use test_case::test_case;

    use super::*;

    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "legacy stats ok"
    )]

    #[test_case(r#"{
            "downloadedBytes": 0,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => Stats {
            downloaded_bytes: 0,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "legacy stats downloaded bytes zero"
    )]
    #[test_case(r#"{
            "downloadedBytes": -1,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy stats downloaded bytes negative"
    )]

    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 0,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 0,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "legacy stats files added zero"
    )]
    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": -1,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy stats files added negative"
    )]

    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": 0,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 0,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "legacy stats seconds active zero"
    )]
    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": -1,
            "sessionCount": 3,
            "uploadedBytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy stats seconds active negative"
    )]

    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": 0,
            "uploadedBytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 0,
            uploaded_bytes: 1,
        } ; "legacy stats session count zero"
    )]
    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": -1,
            "uploadedBytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy stats session count negative"
    )]

    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": 0
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 0,
        } ; "legacy stats uploaded bytes zero"
    )]
    #[test_case(r#"{
            "downloadedBytes": 123456,
            "filesAdded": 1234560000,
            "secondsActive": 60606060,
            "sessionCount": 3,
            "uploadedBytes": -1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy stats uploaded bytes negative"
    )]

    fn stats_deserialize(repr: &str) -> Stats {
        match serde_json::from_str(repr) {
            Ok(stats) => stats,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 10,
        } ; "legacy session stats ok"
    )]

    #[test_case(r#"{
            "activeTorrentCount": 0,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => SessionStats {
            active_torrent_count: 0,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 10,
        } ; "legacy session stats active torrent count zero"
    )]
    #[test_case(r#"{
            "activeTorrentCount": -1,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy session stats active torrent count negative"
    )]

    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 0,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 0,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 10,
        } ; "legacy session stats download speed zero"
    )]
    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": -1,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy session stats download speed negative"
    )]

    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 0,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 0,
            torrent_count: 444,
            upload_speed: 10,
        } ; "legacy session stats paused torrent count zero"
    )]
    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": -1,
            "torrentCount": 444,
            "uploadSpeed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy session stats paused torrent count negative"
    )]

    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": 0,
            "uploadSpeed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 0,
            upload_speed: 10,
        } ; "legacy session stats torrent count zero"
    )]
    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": -1,
            "uploadSpeed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy session stats torrent count negative"
    )]

    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": 0
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 0,
        } ; "legacy session stats upload speed zero"
    )]
    #[test_case(r#"{
            "activeTorrentCount": 123,
            "cumulative-stats": {
                "downloadedBytes": 104,
                "filesAdded": 666,
                "secondsActive": 456000123,
                "sessionCount": 1024,
                "uploadedBytes": 123000456
            },
            "current-stats": {
                "downloadedBytes": 50,
                "filesAdded": 1,
                "secondsActive": 444,
                "sessionCount": 1,
                "uploadedBytes": 9999
            },
            "downloadSpeed": 4200,
            "pausedTorrentCount": 321,
            "torrentCount": 444,
            "uploadSpeed": -1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "legacy session stats upload speed negative"
    )]

    fn session_stats_deserialize(repr: &str) -> SessionStats {
        match serde_json::from_str(repr) {
            Ok(stats) => stats,
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod semver_600_deser_tests {
    use test_case::test_case;

    use super::*;

    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "semver 6.0.0 stats ok"
    )]

    #[test_case(r#"{
            "downloaded_bytes": 0,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => Stats {
            downloaded_bytes: 0,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "semver 6.0.0 stats downloaded bytes zero"
    )]
    #[test_case(r#"{
            "downloaded_bytes": -1,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 stats downloaded bytes negative"
    )]

    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 0,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 0,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "semver 6.0.0 stats files added zero"
    )]
    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": -1,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 stats files added negative"
    )]

    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": 0,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 0,
            session_count: 3,
            uploaded_bytes: 1,
        } ; "semver 6.0.0 stats seconds active zero"
    )]
    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": -1,
            "session_count": 3,
            "uploaded_bytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 stats seconds active negative"
    )]

    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": 0,
            "uploaded_bytes": 1
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 0,
            uploaded_bytes: 1,
        } ; "semver 6.0.0 stats session count zero"
    )]
    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": -1,
            "uploaded_bytes": 1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 stats session count negative"
    )]

    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": 0
        }"# => Stats {
            downloaded_bytes: 123456,
            files_added: 1234560000,
            seconds_active: 60606060,
            session_count: 3,
            uploaded_bytes: 0,
        } ; "semver 6.0.0 stats uploaded bytes zero"
    )]
    #[test_case(r#"{
            "downloaded_bytes": 123456,
            "files_added": 1234560000,
            "seconds_active": 60606060,
            "session_count": 3,
            "uploaded_bytes": -1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 stats uploaded bytes negative"
    )]

    fn stats_deserialize(repr: &str) -> Stats {
        match serde_json::from_str(repr) {
            Ok(stats) => stats,
            Err(err) => panic!("{err}"),
        }
    }

    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 10,
        } ; "semver 6.0.0 session stats ok"
    )]

    // NOTE: No missing (cumulative_stats, current_stats) tests because these fields were added
    // NOTE- before semver-6.0.0 (and so should never be missing).

    #[test_case(r#"{
            "active_torrent_count": 0,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => SessionStats {
            active_torrent_count: 0,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 10,
        } ; "semver 6.0.0 session stats active torrent count zero"
    )]
    #[test_case(r#"{
            "active_torrent_count": -1,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 session stats active torrent count negative"
    )]

    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 0,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 0,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 10,
        } ; "semver 6.0.0 session stats download speed zero"
    )]
    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": -1,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 session stats download speed negative"
    )]

    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 0,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 0,
            torrent_count: 444,
            upload_speed: 10,
        } ; "semver 6.0.0 session stats paused torrent count zero"
    )]
    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": -1,
            "torrent_count": 444,
            "upload_speed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 session stats paused torrent count negative"
    )]

    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": 0,
            "upload_speed": 10
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 0,
            upload_speed: 10,
        } ; "semver 6.0.0 session stats torrent count zero"
    )]
    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": -1,
            "upload_speed": 10
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 session stats torrent count negative"
    )]

    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": 0
        }"# => SessionStats {
            active_torrent_count: 123,
            cumulative_stats: Stats {
                downloaded_bytes: 104,
                files_added: 666,
                seconds_active: 456000123,
                session_count: 1024,
                uploaded_bytes: 123000456,
            },
            current_stats: Stats {
                downloaded_bytes: 50,
                files_added: 1,
                seconds_active: 444,
                session_count: 1,
                uploaded_bytes: 9999,
            },
            download_speed: 4200,
            paused_torrent_count: 321,
            torrent_count: 444,
            upload_speed: 0,
        } ; "semver 6.0.0 session stats upload speed zero"
    )]
    #[test_case(r#"{
            "active_torrent_count": 123,
            "cumulative_stats": {
                "downloaded_bytes": 104,
                "files_added": 666,
                "seconds_active": 456000123,
                "session_count": 1024,
                "uploaded_bytes": 123000456
            },
            "current_stats": {
                "downloaded_bytes": 50,
                "files_added": 1,
                "seconds_active": 444,
                "session_count": 1,
                "uploaded_bytes": 9999
            },
            "download_speed": 4200,
            "paused_torrent_count": 321,
            "torrent_count": 444,
            "upload_speed": -1
        }"# => panics "invalid value: integer `-1`, expected u64"
        ; "semver 6.0.0 session stats upload speed negative"
    )]

    fn session_stats_deserialize(repr: &str) -> SessionStats {
        match serde_json::from_str(repr) {
            Ok(stats) => stats,
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod response_deser_tests {
    use super::*;
    use crate::{json_rpc::JsonRpcResponse, types::{Result, RpcResponse}};

    #[test]
    fn session_stats_v300() -> Result<()> {
        let resp = serde_json::from_str::<RpcResponse<SessionStats>>(
            r#"
            {
              "arguments": {
                "activeTorrentCount": 321,
                "cumulative-stats": {
                  "downloadedBytes": 104,
                  "filesAdded": 666,
                  "secondsActive": 456000123,
                  "sessionCount": 1024,
                  "uploadedBytes": 123000456
                },
                "current-stats": {
                  "downloadedBytes": 50,
                  "filesAdded": 1,
                  "secondsActive": 444,
                  "sessionCount": 1,
                  "uploadedBytes": 9999
                },
                "downloadSpeed": 10,
                "pausedTorrentCount": 5,
                "torrentCount": 10,
                "uploadSpeed": 20
              },
              "result": "success",
              "tag": 12345
            }
            "#
        )?;

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.active_torrent_count, 321);
        assert_eq!(resp.arguments.cumulative_stats, Stats {
            downloaded_bytes: 104,
            files_added: 666,
            seconds_active: 456000123,
            session_count: 1024,
            uploaded_bytes: 123000456,
        });
        assert_eq!(resp.arguments.current_stats, Stats {
            downloaded_bytes: 50,
            files_added: 1,
            seconds_active: 444,
            session_count: 1,
            uploaded_bytes: 9999,
        });
        assert_eq!(resp.arguments.download_speed, 10);
        assert_eq!(resp.arguments.paused_torrent_count, 5);
        assert_eq!(resp.arguments.torrent_count, 10);
        assert_eq!(resp.arguments.upload_speed, 20);

        Ok(())
    }

    #[test]
    fn session_stats_v411() -> Result<()> {
        let resp: RpcResponse<_> = serde_json::from_str::<JsonRpcResponse<SessionStats>>(
            r#"
            {
              "id": 12345,
              "jsonrpc": "2.0",
              "result": {
                "active_torrent_count": 456,
                "cumulative_stats": {
                  "downloaded_bytes": 12167436971470,
                  "files_added": 100000,
                  "seconds_active": 567000000000000,
                  "session_count": 301,
                  "uploaded_bytes": 3
                },
                "current_stats": {
                  "downloaded_bytes": 9999,
                  "files_added": 555,
                  "seconds_active": 454545,
                  "session_count": 1,
                  "uploaded_bytes": 30
                },
                "download_speed": 0,
                "paused_torrent_count": 0,
                "torrent_count": 3,
                "upload_speed": 1000000
              }
            }
            "#
        )?
        .into();

        println!("{resp:#?}");
        assert!(resp.is_ok());

        assert_eq!(resp.arguments.active_torrent_count, 456);
        assert_eq!(resp.arguments.cumulative_stats, Stats {
            downloaded_bytes: 12167436971470,
            files_added: 100000,
            seconds_active: 567000000000000,
            session_count: 301,
            uploaded_bytes: 3,
        });
        assert_eq!(resp.arguments.current_stats, Stats {
            downloaded_bytes: 9999,
            files_added: 555,
            seconds_active: 454545,
            session_count: 1,
            uploaded_bytes: 30,
        });
        assert_eq!(resp.arguments.download_speed, 0);
        assert_eq!(resp.arguments.paused_torrent_count, 0);
        assert_eq!(resp.arguments.torrent_count, 3);
        assert_eq!(resp.arguments.upload_speed, 1000000);

        Ok(())
    }
}
