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

#[cfg(test)]
mod serde_tests {
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
            session_count: Some(1024),
            uploaded_bytes: 123000456,
        });
        assert_eq!(resp.arguments.current_stats, Stats {
            downloaded_bytes: 50,
            files_added: 1,
            seconds_active: 444,
            session_count: Some(1),
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
            session_count: Some(301),
            uploaded_bytes: 3,
        });
        assert_eq!(resp.arguments.current_stats, Stats {
            downloaded_bytes: 9999,
            files_added: 555,
            seconds_active: 454545,
            session_count: Some(1),
            uploaded_bytes: 30,
        });
        assert_eq!(resp.arguments.download_speed, 0);
        assert_eq!(resp.arguments.paused_torrent_count, 0);
        assert_eq!(resp.arguments.torrent_count, 3);
        assert_eq!(resp.arguments.upload_speed, 1000000);

        Ok(())
    }
}
