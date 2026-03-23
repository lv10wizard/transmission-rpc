//! This file defines torrent-get response serde tests.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use chrono::DateTime;
use serde_json;
use url::Url;

use crate::types::response::{TorrentStatus, TrackerState, WebseedsEx};
use crate::types::{
    ErrorType, IdleMode, Priority, RatioMode, Result, RpcResponse, Torrent, Torrents,
};

type TorrentGetResp = RpcResponse<Torrents<Torrent>>;

/// torrent-get test helper to consolidate unit test boilerplate assertions.
///
/// ### Arguments
///
/// * `resp`: The deserialized rpc response (probably created with [`serde_json::from_str`]).
/// * `expected_len`: The number of expected `torrents` in `resp`.
/// * `verify`: Callback to assert the test-specific deserialization data.
fn test_torrent_get<F: Fn(&TorrentGetResp) -> Result<()>>(
    resp: TorrentGetResp,
    expected_len: usize,
    verify: F,
) -> Result<()> {
    println!("{resp:#?}");
    assert!(resp.is_ok());
    assert_eq!(resp.arguments.torrents.len(), expected_len);
    verify(&resp)
}

const EXPECTED_MISSING_LEN: usize = 1;

fn torrent_get_only_id() -> &'static str {
    r#"
    {
        "arguments": {
            "torrents": [
                { "id":123 }
            ]
        },
        "result":"success"
    }
    "#
}

fn torrent_get_only_hash() -> &'static str {
    r#"
    {
        "arguments": {
            "torrents": [
                { "hashString":"e08c426aab2cc58649ae5e73690e3747117b3470" }
            ]
        },
        "result":"success"
    }
    "#
}

// TODO: malformed test, (?)zero length

// ----- activity_date (activityDate, ActivityDate) --------------------

#[test]
fn test_torrent_get_activity_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "activityDate":1718947434 },
                    { "activity_date":1652228910 },
                    { "activityDate":-1 },
                    { "activityDate":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].activity_date,
                Some(DateTime::parse_from_rfc3339("2024-06-21T05:23:54Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[1].activity_date,
                Some(DateTime::parse_from_rfc3339("2022-05-11T00:28:30Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[2].activity_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[3].activity_date,
                Some(DateTime::UNIX_EPOCH)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_activity_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].activity_date, None);
            Ok(())
        },
    )
}

// ----- added_date (addedDate, AddedDate) --------------------

#[test]
fn test_torrent_get_added_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "addedDate":1670612948 },
                    { "added_date":1670612948 },
                    { "addedDate":0 },
                    { "addedDate":-1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].added_date,
                Some(DateTime::parse_from_rfc3339("2022-12-09T19:09:08Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[1].added_date,
                Some(DateTime::parse_from_rfc3339("2022-12-09T19:09:08Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[2].added_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[3].added_date,
                Some(DateTime::UNIX_EPOCH)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_added_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].added_date, None);
            Ok(())
        },
    )
}

// ----- availability (Availability) --------------------

#[test]
fn test_torrent_get_availability_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "availability":[-1,0,1,2,3,10,-1] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].availability, Some(vec![-1,0,1,2,3,10,-1]));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_availability_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].availability, None);
            Ok(())
        },
    )
}

// ----- bandwidth_priority (bandwidthPriority, BandwidthPriority) --------------------

#[test]
fn test_torrent_get_bandwidth_priority() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "bandwidthPriority":-1 },
                    { "bandwidthPriority":0 },
                    { "bandwidthPriority":1 },
                    { "bandwidth_priority":-1 },
                    { "bandwidth_priority":0 },
                    { "bandwidth_priority":1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].bandwidth_priority,
                Some(Priority::Low)
            );
            assert_eq!(
                resp.arguments.torrents[1].bandwidth_priority,
                Some(Priority::Normal)
            );
            assert_eq!(
                resp.arguments.torrents[2].bandwidth_priority,
                Some(Priority::High)
            );
            assert_eq!(
                resp.arguments.torrents[3].bandwidth_priority,
                Some(Priority::Low)
            );
            assert_eq!(
                resp.arguments.torrents[4].bandwidth_priority,
                Some(Priority::Normal)
            );
            assert_eq!(
                resp.arguments.torrents[5].bandwidth_priority,
                Some(Priority::High)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_bandwidth_priority_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].bandwidth_priority, None);
            Ok(())
        },
    )
}

// ----- bytes_completed (BytesCompleted) --------------------

#[test]
fn test_torrent_get_bytes_completed() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "bytes_completed":[] },
                    { "bytes_completed":[790626304] },
                    { "bytesCompleted":[790626304] },
                    { "bytes_completed":[1234,567890,443,8080] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].bytes_completed,
                Some(vec![]),
            );
            assert_eq!(
                resp.arguments.torrents[1].bytes_completed,
                Some(vec![790626304]),
            );
            assert_eq!(
                resp.arguments.torrents[2].bytes_completed,
                Some(vec![790626304]),
            );
            assert_eq!(
                resp.arguments.torrents[3].bytes_completed,
                Some(vec![1234,567890,443,8080]),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_bytes_completed_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].bytes_completed, None);
            Ok(())
        },
    )
}


// ----- comment (Comment) --------------------

#[test]
fn test_torrent_get_comment_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "comment":"lorem ipsum" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].comment,
                Some("lorem ipsum".into())
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_comment_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].comment, None);
            Ok(())
        },
    )
}

// ----- corrupt_ever (corruptEver, CorruptEver) --------------------

#[test]
fn test_torrent_get_corrupt_ever_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "corruptEver":4096 },
                    { "corruptEver":0 },
                    { "corrupt_ever":16384 },
                    { "corrupt_ever":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].corrupt_ever, Some(4096));
            assert_eq!(resp.arguments.torrents[1].corrupt_ever, Some(0));
            assert_eq!(resp.arguments.torrents[2].corrupt_ever, Some(16384));
            assert_eq!(resp.arguments.torrents[3].corrupt_ever, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_corrupt_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].corrupt_ever, None);
            Ok(())
        },
    )
}

// ----- creator (Creator) --------------------

#[test]
fn test_torrent_get_creator_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "creator":"mktorrent 1.1" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].creator,
                Some("mktorrent 1.1".into())
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_creator_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].creator, None);
            Ok(())
        },
    )
}

// ----- date_created (dateCreated, DateCreated) --------------------

#[test]
fn test_torrent_get_date_created_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "dateCreated":1592962706 },
                    { "date_created":1592962706 },
                    { "dateCreated":0 },
                    { "dateCreated":-1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].date_created,
                Some(DateTime::parse_from_rfc3339("2020-06-24T01:38:26Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[1].date_created,
                Some(DateTime::parse_from_rfc3339("2020-06-24T01:38:26Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[2].date_created,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[3].date_created,
                Some(DateTime::UNIX_EPOCH)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_date_created_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].date_created, None);
            Ok(())
        },
    )
}

// ----- desired_available (desiredAvailable, DesiredAvailable) --------------------

#[test]
fn test_torrent_get_desired_available_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "desiredAvailable":20162576 },
                    { "desiredAvailable":0 },
                    { "desired_available":1234567890 },
                    { "desired_available":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].desired_available, Some(20162576));
            assert_eq!(resp.arguments.torrents[1].desired_available, Some(0));
            assert_eq!(resp.arguments.torrents[2].desired_available, Some(1234567890));
            assert_eq!(resp.arguments.torrents[3].desired_available, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_desired_available_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].desired_available, None);
            Ok(())
        },
    )
}

// ----- done_date (doneDate, DoneDate) --------------------

#[test]
fn test_torrent_get_done_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "doneDate":0 },
                    { "doneDate":-1 },
                    { "doneDate":1672060369 },
                    { "done_date":1672060369 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].done_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[1].done_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].done_date,
                Some(DateTime::parse_from_rfc3339("2022-12-26T13:12:49Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[3].done_date,
                Some(DateTime::parse_from_rfc3339("2022-12-26T13:12:49Z")?.to_utc()),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_done_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].done_date, None);
            Ok(())
        },
    )
}

// ----- download_dir (downloadDir, DownloadDir) --------------------

#[test]
fn test_torrent_get_download_dir_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadDir":"/downloads/iso/" },
                    { "download_dir":"/lorem/ipsum/" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].download_dir, Some("/downloads/iso/".into()));
            assert_eq!(resp.arguments.torrents[1].download_dir, Some("/lorem/ipsum/".into()));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_download_dir_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].download_dir, None);
            Ok(())
        },
    )
}

// ----- downloaded_ever (downloadedEver, DownloadedEver) --------------------

#[test]
fn test_torrent_get_downloaded_ever_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadedEver":0 },
                    { "downloadedEver":1340189370 },
                    { "downloaded_ever":0 },
                    { "downloaded_ever":1234567890 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].downloaded_ever, Some(0));
            assert_eq!(resp.arguments.torrents[1].downloaded_ever, Some(1340189370));
            assert_eq!(resp.arguments.torrents[2].downloaded_ever, Some(0));
            assert_eq!(resp.arguments.torrents[3].downloaded_ever, Some(1234567890));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_downloaded_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].downloaded_ever, None);
            Ok(())
        },
    )
}

// ----- download_limit (downloadLimit, DownloadLimit) --------------------

#[test]
fn test_torrent_get_download_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadLimit":0 },
                    { "downloadLimit":1024 },
                    { "downloadLimit":0 },
                    { "downloadLimit":4096 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].download_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].download_limit, Some(1024));
            assert_eq!(resp.arguments.torrents[2].download_limit, Some(0));
            assert_eq!(resp.arguments.torrents[3].download_limit, Some(4096));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_download_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].download_limit, None);
            Ok(())
        },
    )
}

// ----- download_limited (downloadLimited, DownloadLimited) --------------------

#[test]
fn test_torrent_get_download_limited_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadLimited":true },
                    { "downloadLimited":false },
                    { "download_limited":true },
                    { "download_limited":false }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].download_limited, Some(true));
            assert_eq!(resp.arguments.torrents[1].download_limited, Some(false));
            assert_eq!(resp.arguments.torrents[2].download_limited, Some(true));
            assert_eq!(resp.arguments.torrents[3].download_limited, Some(false));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_download_limited_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].download_limited, None);
            Ok(())
        },
    )
}

// ----- edit_date (editDate, DownloadLimited) --------------------

#[test]
fn test_torrent_get_edit_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "editDate":0 },
                    { "editDate":-1 },
                    { "editDate":1723512675 },
                    { "edit_date":1723512675 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].edit_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[1].edit_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].edit_date,
                Some(DateTime::parse_from_rfc3339("2024-08-13T01:31:15Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[3].edit_date,
                Some(DateTime::parse_from_rfc3339("2024-08-13T01:31:15Z")?.to_utc()),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_edit_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].edit_date, None);
            Ok(())
        },
    )
}

// ----- error (Error) --------------------

#[test]
fn test_torrent_get_error_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "error":0 },
                    { "error":1 },
                    { "error":2 },
                    { "error":3 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].error, Some(ErrorType::Ok));
            assert_eq!(
                resp.arguments.torrents[1].error,
                Some(ErrorType::TrackerWarning)
            );
            assert_eq!(
                resp.arguments.torrents[2].error,
                Some(ErrorType::TrackerError)
            );
            assert_eq!(
                resp.arguments.torrents[3].error,
                Some(ErrorType::LocalError)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_error_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].error, None);
            Ok(())
        },
    )
}

// ----- error_string (errorString, ErrorString) --------------------

#[test]
fn test_torrent_get_error_string_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "errorString":"" },
                    { "errorString":"Unregistered torrent" },
                    { "error_string":"Unregistered torrent" },
                    { "errorString":"No data found! Ensure your drives are connected or use \"Set Location\". To re-download, remove the torrent and re-add it." }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].error_string, Some("".into()));
            assert_eq!(
                resp.arguments.torrents[1].error_string,
                Some("Unregistered torrent".into())
            );
            assert_eq!(
                resp.arguments.torrents[2].error_string,
                Some("Unregistered torrent".into())
            );
            assert_eq!(
                resp.arguments.torrents[3].error_string,
                Some(
                    "No data found! Ensure your drives are connected or use \"Set Location\". \
                To re-download, remove the torrent and re-add it."
                .into()
                ),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_error_string_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].error_string, None);
            Ok(())
        },
    )
}

// ----- eta (Eta) --------------------

#[test]
fn test_torrent_get_eta_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "eta":-1 },
                    { "eta":82112 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].eta, Some(-1));
            assert_eq!(resp.arguments.torrents[1].eta, Some(82112));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_eta_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].eta, None);
            Ok(())
        },
    )
}

// ----- eta_idle (etaIdle, EtaIdle) --------------------

#[test]
fn test_torrent_get_eta_idle_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "etaIdle":-1 },
                    { "etaIdle":1234 },
                    { "eta_idle":0 },
                    { "eta_idle":4231 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].eta_idle, Some(-1));
            assert_eq!(resp.arguments.torrents[1].eta_idle, Some(1234));
            assert_eq!(resp.arguments.torrents[2].eta_idle, Some(0));
            assert_eq!(resp.arguments.torrents[3].eta_idle, Some(4231));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_eta_idle_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].eta_idle, None);
            Ok(())
        },
    )
}

// ----- file_count (file-count, FileCount) --------------------

#[test]
fn test_torrent_get_file_count_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "file-count":0 },
                    { "file-count":31 },
                    { "file_count":0 },
                    { "file_count":420 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].file_count, Some(0)); // Probably impossible
            assert_eq!(resp.arguments.torrents[1].file_count, Some(31));
            assert_eq!(resp.arguments.torrents[2].file_count, Some(0)); // Probably impossible
            assert_eq!(resp.arguments.torrents[3].file_count, Some(420));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_file_count_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].file_count, None);
            Ok(())
        },
    )
}

// ----- files (Files) --------------------

/// Pre- rpc-version `18` (transmission `4.1.0`) test where the `File`s will contain neither
/// `begin_piece` nor `end_piece`.
#[test]
fn test_torrent_get_files_success_pre_rpc_ver_18() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "files":[{
                        "bytesCompleted":172415250,
                        "length":3994091520,
                        "name":"debian-12.6.0-amd64-DVD-1.iso"
                    }] },
                    { "files":[
                        {
                            "bytesCompleted":0,
                            "length":1229,
                            "name":"Fedora-Server-40-1.14-x86_64-CHECKSUM"
                        },
                        {
                            "bytesCompleted":0,
                            "length":2612854784,
                            "name":"Fedora-Server-dvd-x86_64-40-1.14.iso"
                        }
                    ] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let first = resp.arguments.torrents[0]
                .files
                .as_ref()
                .expect("files should exist");
            assert_eq!(first.len(), 1);
            assert_eq!(first[0].length, 3994091520);
            assert_eq!(first[0].bytes_completed, 172415250);
            assert_eq!(first[0].name, "debian-12.6.0-amd64-DVD-1.iso");
            assert_eq!(first[0].begin_piece, None);
            assert_eq!(first[0].end_piece, None);
            let second = resp.arguments.torrents[1]
                .files
                .as_ref()
                .expect("files should exist");
            assert_eq!(second.len(), 2);
            assert_eq!(second[0].length, 1229);
            assert_eq!(second[0].bytes_completed, 0);
            assert_eq!(second[0].name, "Fedora-Server-40-1.14-x86_64-CHECKSUM");
            assert_eq!(second[0].begin_piece, None);
            assert_eq!(second[0].end_piece, None);
            assert_eq!(second[1].length, 2612854784);
            assert_eq!(second[1].bytes_completed, 0);
            assert_eq!(second[1].name, "Fedora-Server-dvd-x86_64-40-1.14.iso");
            assert_eq!(second[1].begin_piece, None);
            assert_eq!(second[1].end_piece, None);
            Ok(())
        },
    )
}

/// Post- rpc-version `18` (transmission `4.1.0`) test where the `File`s **will** contain both
/// `begin_piece` and `end_piece`.
#[test]
fn test_torrent_get_files_success_post_rpc_ver_18() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "files":[
                        {
                            "bytesCompleted":0,
                            "length":1229,
                            "name":"Fedora-Server-40-1.14-x86_64-CHECKSUM",
                            "beginPiece": 0,
                            "endPiece": 123456
                        },
                        {
                            "bytesCompleted":0,
                            "length":2612854784,
                            "name":"Fedora-Server-dvd-x86_64-40-1.14.iso",
                            "beginPiece": 123456,
                            "endPiece": 234567
                        }
                    ] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            let files = resp.arguments.torrents[0]
                .files
                .as_ref()
                .expect("files should exist");
            assert_eq!(files.len(), 2);
            assert_eq!(files[0].length, 1229);
            assert_eq!(files[0].bytes_completed, 0);
            assert_eq!(files[0].name, "Fedora-Server-40-1.14-x86_64-CHECKSUM");
            assert_eq!(files[0].begin_piece, Some(0));
            assert_eq!(files[0].end_piece, Some(123456));

            assert_eq!(files[1].length, 2612854784);
            assert_eq!(files[1].bytes_completed, 0);
            assert_eq!(files[1].name, "Fedora-Server-dvd-x86_64-40-1.14.iso");
            assert_eq!(files[1].begin_piece, Some(123456));
            assert_eq!(files[1].end_piece, Some(234567));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_files_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].files.is_none());
            Ok(())
        },
    )
}

// ----- file_stats (fileStats, FileStats) --------------------

#[test]
fn test_torrent_get_file_stats_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { 
                        "fileStats":[
                            {
                                "bytesCompleted": 2972771,
                                "priority": 0,
                                "wanted": true
                            },
                            {
                                "bytesCompleted": 17662350,
                                "priority": 1,
                                "wanted": true
                            },
                            {
                                "bytesCompleted": 0,
                                "priority": -1,
                                "wanted": false
                            }
                        ]
                    },
                    { 
                        "file_stats":[
                            {
                                "bytesCompleted": 0,
                                "priority": 1,
                                "wanted": false
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let first = resp.arguments.torrents[0]
                .file_stats
                .as_ref()
                .expect("file_stats should exist");
            assert_eq!(first.len(), 3);
            assert_eq!(first[0].bytes_completed, 2972771);
            assert_eq!(first[0].priority, Priority::Normal);
            assert_eq!(first[0].wanted, true);
            assert_eq!(first[1].bytes_completed, 17662350);
            assert_eq!(first[1].priority, Priority::High);
            assert_eq!(first[1].wanted, true);
            assert_eq!(first[2].bytes_completed, 0);
            assert_eq!(first[2].priority, Priority::Low);
            assert_eq!(first[2].wanted, false);
            let second = resp.arguments.torrents[1]
                .file_stats
                .as_ref()
                .expect("file_stats should exist");
            assert_eq!(second[0].bytes_completed, 0);
            assert_eq!(second[0].priority, Priority::High);
            assert_eq!(second[0].wanted, false);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_file_stats_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].file_stats.is_none());
            Ok(())
        },
    )
}

// ----- group (Group) --------------------

#[test]
fn test_torrent_get_group_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "group":"foo" } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].group, Some("foo".into()));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_group_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].group, None);
            Ok(())
        },
    )
}

// ----- hash_string (hashString, HashString) --------------------

#[test]
fn test_torrent_get_hash_string_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "hashString":"7fce8abbdacefd47321700ff95106447009aa1e7" },
                    { "hash_string":"3a3e1717f61a0b85777a7e8371425d6588b2a000" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].hash_string,
                Some("7fce8abbdacefd47321700ff95106447009aa1e7".into()),
            );
            assert_eq!(
                resp.arguments.torrents[1].hash_string,
                Some("3a3e1717f61a0b85777a7e8371425d6588b2a000".into()),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_hash_string_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].hash_string, None);
            Ok(())
        },
    )
}

// ----- have_unchecked (haveUnchecked, HaveUnchecked) --------------------

#[test]
fn test_torrent_get_have_unchecked_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "haveUnchecked":39813 },
                    { "haveUnchecked":0 },
                    { "have_unchecked":43245 },
                    { "have_unchecked":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].have_unchecked, Some(39813));
            assert_eq!(resp.arguments.torrents[1].have_unchecked, Some(0));
            assert_eq!(resp.arguments.torrents[2].have_unchecked, Some(43245));
            assert_eq!(resp.arguments.torrents[3].have_unchecked, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_have_unchecked_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].have_unchecked, None);
            Ok(())
        },
    )
}

// ----- have_valid (haveValid, HaveValid) --------------------

#[test]
fn test_torrent_get_have_valid_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "haveValid":1276581443 },
                    { "haveValid":0 },
                    { "have_valid":456231 },
                    { "have_valid":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].have_valid, Some(1276581443));
            assert_eq!(resp.arguments.torrents[1].have_valid, Some(0));
            assert_eq!(resp.arguments.torrents[2].have_valid, Some(456231));
            assert_eq!(resp.arguments.torrents[3].have_valid, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_have_valid_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].have_valid, None);
            Ok(())
        },
    )
}

// ----- honors_session_limits (honorsSessionLimits, HonorsSessionLimits) --------------------

#[test]
fn test_torrent_get_honors_session_limits_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "honorsSessionLimits":false },
                    { "honorsSessionLimits":true },
                    { "honors_session_limits":false },
                    { "honors_session_limits":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].honors_session_limits, Some(false));
            assert_eq!(resp.arguments.torrents[1].honors_session_limits, Some(true));
            assert_eq!(resp.arguments.torrents[2].honors_session_limits, Some(false));
            assert_eq!(resp.arguments.torrents[3].honors_session_limits, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_honors_session_limits_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].honors_session_limits, None);
            Ok(())
        },
    )
}

// ----- id (Id) --------------------

#[test]
fn test_torrent_get_id_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "id":111 } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].id, Some(111));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_id_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_hash())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].id, None);
            Ok(())
        },
    )
}

// ----- is_finished (isFinished, IsFinished) --------------------

#[test]
fn test_torrent_get_is_finished_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "isFinished":true },
                    { "isFinished":false },
                    { "is_finished":true },
                    { "is_finished":false }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].is_finished, Some(true));
            assert_eq!(resp.arguments.torrents[1].is_finished, Some(false));
            assert_eq!(resp.arguments.torrents[2].is_finished, Some(true));
            assert_eq!(resp.arguments.torrents[3].is_finished, Some(false));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_is_finished_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].is_finished, None);
            Ok(())
        },
    )
}

// ----- is_private (isPrivate, IsPrivate) --------------------

#[test]
fn test_torrent_get_is_private_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "isPrivate":false },
                    { "isPrivate":true },
                    { "is_private":false },
                    { "is_private":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].is_private, Some(false));
            assert_eq!(resp.arguments.torrents[1].is_private, Some(true));
            assert_eq!(resp.arguments.torrents[2].is_private, Some(false));
            assert_eq!(resp.arguments.torrents[3].is_private, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_is_private_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].is_private, None);
            Ok(())
        },
    )
}

// ----- is_stalled (isStalled, IsStalled) --------------------

#[test]
fn test_torrent_get_is_stalled_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "isStalled":false },
                    { "isStalled":true },
                    { "is_stalled":false },
                    { "is_stalled":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].is_stalled, Some(false));
            assert_eq!(resp.arguments.torrents[1].is_stalled, Some(true));
            assert_eq!(resp.arguments.torrents[2].is_stalled, Some(false));
            assert_eq!(resp.arguments.torrents[3].is_stalled, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_is_stalled_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].is_stalled, None);
            Ok(())
        },
    )
}

// ----- labels (Labels) --------------------

#[test]
fn test_torrent_get_labels_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "labels":[] },
                    { "labels":["foo"] },
                    { "labels":["bar","baz"] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].labels, Some(vec![]));
            assert_eq!(resp.arguments.torrents[1].labels, Some(vec!["foo".into()]));
            assert_eq!(
                resp.arguments.torrents[2].labels,
                Some(vec!["bar".into(), "baz".into()])
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_labels_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].labels, None);
            Ok(())
        },
    )
}

// ----- left_until_done (leftUntilDone, LeftUntilDone) --------------------

#[test]
fn test_torrent_get_left_until_done_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "leftUntilDone":2138956824 },
                    { "leftUntilDone":0 },
                    { "left_until_done":123 },
                    { "left_until_done":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].left_until_done, Some(2138956824));
            assert_eq!(resp.arguments.torrents[1].left_until_done, Some(0));
            assert_eq!(resp.arguments.torrents[2].left_until_done, Some(123));
            assert_eq!(resp.arguments.torrents[3].left_until_done, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_left_until_done_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].left_until_done, None);
            Ok(())
        },
    )
}

// ----- magnet_link (magnetLink, MagnetLink) --------------------

#[test]
fn test_torrent_get_magnet_link_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "magnetLink":"" },
                    { "magnetLink":"magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&dn=archlinux-2024.08.01-x86_64.iso" },
                    { "magnet_link":"magnet:?xt=urn:btih:eb409198dba6f08a2d8817e71f686fb47fbf32f6&dn=EndeavourOS_Titan-2026.03.06.iso&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Fthetracker.org%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.dutchtracking.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].magnet_link, Some("".into()));
            assert_eq!(
                resp.arguments.torrents[1].magnet_link,
                Some(
                    "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810\
                &dn=archlinux-2024.08.01-x86_64.iso"
                .into()
                ),
            );
            assert_eq!(
                resp.arguments.torrents[2].magnet_link,
                Some(
                    "magnet:?xt=urn:btih:eb409198dba6f08a2d8817e71f686fb47fbf32f6\
                    &dn=EndeavourOS_Titan-2026.03.06.iso\
                    &tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80\
                    &tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce\
                    &tr=udp%3A%2F%2Fthetracker.org%3A80%2Fannounce\
                    &tr=udp%3A%2F%2Ftracker.dutchtracking.com%3A6969%2Fannounce\
                    &tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce"
                .into()
                ),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_magnet_link_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].magnet_link, None);
            Ok(())
        },
    )
}

// ----- manual_announce_time (manualAnnounceTime, ManualAnnounceTime) --------------------

#[test]
fn test_torrent_get_manual_announce_time_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "manualAnnounceTime":-1 },
                    { "manualAnnounceTime":0 },
                    { "manualAnnounceTime":1723512975 },
                    { "manual_announce_time":946684800 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].manual_announce_time,
                Some(DateTime::UNIX_EPOCH),
            );
            assert_eq!(
                resp.arguments.torrents[1].manual_announce_time,
                Some(DateTime::UNIX_EPOCH),
            ); // Might be impossible
            assert_eq!(
                resp.arguments.torrents[2].manual_announce_time,
                Some(DateTime::parse_from_rfc3339("2024-08-13T01:36:15Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[3].manual_announce_time,
                Some(DateTime::parse_from_rfc3339("2000-01-01 00:00:00Z")?.to_utc()),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_manual_announce_time_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].manual_announce_time, None);
            Ok(())
        },
    )
}

// ----- max_connected_peers (maxConnectedPeers, MaxConnectedPeers) --------------------

#[test]
fn test_torrent_get_max_connected_peers_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "maxConnectedPeers":0 },
                    { "maxConnectedPeers":20 },
                    { "max_connected_peers":0 },
                    { "max_connected_peers":100 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].max_connected_peers, Some(0));
            assert_eq!(resp.arguments.torrents[1].max_connected_peers, Some(20));
            assert_eq!(resp.arguments.torrents[2].max_connected_peers, Some(0));
            assert_eq!(resp.arguments.torrents[3].max_connected_peers, Some(100));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_max_connected_peers_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].max_connected_peers, None);
            Ok(())
        },
    )
}

// ----- metadata_percent_complete (metadataPercentComplete, MetadataPercentComplete) ----------

#[test]
fn test_torrent_get_metadata_percent_complete_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "metadataPercentComplete":1 },
                    { "metadataPercentComplete":0 },
                    { "metadataPercentComplete":0.5284 },
                    { "metadata_percent_complete":0.333 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].metadata_percent_complete,
                Some(1.)
            );
            assert_eq!(
                resp.arguments.torrents[1].metadata_percent_complete,
                Some(0.)
            );
            assert_eq!(
                resp.arguments.torrents[2].metadata_percent_complete,
                Some(0.5284)
            );
            assert_eq!(
                resp.arguments.torrents[3].metadata_percent_complete,
                Some(0.333)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_metadata_percent_complete_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].metadata_percent_complete, None);
            Ok(())
        },
    )
}

// ----- name (Name) --------------------

#[test]
fn test_torrent_get_name_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "name":"debian-12.6.0-amd64-DVD-1.iso" } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].name,
                Some("debian-12.6.0-amd64-DVD-1.iso".into())
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_name_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].name, None);
            Ok(())
        },
    )
}

// ----- peer_limit (peer-limit, PeerLimit) --------------------

#[test]
fn test_torrent_get_peer_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peer-limit":0 },
                    { "peer-limit":20 },
                    { "peer_limit":0 },
                    { "peer_limit":50 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peer_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].peer_limit, Some(20));
            assert_eq!(resp.arguments.torrents[2].peer_limit, Some(0));
            assert_eq!(resp.arguments.torrents[3].peer_limit, Some(50));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peer_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peer_limit, None);
            Ok(())
        },
    )
}

// ----- peers (Peer) --------------------

#[test]
fn test_torrent_get_peers_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peers":[] },
                    {
                        "peers":[
                            {
                                "address":"10.0.0.100",
                                "clientIsChoked":false,
                                "clientIsInterested":true,
                                "clientName":"\u00b5Torrent 3.5.5",
                                "flagStr":"dUEI",
                                "isDownloadingFrom":false,
                                "isEncrypted":true,
                                "isIncoming":true,
                                "isUploadingTo":true,
                                "isUTP":false,
                                "peerIsChoked":false,
                                "peerIsInterested":true,
                                "port":55555,
                                "progress":0.2641,
                                "rateToClient":0,
                                "rateToPeer":385000
                            },
                            {
                                "address":"2001:0db8:85a3:0000:0000:8a2e:0370:7334",
                                "bytes_to_client":12345,
                                "bytes_to_peer":6,
                                "client_is_choked":false,
                                "client_is_interested":true,
                                "client_name":"qBittorrent 4.6.5",
                                "flag_str":"TDI",
                                "is_downloading_from":true,
                                "is_encrypted":false,
                                "is_incoming":true,
                                "is_uploading_to":false,
                                "is_utp":true,
                                "peer_id":"IoxddjPhaC1vb0toOUdUTGhvbzc=",
                                "peer_is_choked":true,
                                "peer_is_interested":false,
                                "port":36667,
                                "progress":1,
                                "rate_to_client":8000,
                                "rate_to_peer":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let first = resp.arguments.torrents[0]
                .peers
                .as_ref()
                .expect("peers should exist");
            assert_eq!(first.len(), 0);

            let second = resp.arguments.torrents[1]
                .peers
                .as_ref()
                .expect("peers should exist");
            assert_eq!(second.len(), 2);
            assert_eq!(second[0].address, IpAddr::V4(Ipv4Addr::new(10, 0, 0, 100)));
            assert_eq!(second[0].bytes_to_client, 0);
            assert_eq!(second[0].bytes_to_peer, 0);
            assert_eq!(second[0].client_name, "µTorrent 3.5.5".to_string());
            assert_eq!(second[0].client_is_choked, false);
            assert_eq!(second[0].client_is_interested, true);
            assert_eq!(second[0].flag_str, "dUEI".to_string());
            assert_eq!(second[0].is_downloading_from, false);
            assert_eq!(second[0].is_encrypted, true);
            assert_eq!(second[0].is_incoming, true);
            assert_eq!(second[0].is_uploading_to, true);
            assert_eq!(second[0].is_utp, false);
            assert_eq!(second[0].peer_id, "".to_string());
            assert_eq!(second[0].peer_is_choked, false);
            assert_eq!(second[0].peer_is_interested, true);
            assert_eq!(second[0].port, 55555);
            assert_eq!(second[0].progress, 0.2641);
            assert_eq!(second[0].rate_to_client, 0);
            assert_eq!(second[0].rate_to_peer, 385000);

            assert_eq!(
                second[1].address,
                IpAddr::V6(Ipv6Addr::new(8193, 3512, 34211, 0, 0, 35374, 880, 29492))
            );
            assert_eq!(second[1].bytes_to_client, 12345);
            assert_eq!(second[1].bytes_to_peer, 6);
            assert_eq!(second[1].client_name, "qBittorrent 4.6.5".to_string());
            assert_eq!(second[1].client_is_choked, false);
            assert_eq!(second[1].client_is_interested, true);
            assert_eq!(second[1].flag_str, "TDI".to_string());
            assert_eq!(second[1].is_downloading_from, true);
            assert_eq!(second[1].is_encrypted, false);
            assert_eq!(second[1].is_incoming, true);
            assert_eq!(second[1].is_uploading_to, false);
            assert_eq!(second[1].is_utp, true);
            assert_eq!(second[1].peer_id, "IoxddjPhaC1vb0toOUdUTGhvbzc=".to_string());
            assert_eq!(second[1].peer_is_choked, true);
            assert_eq!(second[1].peer_is_interested, false);
            assert_eq!(second[1].port, 36667);
            assert_eq!(second[1].progress, 1.);
            assert_eq!(second[1].rate_to_client, 8000);
            assert_eq!(second[1].rate_to_peer, 0);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peers_semver_600_compat() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    {
                        "peers":[
                            {
                                "address":"2001:0db8:85a3:0000:0000:8a2e:0370:7334",
                                "client_is_choked":false,
                                "client_is_interested":true,
                                "client_name":"qBittorrent 4.6.5",
                                "flag_str":"TDI",
                                "is_downloading_from":true,
                                "is_encrypted":false,
                                "is_incoming":true,
                                "is_uploading_to":false,
                                "is_utp":true,
                                "peer_is_choked":true,
                                "peer_is_interested":false,
                                "port":36667,
                                "progress":1,
                                "rate_to_client":8000,
                                "rate_to_peer":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            let first = resp.arguments.torrents[0]
                .peers
                .as_ref()
                .expect("peers should exist");
            assert_eq!(first.len(), 1);

            // Verify the Torrent deserialized okay even if the response json is missing fields
            // from semver-6.0.0 and later.
            assert_eq!(first[0].bytes_to_client, 0);
            assert_eq!(first[0].bytes_to_peer, 0);
            assert_eq!(first[0].peer_id, "".to_string());

            assert_eq!(
                first[0].address,
                IpAddr::V6(Ipv6Addr::new(8193, 3512, 34211, 0, 0, 35374, 880, 29492))
            );
            assert_eq!(first[0].client_name, "qBittorrent 4.6.5".to_string());
            assert_eq!(first[0].client_is_choked, false);
            assert_eq!(first[0].client_is_interested, true);
            assert_eq!(first[0].flag_str, "TDI".to_string());
            assert_eq!(first[0].is_downloading_from, true);
            assert_eq!(first[0].is_encrypted, false);
            assert_eq!(first[0].is_incoming, true);
            assert_eq!(first[0].is_uploading_to, false);
            assert_eq!(first[0].is_utp, true);
            assert_eq!(first[0].peer_is_choked, true);
            assert_eq!(first[0].peer_is_interested, false);
            assert_eq!(first[0].port, 36667);
            assert_eq!(first[0].progress, 1.);
            assert_eq!(first[0].rate_to_client, 8000);
            assert_eq!(first[0].rate_to_peer, 0);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peers_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].peers.is_none());
            Ok(())
        },
    )
}

// ----- peers_connected (peersConnected, PeersConnected) --------------------

#[test]
fn test_torrent_get_peers_connected_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peersConnected":0 },
                    { "peersConnected":6 },
                    { "peers_connected":10 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peers_connected, Some(0));
            assert_eq!(resp.arguments.torrents[1].peers_connected, Some(6));
            assert_eq!(resp.arguments.torrents[2].peers_connected, Some(10));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peers_connected_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peers_connected, None);
            Ok(())
        },
    )
}

// ----- peers_from (peersFrom, PeersFrom) --------------------

#[test]
fn test_torrent_get_peers_from_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { 
                        "peersFrom": {
                            "fromCache":0,
                            "fromDht":1,
                            "fromIncoming":2,
                            "fromLpd":3,
                            "fromLtep":4,
                            "fromPex":5,
                            "fromTracker":6
                        }
                    },
                    { 
                        "peers_from": {
                            "from_cache":10,
                            "from_dht":11,
                            "from_incoming":12,
                            "from_lpd":13,
                            "from_ltep":14,
                            "from_pex":15,
                            "from_tracker":16
                        }
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let first = resp.arguments.torrents[0]
                .peers_from
                .as_ref()
                .expect("peers_from should exist");
            assert_eq!(first.from_cache, 0);
            assert_eq!(first.from_dht, 1);
            assert_eq!(first.from_incoming, 2);
            assert_eq!(first.from_lpd, 3);
            assert_eq!(first.from_ltep, 4);
            assert_eq!(first.from_pex, 5);
            assert_eq!(first.from_tracker, 6);
            let second = resp.arguments.torrents[1]
                .peers_from
                .as_ref()
                .expect("peers_from should exist");
            assert_eq!(second.from_cache, 10);
            assert_eq!(second.from_dht, 11);
            assert_eq!(second.from_incoming, 12);
            assert_eq!(second.from_lpd, 13);
            assert_eq!(second.from_ltep, 14);
            assert_eq!(second.from_pex, 15);
            assert_eq!(second.from_tracker, 16);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peers_from_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].peers_from.is_none());
            Ok(())
        },
    )
}

// ----- peers_getting_from_us (peersGettingFromUs, PeersGettingFromUs) --------------------

#[test]
fn test_torrent_get_peers_getting_from_us_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peersGettingFromUs":0 },
                    { "peersGettingFromUs":2 },
                    { "peers_getting_from_us":0 },
                    { "peers_getting_from_us":24 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peers_getting_from_us, Some(0));
            assert_eq!(resp.arguments.torrents[1].peers_getting_from_us, Some(2));
            assert_eq!(resp.arguments.torrents[2].peers_getting_from_us, Some(0));
            assert_eq!(resp.arguments.torrents[3].peers_getting_from_us, Some(24));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peers_getting_from_us_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peers_getting_from_us, None);
            Ok(())
        },
    )
}

// ----- peers_sending_to_us (peersSendingToUs, PeersSendingToUs) --------------------

#[test]
fn test_torrent_get_peers_sending_to_us_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peersSendingToUs":0 },
                    { "peersSendingToUs":9 },
                    { "peers_sending_to_us":0 },
                    { "peers_sending_to_us":91 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peers_sending_to_us, Some(0));
            assert_eq!(resp.arguments.torrents[1].peers_sending_to_us, Some(9));
            assert_eq!(resp.arguments.torrents[2].peers_sending_to_us, Some(0));
            assert_eq!(resp.arguments.torrents[3].peers_sending_to_us, Some(91));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_peers_sending_to_us_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].peers_sending_to_us, None);
            Ok(())
        },
    )
}

// ----- percent_complete (percentComplete, PercentComplete) --------------------

#[test]
fn test_torrent_get_percent_complete_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "percentComplete":1 },
                    { "percentComplete":0 },
                    { "percentComplete":0.321 },
                    { "percent_complete":1 },
                    { "percent_complete":0 },
                    { "percent_complete":0.567 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].percent_complete, Some(1.));
            assert_eq!(resp.arguments.torrents[1].percent_complete, Some(0.));
            assert_eq!(resp.arguments.torrents[2].percent_complete, Some(0.321));
            assert_eq!(resp.arguments.torrents[3].percent_complete, Some(1.));
            assert_eq!(resp.arguments.torrents[4].percent_complete, Some(0.));
            assert_eq!(resp.arguments.torrents[5].percent_complete, Some(0.567));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_percent_complete_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].percent_complete, None);
            Ok(())
        },
    )
}

// ----- percent_done (percentDone, PercentDone) --------------------

#[test]
fn test_torrent_get_percent_done_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "percentDone":0 },
                    { "percentDone":1 },
                    { "percentDone":0.4231 },
                    { "percent_done":0 },
                    { "percent_done":1 },
                    { "percent_done":0.9876 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].percent_done, Some(0.));
            assert_eq!(resp.arguments.torrents[1].percent_done, Some(1.));
            assert_eq!(resp.arguments.torrents[2].percent_done, Some(0.4231));
            assert_eq!(resp.arguments.torrents[3].percent_done, Some(0.));
            assert_eq!(resp.arguments.torrents[4].percent_done, Some(1.));
            assert_eq!(resp.arguments.torrents[5].percent_done, Some(0.9876));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_percent_done_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].percent_done, None);
            Ok(())
        },
    )
}

// ----- pieces (Pieces) --------------------

#[test]
fn test_torrent_get_pieces_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 

                    { "pieces":"/Pb49/m+8tPzi+Z/e/39" },

                    { "pieces":"//////////////////////////////////////////////////////////////////////////////////////////////////////////////////w=" },

                    { "pieces":"AAAAAAAAAAAA" }

                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        |resp| {
            let first = resp.arguments.torrents[0]
                .pieces
                .as_ref()
                .expect("pieces should exist");
            assert_eq!(first.len(), 15); // 120 pieces (8 * 15 = 120)
    let bitfield: Vec<u8> = vec![
        0xFC, 0xF6, 0xF8, 0xF7, 0xF9, 0xBE, 0xF2, 0xD3, 0xF3, 0x8B, 0xE6, 0x7F, 0x7B, 0xFD,
        0xFD,
    ];
    assert_eq!(first, &bitfield);

    let second = resp.arguments.torrents[1]
        .pieces
        .as_ref()
        .expect("pieces should exist");
            assert_eq!(second.len(), 86); // 686 pieces (8 * 86 = 688 => 2 extra bits)
    let mut bitfield = vec![u8::MAX; 85];
    bitfield.push(0xFC);
    assert_eq!(second, &bitfield);

    let third = resp.arguments.torrents[2]
        .pieces
        .as_ref()
        .expect("pieces should exist");
            assert_eq!(third.len(), 9); // 72 pieces (8 * 9 = 72)
            let bitfield = vec![0u8; 9];
            assert_eq!(third, &bitfield);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_pieces_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].pieces.is_none());
            Ok(())
        },
    )
}

// ----- piece_count (pieceCount, PieceCount) --------------------

#[test]
fn test_torrent_get_piece_count_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "pieceCount":10234 },
                    { "pieceCount":9876 },
                    { "piece_count":123 },
                    { "piece_count":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].piece_count, Some(10234));
            assert_eq!(resp.arguments.torrents[1].piece_count, Some(9876));
            assert_eq!(resp.arguments.torrents[2].piece_count, Some(123));
            assert_eq!(resp.arguments.torrents[3].piece_count, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_piece_count_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].piece_count, None);
            Ok(())
        },
    )
}

// ----- piece_size (pieceSize, PieceSize) --------------------

#[test]
fn test_torrent_get_piece_size_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "pieceSize":2097152 },
                    { "pieceSize":1048576 },
                    { "piece_size":4096 },
                    { "piece_size":65536 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].piece_size, Some(2097152));
            assert_eq!(resp.arguments.torrents[1].piece_size, Some(1048576));
            assert_eq!(resp.arguments.torrents[2].piece_size, Some(4096));
            assert_eq!(resp.arguments.torrents[3].piece_size, Some(65536));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_piece_size_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].piece_size, None);
            Ok(())
        },
    )
}

// ----- primary_mime_type (primary-mime-type, PrimaryMimeType) --------------------

#[test]
fn test_torrent_get_primary_mime_type_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "primary-mime-type":"application/octet-stream" },
                    { "primary-mime-type":"audio/x-flac" },
                    { "primary_mime_type":"image/apng" },
                    { "primary_mime_type":"image/webp" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].primary_mime_type,
                Some("application/octet-stream".into()),
            );
            assert_eq!(
                resp.arguments.torrents[1].primary_mime_type,
                Some("audio/x-flac".into())
            );
            assert_eq!(
                resp.arguments.torrents[2].primary_mime_type,
                Some("image/apng".into()),
            );
            assert_eq!(
                resp.arguments.torrents[3].primary_mime_type,
                Some("image/webp".into())
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_primary_mime_type_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].primary_mime_type, None);
            Ok(())
        },
    )
}

// ----- queue_position (queuePosition, QueuePosition) --------------------

#[test]
fn test_torrent_get_queue_position_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "queuePosition":0 },
                    { "queuePosition":1 },
                    { "queuePosition":342 },
                    { "queue_position":0 },
                    { "queue_position":1 },
                    { "queue_position":784 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].queue_position, Some(0));
            assert_eq!(resp.arguments.torrents[1].queue_position, Some(1));
            assert_eq!(resp.arguments.torrents[2].queue_position, Some(342));
            assert_eq!(resp.arguments.torrents[3].queue_position, Some(0));
            assert_eq!(resp.arguments.torrents[4].queue_position, Some(1));
            assert_eq!(resp.arguments.torrents[5].queue_position, Some(784));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_queue_position_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].queue_position, None);
            Ok(())
        },
    )
}

// ----- rate_download (rateDownload, RateDownload) --------------------

#[test]
fn test_torrent_get_rate_download_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "rateDownload":93000 },
                    { "rateDownload":0 },
                    { "rate_download":100 },
                    { "rate_download":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].rate_download, Some(93000));
            assert_eq!(resp.arguments.torrents[1].rate_download, Some(0));
            assert_eq!(resp.arguments.torrents[2].rate_download, Some(100));
            assert_eq!(resp.arguments.torrents[3].rate_download, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_rate_download_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].rate_download, None);
            Ok(())
        },
    )
}

// ----- rate_upload (rateUpload, RateUpload) --------------------

#[test]
fn test_torrent_get_rate_upload_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "rateUpload":0 },
                    { "rateUpload":150000 },
                    { "rate_upload":0 },
                    { "rate_upload":300000 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].rate_upload, Some(0));
            assert_eq!(resp.arguments.torrents[1].rate_upload, Some(150000));
            assert_eq!(resp.arguments.torrents[2].rate_upload, Some(0));
            assert_eq!(resp.arguments.torrents[3].rate_upload, Some(300000));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_rate_upload_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].rate_upload, None);
            Ok(())
        },
    )
}

// ----- recheck_progress (recheckProgress, RecheckProgress) --------------------

#[test]
fn test_torrent_get_recheck_progress_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "recheckProgress":0 },
                    { "recheckProgress":1 },
                    { "recheckProgress":0.4051 },
                    { "recheck_progress":0 },
                    { "recheck_progress":1 },
                    { "recheck_progress":0.6183 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].recheck_progress, Some(0.));
            assert_eq!(resp.arguments.torrents[1].recheck_progress, Some(1.));
            assert_eq!(resp.arguments.torrents[2].recheck_progress, Some(0.4051));
            assert_eq!(resp.arguments.torrents[3].recheck_progress, Some(0.));
            assert_eq!(resp.arguments.torrents[4].recheck_progress, Some(1.));
            assert_eq!(resp.arguments.torrents[5].recheck_progress, Some(0.6183));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_recheck_progress_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].recheck_progress, None);
            Ok(())
        },
    )
}

// ----- seconds_downloading (secondsDownloading, SecondsDownloading) --------------------

#[test]
fn test_torrent_get_seconds_downloading_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "secondsDownloading":0 },
                    { "secondsDownloading":41744 },
                    { "seconds_downloading":0 },
                    { "seconds_downloading":12345 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seconds_downloading, Some(0));
            assert_eq!(resp.arguments.torrents[1].seconds_downloading, Some(41744));
            assert_eq!(resp.arguments.torrents[2].seconds_downloading, Some(0));
            assert_eq!(resp.arguments.torrents[3].seconds_downloading, Some(12345));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_seconds_downloading_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seconds_downloading, None);
            Ok(())
        },
    )
}

// ----- seconds_seeding (secondsSeeding, SecondsSeeding) --------------------

#[test]
fn test_torrent_get_seconds_seeding_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "secondsSeeding":0 },
                    { "secondsSeeding":13359445 },
                    { "seconds_seeding":0 },
                    { "seconds_seeding":98767 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seconds_seeding, Some(0));
            assert_eq!(resp.arguments.torrents[1].seconds_seeding, Some(13359445));
            assert_eq!(resp.arguments.torrents[2].seconds_seeding, Some(0));
            assert_eq!(resp.arguments.torrents[3].seconds_seeding, Some(98767));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_seconds_seeding_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seconds_seeding, None);
            Ok(())
        },
    )
}

// ----- seed_idle_limit (seedIdleLimit, SeedIdleLimit) --------------------

#[test]
fn test_torrent_get_seed_idle_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "seedIdleLimit":0 },
                    { "seedIdleLimit":30 },
                    { "seed_idle_limit":0 },
                    { "seed_idle_limit":10 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seed_idle_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].seed_idle_limit, Some(30));
            assert_eq!(resp.arguments.torrents[2].seed_idle_limit, Some(0));
            assert_eq!(resp.arguments.torrents[3].seed_idle_limit, Some(10));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_seed_idle_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seed_idle_limit, None);
            Ok(())
        },
    )
}

// ----- seed_idle_mode (seedIdleMode, SeedIdleMode) --------------------

#[test]
fn test_torrent_get_seed_idle_mode_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "seedIdleMode":0 },
                    { "seedIdleMode":1 },
                    { "seedIdleMode":2 },
                    { "seed_idle_mode":0 },
                    { "seed_idle_mode":1 },
                    { "seed_idle_mode":2 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].seed_idle_mode,
                Some(IdleMode::Global)
            );
            assert_eq!(
                resp.arguments.torrents[1].seed_idle_mode,
                Some(IdleMode::Single)
            );
            assert_eq!(
                resp.arguments.torrents[2].seed_idle_mode,
                Some(IdleMode::Unlimited)
            );
            assert_eq!(
                resp.arguments.torrents[3].seed_idle_mode,
                Some(IdleMode::Global)
            );
            assert_eq!(
                resp.arguments.torrents[4].seed_idle_mode,
                Some(IdleMode::Single)
            );
            assert_eq!(
                resp.arguments.torrents[5].seed_idle_mode,
                Some(IdleMode::Unlimited)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_seed_idle_mode_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seed_idle_mode, None);
            Ok(())
        },
    )
}

// ----- seed_ratio_limit (seedRatioLimit, SeedRatioLimit) --------------------

#[test]
fn test_torrent_get_seed_ratio_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "seedRatioLimit":0 },
                    { "seedRatioLimit":0.25 },
                    { "seedRatioLimit":15 },
                    { "seed_ratio_limit":0 },
                    { "seed_ratio_limit":0.75 },
                    { "seed_ratio_limit":3 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seed_ratio_limit, Some(0.));
            assert_eq!(resp.arguments.torrents[1].seed_ratio_limit, Some(0.25));
            assert_eq!(resp.arguments.torrents[2].seed_ratio_limit, Some(15.));
            assert_eq!(resp.arguments.torrents[3].seed_ratio_limit, Some(0.));
            assert_eq!(resp.arguments.torrents[4].seed_ratio_limit, Some(0.75));
            assert_eq!(resp.arguments.torrents[5].seed_ratio_limit, Some(3.));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_seed_ratio_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seed_ratio_limit, None);
            Ok(())
        },
    )
}

// ----- seed_ratio_mode (seedRatioMode, SeedRatioMode) --------------------

#[test]
fn test_torrent_get_seed_ratio_mode_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "seedRatioMode":2 },
                    { "seedRatioMode":1 },
                    { "seedRatioMode":0 },
                    { "seed_ratio_mode":2 },
                    { "seed_ratio_mode":1 },
                    { "seed_ratio_mode":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].seed_ratio_mode,
                Some(RatioMode::Unlimited)
            );
            assert_eq!(
                resp.arguments.torrents[1].seed_ratio_mode,
                Some(RatioMode::Single)
            );
            assert_eq!(
                resp.arguments.torrents[2].seed_ratio_mode,
                Some(RatioMode::Global)
            );
            assert_eq!(
                resp.arguments.torrents[3].seed_ratio_mode,
                Some(RatioMode::Unlimited)
            );
            assert_eq!(
                resp.arguments.torrents[4].seed_ratio_mode,
                Some(RatioMode::Single)
            );
            assert_eq!(
                resp.arguments.torrents[5].seed_ratio_mode,
                Some(RatioMode::Global)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_seed_ratio_mode_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].seed_ratio_mode, None);
            Ok(())
        },
    )
}

// ----- sequential_download (sequentialDownload, SequentialDownload) --------------------

#[test]
fn test_torrent_get_sequential_download_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "sequential_download":true },
                    { "sequential_download":false }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].sequential_download, Some(true));
            assert_eq!(resp.arguments.torrents[1].sequential_download, Some(false));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_sequential_download_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].sequential_download, None);
            Ok(())
        },
    )
}

// ----- sequential_download_from_piece (SequentialDownloadFromPiece) --------------------

#[test]
fn test_torrent_get_sequential_download_from_piece_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "sequential_download_from_piece":234 },
                    { "sequential_download_from_piece":9001 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].sequential_download_from_piece, Some(234));
            assert_eq!(resp.arguments.torrents[1].sequential_download_from_piece, Some(9001));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_sequential_download_from_piece_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].sequential_download_from_piece, None);
            Ok(())
        },
    )
}

// ----- size_when_done (sizeWhenDone, SizeWhenDone) --------------------

#[test]
fn test_torrent_get_size_when_done_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "sizeWhenDone":2965366874 },
                    { "size_when_done":11111 },
                    { "size_when_done":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].size_when_done, Some(2965366874));
            assert_eq!(resp.arguments.torrents[1].size_when_done, Some(11111));
            assert_eq!(resp.arguments.torrents[2].size_when_done, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_size_when_done_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].size_when_done, None);
            Ok(())
        },
    )
}

// ----- start_date (startDate, StartDate) --------------------

#[test]
fn test_torrent_get_start_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "startDate":0 },
                    { "startDate":-1 },
                    { "startDate":1723479770 },
                    { "start_date":0 },
                    { "start_date":-1 },
                    { "start_date":1774014859 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].start_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[1].start_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].start_date,
                Some(DateTime::parse_from_rfc3339("2024-08-12T16:22:50Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[3].start_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[4].start_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[5].start_date,
                Some(DateTime::parse_from_rfc3339("2026-03-20 13:54:19+00:00")?.to_utc()),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_start_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].start_date, None);
            Ok(())
        },
    )
}

// ----- status (Status) --------------------

#[test]
fn test_torrent_get_status_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "status":0 },
                    { "status":1 },
                    { "status":2 },
                    { "status":3 },
                    { "status":4 },
                    { "status":5 },
                    { "status":6 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        7,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].status,
                Some(TorrentStatus::Stopped)
            );
            assert_eq!(
                resp.arguments.torrents[1].status,
                Some(TorrentStatus::QueuedToVerify)
            );
            assert_eq!(
                resp.arguments.torrents[2].status,
                Some(TorrentStatus::Verifying)
            );
            assert_eq!(
                resp.arguments.torrents[3].status,
                Some(TorrentStatus::QueuedToDownload)
            );
            assert_eq!(
                resp.arguments.torrents[4].status,
                Some(TorrentStatus::Downloading)
            );
            assert_eq!(
                resp.arguments.torrents[5].status,
                Some(TorrentStatus::QueuedToSeed)
            );
            assert_eq!(
                resp.arguments.torrents[6].status,
                Some(TorrentStatus::Seeding)
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_status_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].status, None);
            Ok(())
        },
    )
}

// ----- torrent_file (torrentFile, TorrentFile) --------------------

#[test]
fn test_torrent_get_torrent_file_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "torrentFile":"/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent" },
                    { "torrent_file":"/torrents/1f735c2f71631bfed78d5bc9047cf8c0d21dc069.torrent" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].torrent_file,
                Some("/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent".into()),
            );
            assert_eq!(
                resp.arguments.torrents[1].torrent_file,
                Some("/torrents/1f735c2f71631bfed78d5bc9047cf8c0d21dc069.torrent".into()),
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_torrent_file_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].torrent_file, None);
            Ok(())
        },
    )
}

// ----- total_size (totalSize, TotalSize) --------------------

#[test]
fn test_torrent_get_total_size_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "totalSize":2050306968 },
                    { "total_size":4482 },
                    { "total_size":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].total_size, Some(2050306968));
            assert_eq!(resp.arguments.torrents[1].total_size, Some(4482));
            assert_eq!(resp.arguments.torrents[2].total_size, Some(0));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_total_size_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].total_size, None);
            Ok(())
        },
    )
}

// ----- trackers (Tracker) --------------------

#[test]
fn test_torrent_get_trackers_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    {
                        "trackers": [
                            {
                                "announce":"https://example.com:1024/announce",
                                "id":0,
                                "scrape":"https://example.com:1024/scrape",
                                "tier":0
                            },
                            {
                                "announce":"https://example.com:2048/announce",
                                "id":1,
                                "scrape":"https://example.com:2048/scrape",
                                "tier":0
                            }
                        ]
                    },
                    {
                        "trackers": [
                            {
                                "announce":"https://example.com:4096/announce",
                                "id":44023,
                                "scrape":"https://example.com:4096/scrape",
                                "sitename":"example",
                                "tier":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let first = resp.arguments.torrents[0]
                .trackers
                .as_ref()
                .expect("trackers should exist");
            assert_eq!(first.len(), 2);
            assert_eq!(first[0].id, 0);
            assert_eq!(first[0].announce, "https://example.com:1024/announce");
            assert_eq!(first[0].scrape, "https://example.com:1024/scrape");
            assert_eq!(first[0].sitename, "");
            assert_eq!(first[0].tier, 0);
            assert_eq!(first[1].id, 1);
            assert_eq!(first[1].announce, "https://example.com:2048/announce");
            assert_eq!(first[1].scrape, "https://example.com:2048/scrape");
            assert_eq!(first[1].sitename, "");
            assert_eq!(first[1].tier, 0);
            let second = resp.arguments.torrents[1]
                .trackers
                .as_ref()
                .expect("trackers should exist");
            assert_eq!(second.len(), 1);
            assert_eq!(second[0].id, 44023);
            assert_eq!(second[0].announce, "https://example.com:4096/announce");
            assert_eq!(second[0].scrape, "https://example.com:4096/scrape");
            assert_eq!(second[0].sitename, "example");
            assert_eq!(second[0].tier, 0);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_trackers_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].trackers.is_none());
            Ok(())
        },
    )
}

// ----- tracker_list (trackerList, TrackerList) --------------------

#[test]
fn test_torrent_get_tracker_list_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "trackerList":"https://example.org/a\n\nhttp://example.org/b\n" },
                    { "tracker_list":"http://bt1.archive.org:6969/announce\n\nhttp://bt2.archive.org:6969/announce\n" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(
                resp.arguments.torrents[0].tracker_list,
                Some(vec![
                    vec![Url::parse("https://example.org/a")?],
                    vec![Url::parse("http://example.org/b")?],
                ].into()));
            assert_eq!(
                resp.arguments.torrents[1].tracker_list,
                Some(vec![
                    vec![Url::parse("http://bt1.archive.org:6969/announce")?],
                    vec![Url::parse("http://bt2.archive.org:6969/announce")?],
                ].into()));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_tracker_list_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].tracker_list, None);
            Ok(())
        },
    )
}

// ----- tracker_stats (trackerStats, TrackerStats) --------------------

#[test]
fn test_torrent_get_tracker_stats_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    {
                        "trackerStats": [
                            {
                                "announce":"https://example.com/announce",
                                "announceState":1,
                                "downloadCount":245,
                                "downloader_count":-1,
                                "hasAnnounced":true,
                                "hasScraped":true,
                                "host":"https://example.com:8080",
                                "id":0,
                                "isBackup":false,
                                "lastAnnouncePeerCount":86,
                                "lastAnnounceResult":"Success",
                                "lastAnnounceStartTime":1723614865,
                                "lastAnnounceSucceeded":true,
                                "lastAnnounceTime":1723614865,
                                "lastAnnounceTimedOut":false,
                                "lastScrapeResult":"Could not connect to tracker",
                                "lastScrapeStartTime":0,
                                "lastScrapeSucceeded":false,
                                "lastScrapeTime":1723614865,
                                "lastScrapeTimedOut":false,
                                "leecherCount":9,
                                "nextAnnounceTime":1723618230,
                                "nextScrapeTime":0,
                                "scrapeState":2,
                                "scrape":"",
                                "seederCount":77,
                                "tier":0
                            }
                        ]
                    },
                    {
                        "tracker_stats": [
                            {
                                "announce":"http://example.org/foo/announce",
                                "announce_state":0,
                                "download_count":24,
                                "downloader_count":7,
                                "has_announced":false,
                                "has_scraped":false,
                                "host":"http://example.org:8080",
                                "id":666,
                                "is_backup":true,
                                "last_announce_peer_count":9999,
                                "last_announce_result":"IPv4 connection failed",
                                "last_announce_start_time":0,
                                "last_announce_succeeded":false,
                                "last_announce_time":1773989659,
                                "last_announce_timed_out":false,
                                "last_scrape_result":"Could not connect to tracker",
                                "last_scrape_start_time":0,
                                "last_scrape_succeeded":false,
                                "last_scrape_time":1723614865,
                                "last_scrape_timed_out":false,
                                "leecher_count":2,
                                "next_announce_time":1723618230,
                                "next_scrape_time":0,
                                "scrape_state":0,
                                "scrape":"http://example.org/scrape",
                                "seeder_count":5,
                                "sitename":"example",
                                "tier":1
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let first = resp.arguments.torrents[0]
                .tracker_stats
                .as_ref()
                .expect("tracker_stats should exist");
            assert_eq!(first.len(), 1);
            assert_eq!(
                first[0].announce,
                "https://example.com/announce".to_string()
            );
            assert!(matches!(first[0].announce_state, TrackerState::Waiting));
            assert_eq!(first[0].download_count, 245);
            assert_eq!(first[0].downloader_count, -1);
            assert_eq!(first[0].has_announced, true);
            assert_eq!(first[0].has_scraped, true);
            assert_eq!(first[0].host, "https://example.com:8080");
            assert!(matches!(first[0].id, 0));
            assert_eq!(first[0].is_backup, false);
            assert_eq!(first[0].last_announce_peer_count, 86);
            assert_eq!(first[0].last_announce_result, "Success".to_string());
            assert_eq!(
                first[0].last_announce_start_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(first[0].last_announce_succeeded, true);
            assert_eq!(
                first[0].last_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(first[0].last_announce_timed_out, false);
            assert_eq!(
                first[0].last_scrape_result,
                "Could not connect to tracker".to_string()
            );
            assert_eq!(first[0].last_scrape_start_time, DateTime::UNIX_EPOCH);
            assert_eq!(first[0].last_scrape_succeeded, false);
            assert_eq!(
                first[0].last_scrape_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(first[0].last_scrape_timed_out, false);
            assert_eq!(first[0].leecher_count, 9);
            assert_eq!(
                first[0].next_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")?.to_utc(),
            );
            assert_eq!(first[0].next_scrape_time, DateTime::UNIX_EPOCH);
            assert!(matches!(first[0].scrape_state, TrackerState::Queued));
            assert_eq!(first[0].scrape, "".to_string());
            assert_eq!(first[0].seeder_count, 77);
            assert_eq!(first[0].sitename, "".to_string());
            assert_eq!(first[0].tier, 0);

            let second = resp.arguments.torrents[1]
                .tracker_stats
                .as_ref()
                .expect("tracker_stats should exist");
            assert_eq!(second.len(), 1);
            assert_eq!(
                second[0].announce,
                "http://example.org/foo/announce".to_string()
            );
            assert!(matches!(second[0].announce_state, TrackerState::Inactive));
            assert_eq!(second[0].download_count, 24);
            assert_eq!(second[0].downloader_count, 7);
            assert_eq!(second[0].has_announced, false);
            assert_eq!(second[0].has_scraped, false);
            assert_eq!(second[0].host, "http://example.org:8080");
            assert!(matches!(second[0].id, 666));
            assert_eq!(second[0].is_backup, true);
            assert_eq!(second[0].last_announce_peer_count, 9999);
            assert_eq!(second[0].last_announce_result, "IPv4 connection failed".to_string());
            assert_eq!(second[0].last_announce_start_time, DateTime::UNIX_EPOCH);
            assert_eq!(second[0].last_announce_succeeded, false);
            assert_eq!(
                second[0].last_announce_time,
                DateTime::parse_from_rfc3339("2026-03-20 06:54:19+00:00")?.to_utc(),
            );
            assert_eq!(second[0].last_announce_timed_out, false);
            assert_eq!(
                second[0].last_scrape_result,
                "Could not connect to tracker".to_string()
            );
            assert_eq!(second[0].last_scrape_start_time, DateTime::UNIX_EPOCH);
            assert_eq!(second[0].last_scrape_succeeded, false);
            assert_eq!(
                second[0].last_scrape_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(second[0].last_scrape_timed_out, false);
            assert_eq!(second[0].leecher_count, 2);
            assert_eq!(
                second[0].next_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")?.to_utc(),
            );
            assert_eq!(second[0].next_scrape_time, DateTime::UNIX_EPOCH);
            assert!(matches!(second[0].scrape_state, TrackerState::Inactive));
            assert_eq!(second[0].scrape, "http://example.org/scrape".to_string());
            assert_eq!(second[0].seeder_count, 5);
            assert_eq!(second[0].sitename, "example".to_string());
            assert_eq!(second[0].tier, 1);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_tracker_stats_semver_530_compat() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    {
                        "trackerStats": [
                            {
                                "announce":"https://example.com/announce",
                                "announceState":1,
                                "downloadCount":245,
                                "hasAnnounced":true,
                                "hasScraped":true,
                                "host":"https://example.com:8080",
                                "id":0,
                                "isBackup":false,
                                "lastAnnouncePeerCount":86,
                                "lastAnnounceResult":"Success",
                                "lastAnnounceStartTime":1723614865,
                                "lastAnnounceSucceeded":true,
                                "lastAnnounceTime":1723614865,
                                "lastAnnounceTimedOut":false,
                                "lastScrapeResult":"Could not connect to tracker",
                                "lastScrapeStartTime":0,
                                "lastScrapeSucceeded":false,
                                "lastScrapeTime":1723614865,
                                "lastScrapeTimedOut":false,
                                "leecherCount":9,
                                "nextAnnounceTime":1723618230,
                                "nextScrapeTime":0,
                                "scrapeState":2,
                                "scrape":"",
                                "seederCount":77,
                                "tier":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        |resp| {
            let first = resp.arguments.torrents[0]
                .tracker_stats
                .as_ref()
                .expect("tracker_stats should exist");
            assert_eq!(first.len(), 1);

            // Verify the Torrent deserialized okay even if the response json is missing fields
            // from semver-5.3.0 and later.
            assert_eq!(first[0].downloader_count, -1); // Added in semver-6.0.0
            assert_eq!(first[0].sitename, "".to_string()); // Added in semver-5.3.0

            assert_eq!(
                first[0].announce,
                "https://example.com/announce".to_string()
            );
            assert!(matches!(first[0].announce_state, TrackerState::Waiting));
            assert_eq!(first[0].download_count, 245);
            assert_eq!(first[0].has_announced, true);
            assert_eq!(first[0].has_scraped, true);
            assert_eq!(first[0].host, "https://example.com:8080");
            assert!(matches!(first[0].id, 0));
            assert_eq!(first[0].is_backup, false);
            assert_eq!(first[0].last_announce_peer_count, 86);
            assert_eq!(first[0].last_announce_result, "Success".to_string());
            assert_eq!(
                first[0].last_announce_start_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(first[0].last_announce_succeeded, true);
            assert_eq!(
                first[0].last_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(first[0].last_announce_timed_out, false);
            assert_eq!(
                first[0].last_scrape_result,
                "Could not connect to tracker".to_string()
            );
            assert_eq!(first[0].last_scrape_start_time, DateTime::UNIX_EPOCH);
            assert_eq!(first[0].last_scrape_succeeded, false);
            assert_eq!(
                first[0].last_scrape_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert_eq!(first[0].last_scrape_timed_out, false);
            assert_eq!(first[0].leecher_count, 9);
            assert_eq!(
                first[0].next_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")?.to_utc(),
            );
            assert_eq!(first[0].next_scrape_time, DateTime::UNIX_EPOCH);
            assert!(matches!(first[0].scrape_state, TrackerState::Queued));
            assert_eq!(first[0].scrape, "".to_string());
            assert_eq!(first[0].seeder_count, 77);
            assert_eq!(first[0].tier, 0);
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_tracker_stats_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert!(resp.arguments.torrents[0].tracker_stats.is_none());
            Ok(())
        },
    )
}

// ----- upload_ratio (uploadRatio, UploadRatio) --------------------

#[test]
fn test_torrent_get_upload_ratio_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadRatio":-1 },
                    { "uploadRatio":0 },
                    { "uploadRatio":1.23 },
                    { "upload_ratio":-1 },
                    { "upload_ratio":0 },
                    { "upload_ratio":6.92 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        6,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].upload_ratio, Some(-1.));
            assert_eq!(resp.arguments.torrents[1].upload_ratio, Some(0.));
            assert_eq!(resp.arguments.torrents[2].upload_ratio, Some(1.23));
            assert_eq!(resp.arguments.torrents[3].upload_ratio, Some(-1.));
            assert_eq!(resp.arguments.torrents[4].upload_ratio, Some(0.));
            assert_eq!(resp.arguments.torrents[5].upload_ratio, Some(6.92));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_upload_ratio_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].upload_ratio, None);
            Ok(())
        },
    )
}

// ----- uploaded_ever (uploadedEver, UploadedEver) --------------------

#[test]
fn test_torrent_get_uploaded_ever_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadedEver":0 },
                    { "uploadedEver":1301396208 },
                    { "uploaded_ever":0 },
                    { "uploaded_ever":4567 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].uploaded_ever, Some(0));
            assert_eq!(resp.arguments.torrents[1].uploaded_ever, Some(1301396208));
            assert_eq!(resp.arguments.torrents[2].uploaded_ever, Some(0));
            assert_eq!(resp.arguments.torrents[3].uploaded_ever, Some(4567));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_uploaded_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].uploaded_ever, None);
            Ok(())
        },
    )
}

// ----- upload_limit (uploadLimit, UploadLimit) --------------------

#[test]
fn test_torrent_get_upload_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadLimit":0 },
                    { "uploadLimit":1024 },
                    { "upload_limit":0 },
                    { "upload_limit":250 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].upload_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].upload_limit, Some(1024));
            assert_eq!(resp.arguments.torrents[2].upload_limit, Some(0));
            assert_eq!(resp.arguments.torrents[3].upload_limit, Some(250));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_upload_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].upload_limit, None);
            Ok(())
        },
    )
}

// ----- upload_limited (uploadLimited, UploadLimited) --------------------

#[test]
fn test_torrent_get_upload_limited_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadLimited":false },
                    { "uploadLimited":true },
                    { "upload_limited":false },
                    { "upload_limited":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].upload_limited, Some(false));
            assert_eq!(resp.arguments.torrents[1].upload_limited, Some(true));
            assert_eq!(resp.arguments.torrents[2].upload_limited, Some(false));
            assert_eq!(resp.arguments.torrents[3].upload_limited, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_upload_limited_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].upload_limited, None);
            Ok(())
        },
    )
}

// ----- wanted (Wanted) --------------------

#[test]
fn test_torrent_get_wanted_int_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "wanted":[0, 1, 0, 0, 1] },
                    { "wanted":[] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let wanted = resp.arguments.torrents[0]
                .wanted
                .as_ref()
                .expect("wanted is some");
            assert_eq!(wanted, &vec![false, true, false, false, true]);
            assert_eq!(
                resp.arguments.torrents[1].wanted.as_ref().expect("wanted is some"),
                &Vec::<bool>::new());
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_wanted_bool_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "wanted":[true, true, false, false, true, false, true] },
                    { "wanted":[] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            let wanted = resp.arguments.torrents[0]
                .wanted
                .as_ref()
                .expect("wanted is some");
            assert_eq!(wanted, &vec![true, true, false, false, true, false, true]);
            assert_eq!(
                resp.arguments.torrents[1].wanted.as_ref().expect("wanted is some"),
                &Vec::<bool>::new());
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_wanted_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].wanted, None);
            Ok(())
        },
    )
}

// ----- webseeds (Webseeds) --------------------

#[test]
fn test_torrent_get_webseeds_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "webseeds":[] },
                    { "webseeds":["https://example.com/"] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].webseeds, Some(vec![]));
            assert_eq!(
                resp.arguments.torrents[1].webseeds,
                Some(vec!["https://example.com/".into()])
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_webseeds_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].webseeds, None);
            Ok(())
        },
    )
}


// ----- webseeds_ex (WebseedsEx) --------------------

#[test]
#[allow(unreachable_code)] // TODO: Remove when implemented
#[ignore] // TODO: Remove when implemented
fn test_torrent_get_webseeds_ex_success() -> Result<()> {
    todo!(); // Need webseeds_ex data

    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "webseeds_ex":[] },
                    { "webseeds_ex":[
                        {
                            "url":"https://example.com",
                            "is_downloading":true,
                            "download_bytes_per_second":0
                        },
                        {
                            "url":"https://example.com/foo/lorem.png",
                            "is_downloading":true,
                            "download_bytes_per_second":12345
                        },
                        {
                            "url":"https://example.com/foo/bar.iso",
                            "is_downloading":false,
                            "download_bytes_per_second":0
                        }
                    ] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].webseeds_ex, Some(vec![]));
            assert_eq!(
                resp.arguments.torrents[1].webseeds_ex,
                Some(vec![
                    WebseedsEx {
                        url: "https://example.com".into(),
                        is_downloading: true,
                        download_bytes_per_second: 0,
                    },
                    WebseedsEx {
                        url: "https://example.com/foo/lorem.png".into(),
                        is_downloading: true,
                        download_bytes_per_second: 12345,
                    },
                    WebseedsEx {
                        url: "https://example.com/foo/bar.iso".into(),
                        is_downloading: false,
                        download_bytes_per_second: 0,
                    },
                ])
            );
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_webseeds_ex_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].webseeds_ex, None);
            Ok(())
        },
    )
}

// ----- webseeds_sending_to_us (webseedsSendingToUs, WebseedsSendingToUs) --------------------

#[test]
fn test_torrent_get_webseeds_sending_to_us_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "webseedsSendingToUs":0 },
                    { "webseedsSendingToUs":1234 },
                    { "webseeds_sending_to_us":0 },
                    { "webseeds_sending_to_us":1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].webseeds_sending_to_us, Some(0));
            assert_eq!(resp.arguments.torrents[1].webseeds_sending_to_us, Some(1234));
            assert_eq!(resp.arguments.torrents[2].webseeds_sending_to_us, Some(0));
            assert_eq!(resp.arguments.torrents[3].webseeds_sending_to_us, Some(1));
            Ok(())
        },
    )
}

#[test]
fn test_torrent_get_webseeds_sending_to_us_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        |resp| {
            assert_eq!(resp.arguments.torrents[0].webseeds_sending_to_us, None);
            Ok(())
        },
    )
}
