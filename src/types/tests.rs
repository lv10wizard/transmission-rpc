use chrono::DateTime;
use serde_json;

use crate::types::{request::Priority, Result, RpcResponse, Torrents, Torrent};

type TorrentGetResp = RpcResponse<Torrents<Torrent>>;

/// torrent-get test helper to consolidate unit test boilerplate.
fn test_torrent_get(
    resp: TorrentGetResp,
    verify: Box<dyn Fn(&TorrentGetResp) -> Result<()>>,
) -> Result<()>
{
    println!("{resp:#?}");
    assert!(resp.is_ok());
    assert_eq!(resp.arguments.torrents.len(), 1);
    verify(&resp)
}

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

// ----- activity_date (activityDate, ActivityDate) --------------------

#[test]
fn test_torrent_get_activity_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "activityDate":1718947434 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].activity_date,
            Some(DateTime::parse_from_rfc3339("2024-06-21T05:23:54Z")?.to_utc()),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_activity_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].activity_date, None);
        Ok(())
    }))
}

// ----- added_date (addedDate, AddedDate) --------------------

#[test]
fn test_torrent_get_added_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "addedDate":1670612948 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].added_date,
            Some(DateTime::parse_from_rfc3339("2022-12-09T19:09:08Z")?.to_utc()),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_added_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].added_date, None);
        Ok(())
    }))
}

// ----- availability (Availability) --------------------

#[test]
#[allow(unreachable_code)] // TODO: Remove when implemented
fn test_torrent_get_availability_success() -> Result<()> {
    todo!();

    let resp = serde_json::from_str(
        // TODO: Need availability data
        r#"
        {
            "arguments": {
                "torrents": [
                    { "availability":[] }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].availability,
            Some(vec![]),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_availability_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].availability, None);
        Ok(())
    }))
}

// ----- bandwidth_priority (bandwidthPriority, BandwidthPriority) --------------------

#[test]
fn test_torrent_get_bandwidth_priority_low() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "bandwidthPriority":-1 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].bandwidth_priority,
            Some(Priority::Low),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_bandwidth_priority_normal() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "bandwidthPriority":0 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].bandwidth_priority,
            Some(Priority::Normal),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_bandwidth_priority_high() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "bandwidthPriority":1 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].bandwidth_priority,
            Some(Priority::High),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_bandwidth_priority_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].bandwidth_priority, None);
        Ok(())
    }))
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
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].comment, Some("lorem ipsum".into()));
        Ok(())
    }))
}

#[test]
fn test_torrent_get_comment_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].comment, None);
        Ok(())
    }))
}

// ----- corrupt_ever (corruptEver, CorruptEver) --------------------

#[test]
fn test_torrent_get_corrupt_ever_success() -> Result<()> {
    // TODO? Is it worth testing corruptEver==0 also?
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "corruptEver":4096 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].corrupt_ever, Some(4096));
        Ok(())
    }))
}

#[test]
fn test_torrent_get_corrupt_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].corrupt_ever, None);
        Ok(())
    }))
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
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].creator, Some("mktorrent 1.1".into()));
        Ok(())
    }))
}

#[test]
fn test_torrent_get_creator_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].creator, None);
        Ok(())
    }))
}

// ----- date_created (dateCreated, DateCreated) --------------------

#[test]
fn test_torrent_get_date_created_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "dateCreated":1592962706 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].date_created,
            Some(DateTime::parse_from_rfc3339("2020-06-24T01:38:26Z")?.to_utc()),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_date_created_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].date_created, None);
        Ok(())
    }))
}

// ----- desired_available (desiredAvailable, DesiredAvailable) --------------------

#[test]
fn test_torrent_get_desired_available_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "desiredAvailable":20162576 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].desired_available, Some(20162576));
        Ok(())
    }))
}

#[test]
fn test_torrent_get_desired_available_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].desired_available, None);
        Ok(())
    }))
}

// ----- done_date (doneDate, DoneDate) --------------------

#[test]
fn test_torrent_get_done_date_not_done() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "doneDate":0 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].done_date, Some(DateTime::UNIX_EPOCH));
        Ok(())
    }))
}

#[test]
fn test_torrent_get_done_date_is_done() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "doneDate":1672060369 }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(
            resp.arguments.torrents[0].done_date,
            Some(DateTime::parse_from_rfc3339("2022-12-26T13:12:49Z")?.to_utc()),
        );
        Ok(())
    }))
}

#[test]
fn test_torrent_get_done_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].done_date, None);
        Ok(())
    }))
}

// ----- download_dir (downloadDir, DownloadDir) --------------------

#[test]
fn test_torrent_get_download_dir_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadDir":"/downloads/iso/" }
                ]
            },
            "result":"success"
        }
        "#
    )?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].download_dir, Some("/downloads/iso/".into()));
        Ok(())
    }))
}

#[test]
fn test_torrent_get_download_dir_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(resp, Box::new(|resp: &TorrentGetResp| {
        assert_eq!(resp.arguments.torrents[0].download_dir, None);
        Ok(())
    }))
}

// ----- downloaded_ever (downloadedEver, DownloadedEver) --------------------

// TODO

