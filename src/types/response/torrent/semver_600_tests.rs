//! This file defines semver-6.0.0+ [`Torrent`] deserialization tests.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use chrono::DateTime;
use serde_json::Result as SerdeResult;
use test_case::test_case;
use url::Url;

use crate::{
json_rpc::JsonRpcResponse, types::{
    JSON_RPC_VERSION_2_0,
    ErrorType, File, FileStat, IdleMode, Peer, PeersFrom, Priority,
    RatioMode, Result, RpcResponse, Torrent, TorrentStatus, Torrents, Tracker, TrackerStat,
    TrackerState, WebseedsEx,
}};

/// Deserializes any [`IntoIterator`] (eg. `Vec<_>` or `[_]`) of torrent response data from a
/// jsonrpc formatted json string.
///
/// ### Arguments
///
/// * `data`: Any collection strings implementing [`IntoIterator`] (like `Vec<_>` or `[_]`) of
/// torrent response data. Each item should represent a single [`Torrent`]'s json data.
fn torrents_json<I, S>(data: I) -> SerdeResult<RpcResponse<Torrents<Torrent>>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let data = data.into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",");
    println!("data> {data}");
    let formatted = format!("{{\
        \"id\":0,\
        \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
        \"result\":{{\
            \"torrents\":[\
                {data}\
            ]\
        }}\
    }}");
    println!("formatted>\n{formatted}\n");
    serde_json::from_str::<JsonRpcResponse<Torrents<Torrent>>>(&formatted)
        .map(Into::into)
}

#[test]
fn torrents_deserialize_empty() -> Result<()> {
    let resp = torrents_json::<_, &str>([])?;

    println!("{resp:#?}");
    assert!(resp.is_ok());

    assert_eq!(resp.arguments.torrents.is_empty(), true);
    Ok(())
}

#[test]
fn torrents_deserialize_multiple() -> Result<()> {
    let resp = torrents_json([
        r#"{ "id": 123 }"#,
        r#"{ "comment": "test comment" }"#,
        r#"{ "creator": "foo bar v0.2.0-23" }"#,
    ])?;

    println!("{resp:#?}");
    assert!(resp.is_ok());

    assert_eq!(resp.arguments.torrents.len(), 3);
    assert_eq!(resp.arguments.torrents[0], Torrent {
        id: Some(123),
        ..Default::default()
    });
    assert_eq!(resp.arguments.torrents[1], Torrent {
        comment: Some("test comment".into()),
        ..Default::default()
    });
    assert_eq!(resp.arguments.torrents[2], Torrent {
        creator: Some("foo bar v0.2.0-23".into()),
        ..Default::default()
    });
    Ok(())
}

#[test_case(r#"{"activity_date":1718947434}"# => Torrent {
        activity_date: DateTime::parse_from_rfc3339("2024-06-21T05:23:54Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 activity date"
)]
#[test_case(r#"{"activity_date":-1}"# => Torrent {
        activity_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 activity date negative"
)]
#[test_case(r#"{"activity_date":0}"# => Torrent {
        activity_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 activity date zero"
)]

#[test_case(r#"{"added_date":1670612948}"# => Torrent {
        added_date: DateTime::parse_from_rfc3339("2022-12-09T19:09:08Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 added date"
)]
#[test_case(r#"{"added_date":-1}"# => Torrent {
        added_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 added date negative"
)]
#[test_case(r#"{"added_date":0}"# => Torrent {
        added_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 added date zero"
)]

#[test_case(r#"{"availability":[-1,0,1,2,3,10,-1]}"# => Torrent {
        availability: Some(vec![-1,0,1,2,3,10,-1]),
        ..Default::default()
    } ; "semver 6.0.0 availability"
)]
#[test_case(r#"{"availability":[]}"# => Torrent {
        availability: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 availability empty"
)]

#[test_case(r#"{"bandwidth_priority":0}"# => Torrent {
        bandwidth_priority: Some(Priority::Normal),
        ..Default::default()
    } ; "semver 6.0.0 bandwidth priority normal"
)]

#[test_case(r#"{"bytes_completed":[]}"# => Torrent {
        bytes_completed: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 bytes completed empty"
)]
#[test_case(r#"{"bytes_completed":[790626304]}"# => Torrent {
        bytes_completed: Some(vec![790626304]),
        ..Default::default()
    } ; "semver 6.0.0 bytes completed single"
)]
#[test_case(r#"{"bytes_completed":[1234,567890,443,8080]}"# => Torrent {
        bytes_completed: Some(vec![1234,567890,443,8080]),
        ..Default::default()
    } ; "semver 6.0.0 bytes completed multiple"
)]
#[test_case(r#"{"bytes_completed":[0]}"# => Torrent {
        bytes_completed: Some(vec![0]),
        ..Default::default()
    } ; "semver 6.0.0 bytes completed zero"
)]
#[test_case(r#"{"bytes_completed":[-1]}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 bytes completed negative"
)]

#[test_case(r#"{"comment":"lorem ipsum"}"# => Torrent {
        comment: Some("lorem ipsum".into()),
        ..Default::default()
    } ; "semver 6.0.0 comment"
)]
#[test_case(r#"{"comment":""}"# => Torrent {
        comment: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 comment empty"
)]

#[test_case(r#"{"corrupt_ever":4096}"# => Torrent {
        corrupt_ever: Some(4096),
        ..Default::default()
    } ; "semver 6.0.0 corrupt ever"
)]
#[test_case(r#"{"corrupt_ever":0}"# => Torrent {
        corrupt_ever: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 corrupt ever zero"
)]
#[test_case(r#"{"corrupt_ever":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 corrupt ever negative"
)]

#[test_case(r#"{"creator":"mktorrent 1.1"}"# => Torrent {
        creator: Some("mktorrent 1.1".into()),
        ..Default::default()
    } ; "semver 6.0.0 creator"
)]
#[test_case(r#"{"creator":""}"# => Torrent {
        creator: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 creator empty"
)]

#[test_case(r#"{"date_created":1592962706}"# => Torrent {
        date_created: DateTime::parse_from_rfc3339("2020-06-24T01:38:26Z")
            .ok()
            .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 date created"
)]
#[test_case(r#"{"date_created":0}"# => Torrent {
        date_created: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 date created zero"
)]
#[test_case(r#"{"date_created":-1}"# => Torrent {
        date_created: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 date created negative"
)]

#[test_case(r#"{"desired_available":20162576}"# => Torrent {
        desired_available: Some(20162576),
        ..Default::default()
    } ; "semver 6.0.0 desired available"
)]
#[test_case(r#"{"desired_available":0}"# => Torrent {
        desired_available: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 desired available zero"
)]
#[test_case(r#"{"desired_available":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 desired available negative"
)]

#[test_case(r#"{"done_date":1672060369}"# => Torrent {
        done_date: DateTime::parse_from_rfc3339("2022-12-26T13:12:49Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 done date"
)]
#[test_case(r#"{"done_date":-1}"# => Torrent {
        done_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 done date negative"
)]
#[test_case(r#"{"done_date":0}"# => Torrent {
        done_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 done date zero"
)]

#[test_case(r#"{"download_dir":"/downloads/iso/"}"# => Torrent {
        download_dir: Some("/downloads/iso/".into()),
        ..Default::default()
    } ; "semver 6.0.0 download dir"
)]
#[test_case(r#"{"download_dir":""}"# => Torrent {
        download_dir: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 download dir empty"
)]

#[test_case(r#"{"downloaded_ever":1340189370}"# => Torrent {
        downloaded_ever: Some(1340189370),
        ..Default::default()
    } ; "semver 6.0.0 downloaded ever"
)]
#[test_case(r#"{"downloaded_ever":0}"# => Torrent {
        downloaded_ever: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 downloaded ever zero"
)]
#[test_case(r#"{"downloaded_ever":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 downloaded ever negative"
)]

#[test_case(r#"{"download_limit":2048}"# => Torrent {
        download_limit: Some(2048),
        ..Default::default()
    } ; "semver 6.0.0 downloaded limit"
)]
#[test_case(r#"{"download_limit":0}"# => Torrent {
        download_limit: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 downloaded limit zero"
)]
#[test_case(r#"{"download_limit":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 downloaded limit negative"
)]

#[test_case(r#"{"download_limited":true}"# => Torrent {
        download_limited: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 downloaded limit true"
)]
#[test_case(r#"{"download_limited":false}"# => Torrent {
        download_limited: Some(false),
        ..Default::default()
    } ; "semver 6.0.0 downloaded limit false"
)]

#[test_case(r#"{"edit_date":1723512675}"# => Torrent {
        edit_date: DateTime::parse_from_rfc3339("2024-08-13T01:31:15Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 edit date"
)]
#[test_case(r#"{"edit_date":0}"# => Torrent {
        edit_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 edit date zero"
)]
#[test_case(r#"{"edit_date":-1}"# => Torrent {
        edit_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 edit date negative"
)]

#[test_case(r#"{"error":0}"# => Torrent {
        error: Some(ErrorType::Ok),
        ..Default::default()
    } ; "semver 6.0.0 error ok"
)]

#[test_case(r#"{"error_string":"Unregistered torrent"}"# => Torrent {
        error_string: Some("Unregistered torrent".into()),
        ..Default::default()
    } ; "semver 6.0.0 error string unregistered torrent"
)]
#[test_case(r#"{"error_string":""}"# => Torrent {
        error_string: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 error string empty"
)]

#[test_case(r#"{"eta":82112}"# => Torrent {
        eta: Some(82112),
        ..Default::default()
    } ; "semver 6.0.0 eta"
)]
#[test_case(r#"{"eta":0}"# => Torrent {
        eta: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 eta zero"
)]
#[test_case(r#"{"eta":-1}"# => Torrent {
        eta: Some(-1),
        ..Default::default()
    } ; "semver 6.0.0 eta negative"
)]

#[test_case(r#"{"eta_idle":1234}"# => Torrent {
        eta_idle: Some(1234),
        ..Default::default()
    } ; "semver 6.0.0 eta idle"
)]
#[test_case(r#"{"eta_idle":0}"# => Torrent {
        eta_idle: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 eta idle zero"
)]
#[test_case(r#"{"eta_idle":-1}"# => Torrent {
        eta_idle: Some(-1),
        ..Default::default()
    } ; "semver 6.0.0 eta idle negative"
)]

#[test_case(r#"{"file_count":420}"# => Torrent {
        file_count: Some(420),
        ..Default::default()
    } ; "semver 6.0.0 file count"
)]
#[test_case(r#"{"file_count":0}"# => Torrent {
        file_count: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 file count zero"
)]
#[test_case(r#"{"file_count":-1}"# => panics "invalid value: integer `-1`, expected usize"
    ; "semver 6.0.0 file count negative"
)]

#[test_case(r#"{ "files": [] }"#
    => Torrent {
        files: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 files empty"
)]
#[test_case(
    r#"{
        "files": [
            {
                "bytes_completed":172415250,
                "length":3994091520,
                "name":"debian-12.6.0-amd64-DVD-1.iso"
            }
        ]
    }"#
    => Torrent {
        files: Some(vec![
            File {
                bytes_completed: 172415250,
                length: 3994091520,
                name: "debian-12.6.0-amd64-DVD-1.iso".into(),
                begin_piece: None,
                end_piece: None,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 files single"
)]
#[test_case(
    r#"{
        "files": [
            {
                "bytes_completed":172415250,
                "length":3994091520,
                "name":"debian-12.6.0-amd64-DVD-1.iso"
            },
            {
                "bytes_completed":0,
                "length":1229,
                "name":"Fedora-Server-40-1.14-x86_64-CHECKSUM"
            },
            {
                "bytes_completed":0,
                "length":2612854784,
                "name":"Fedora-Server-dvd-x86_64-40-1.14.iso"
            }
        ]
    }"#
    => Torrent {
        files: Some(vec![
            File {
                bytes_completed: 172415250,
                length: 3994091520,
                name: "debian-12.6.0-amd64-DVD-1.iso".into(),
                begin_piece: None,
                end_piece: None,
            },
            File {
                bytes_completed: 0,
                length: 1229,
                name: "Fedora-Server-40-1.14-x86_64-CHECKSUM".into(),
                begin_piece: None,
                end_piece: None,
            },
            File {
                bytes_completed: 0,
                length: 2612854784,
                name: "Fedora-Server-dvd-x86_64-40-1.14.iso".into(),
                begin_piece: None,
                end_piece: None,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 files multiple"
)]

#[test_case(r#"{ "file_stats": [] }"#
    => Torrent {
        file_stats: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 file stats empty"
)]
#[test_case(
    r#"{
        "file_stats": [
            {
                "bytes_completed": 0,
                "priority": 1,
                "wanted": false
            }
        ]
    }"#
    => Torrent {
        file_stats: Some(vec![
            FileStat {
                bytes_completed: 0,
                priority: Priority::High,
                wanted: false,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 file stats single"
)]
#[test_case(
    r#"{
        "file_stats": [
            {
                "bytes_completed": 1,
                "priority": -1,
                "wanted": false
            },
            {
                "bytes_completed": 2,
                "priority": 0,
                "wanted": true
            },
            {
                "bytes_completed": 300,
                "priority": 1,
                "wanted": true
            }
        ]
    }"#
    => Torrent {
        file_stats: Some(vec![
            FileStat {
                bytes_completed: 1,
                priority: Priority::Low,
                wanted: false,
            },
            FileStat {
                bytes_completed: 2,
                priority: Priority::Normal,
                wanted: true,
            },
            FileStat {
                bytes_completed: 300,
                priority: Priority::High,
                wanted: true,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 file stats multiple"
)]

#[test_case(r#"{"group":"foo bar"}"# => Torrent {
        group: Some("foo bar".into()),
        ..Default::default()
    } ; "semver 6.0.0 group"
)]
#[test_case(r#"{"group":""}"# => Torrent {
        group: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 group empty"
)]

#[test_case(r#"{"hash_string":"e08c426aab2cc58649ae5e73690e3747117b3470"}"# => Torrent {
        hash_string: Some("e08c426aab2cc58649ae5e73690e3747117b3470".into()),
        ..Default::default()
    } ; "semver 6.0.0 hash string"
)]
#[test_case(r#"{"hash_string":""}"# => Torrent {
        hash_string: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 hash string empty"
)]

#[test_case(r#"{"have_unchecked":39813}"# => Torrent {
        have_unchecked: Some(39813),
        ..Default::default()
    } ; "semver 6.0.0 have unchecked"
)]
#[test_case(r#"{"have_unchecked":0}"# => Torrent {
        have_unchecked: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 have unchecked zero"
)]
#[test_case(r#"{"have_unchecked":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 have unchecked negative"
)]

#[test_case(r#"{"have_valid":39813}"# => Torrent {
        have_valid: Some(39813),
        ..Default::default()
    } ; "semver 6.0.0 have valid"
)]
#[test_case(r#"{"have_valid":0}"# => Torrent {
        have_valid: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 have valid zero"
)]
#[test_case(r#"{"have_valid":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 have valid negative"
)]

#[test_case(r#"{"honors_session_limits":true}"# => Torrent {
        honors_session_limits: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 honors session limits true"
)]
#[test_case(r#"{"honors_session_limits":false}"# => Torrent {
        honors_session_limits: Some(false),
        ..Default::default()
    } ; "semver 6.0.0 honors session limits false"
)]

#[test_case(r#"{"id":1}"# => Torrent {
        id: Some(1),
        ..Default::default()
    } ; "semver 6.0.0 id"
)]
#[test_case(r#"{"id":0}"# => Torrent {
        id: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 id zero"
)]
#[test_case(r#"{"id":-1}"# => Torrent {
        id: Some(-1),
        ..Default::default()
    } ; "semver 6.0.0 id negative"
)]

#[test_case(r#"{"is_finished":true}"# => Torrent {
        is_finished: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 is finished true"
)]
#[test_case(r#"{"is_finished":false}"# => Torrent {
        is_finished: Some(false),
        ..Default::default()
    } ; "semver 6.0.0 is finished false"
)]

#[test_case(r#"{"is_private":true}"# => Torrent {
        is_private: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 is private true"
)]
#[test_case(r#"{"is_private":false}"# => Torrent {
        is_private: Some(false),
        ..Default::default()
    } ; "semver 6.0.0 is private false"
)]

#[test_case(r#"{"is_stalled":true}"# => Torrent {
        is_stalled: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 is stalled true"
)]
#[test_case(r#"{"is_stalled":false}"# => Torrent {
        is_stalled: Some(false),
        ..Default::default()
    } ; "semver 6.0.0 is stalled false"
)]

#[test_case(r#"{"labels":["foo"]}"# => Torrent {
        labels: Some(vec!["foo".into()]),
        ..Default::default()
    } ; "semver 6.0.0 labels single"
)]
#[test_case(r#"{"labels":["bar", "baz", "qux"]}"# => Torrent {
        labels: Some(vec![
            "bar".into(),
            "baz".into(),
            "qux".into(),
        ]),
        ..Default::default()
    } ; "semver 6.0.0 labels multiple"
)]
#[test_case(r#"{"labels":[]}"# => Torrent {
        labels: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 labels empty"
)]

#[test_case(r#"{"left_until_done":2138956824}"# => Torrent {
        left_until_done: Some(2138956824),
        ..Default::default()
    } ; "semver 6.0.0 left until done"
)]
#[test_case(r#"{"left_until_done":0}"# => Torrent {
        left_until_done: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 left until done zero"
)]
#[test_case(r#"{"left_until_done":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 left until done negative"
)]

#[test_case("{\
    \"magnet_link\":\"magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&\
        dn=archlinux-2024.08.01-x86_64.iso\"\
    }" => Torrent {
        magnet_link: Some(
            "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810\
            &dn=archlinux-2024.08.01-x86_64.iso".into()
        ),
        ..Default::default()
    } ; "semver 6.0.0 magnet link"
)]
#[test_case(r#"{"magnet_link":""}"# => Torrent {
        magnet_link: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 magnet link empty"
)]

#[test_case(r#"{"manual_announce_time":1723512975}"# => Torrent {
        manual_announce_time: DateTime::parse_from_rfc3339("2024-08-13T01:36:15Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 manual announce time"
)]
#[test_case(r#"{"manual_announce_time":-1}"# => Torrent {
        manual_announce_time: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 manual announce time negative"
)]
#[test_case(r#"{"manual_announce_time":0}"# => Torrent {
        manual_announce_time: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 manual announce time zero"
)]

#[test_case(r#"{"max_connected_peers":101}"# => Torrent {
        max_connected_peers: Some(101),
        ..Default::default()
    } ; "semver 6.0.0 max connected peers"
)]
#[test_case(r#"{"max_connected_peers":0}"# => Torrent {
        max_connected_peers: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 max connected peers zero"
)]
#[test_case(r#"{"max_connected_peers":-1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 max connected peers negative"
)]

#[test_case(r#"{"metadata_percent_complete":0.5284}"# => Torrent {
        metadata_percent_complete: Some(0.5284),
        ..Default::default()
    } ; "semver 6.0.0 metadata percent complete"
)]
#[test_case(r#"{"metadata_percent_complete":0}"# => Torrent {
        metadata_percent_complete: Some(0.),
        ..Default::default()
    } ; "semver 6.0.0 metadata percent complete zero"
)]
#[test_case(r#"{"metadata_percent_complete":1}"# => Torrent {
        metadata_percent_complete: Some(1.),
        ..Default::default()
    } ; "semver 6.0.0 metadata percent complete one"
)]
#[test_case(r#"{"metadata_percent_complete":-1}"# => Torrent {
        metadata_percent_complete: Some(-1.),
        ..Default::default()
    } ; "semver 6.0.0 metadata percent complete negative"
)]

#[test_case(r#"{"name":"debian-12.6.0-amd64-DVD-1.iso"}"# => Torrent {
        name: Some("debian-12.6.0-amd64-DVD-1.iso".into()),
        ..Default::default()
    } ; "semver 6.0.0 name"
)]
#[test_case(r#"{"name":""}"# => Torrent {
        name: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 name empty"
)]

#[test_case(r#"{"peer_limit":55}"# => Torrent {
        peer_limit: Some(55),
        ..Default::default()
    } ; "semver 6.0.0 peer limit"
)]
#[test_case(r#"{"peer_limit":0}"# => Torrent {
        peer_limit: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 peer limit zero"
)]
#[test_case(r#"{"peer_limit":-1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 peer limit negative"
)]

#[test_case(r#"{ "peers": [] }"# => Torrent {
        peers: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 peers empty"
)]
#[test_case(
    r#"{
        "peers": [
            {
                "address":"10.0.0.100",
                "client_is_choked":false,
                "client_is_interested":true,
                "client_name":"\u00b5Torrent 3.5.5",
                "flag_str":"dUEI",
                "is_downloading_from":false,
                "is_encrypted":true,
                "is_incoming":true,
                "is_uploading_to":true,
                "is_utp":false,
                "peer_is_choked":false,
                "peer_is_interested":true,
                "port":55555,
                "progress":0.2641,
                "rate_to_client":0,
                "rate_to_peer":385000
            }
        ]
    }"# => Torrent {
        peers: Some(vec![
            Peer {
                address: IpAddr::V4(Ipv4Addr::from_octets([10, 0, 0, 100])),
                bytes_to_client: 0,
                bytes_to_peer: 0,
                client_is_choked: false,
                client_is_interested: true,
                client_name: "\u{00b5}Torrent 3.5.5".into(),
                flag_str: "dUEI".into(),
                is_downloading_from: false,
                is_encrypted: true,
                is_incoming: true,
                is_uploading_to: true,
                is_utp: false,
                peer_id: "".into(),
                peer_is_choked: false,
                peer_is_interested: true,
                port: 55555,
                progress: 0.2641,
                rate_to_client: 0,
                rate_to_peer: 385000
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 peers single"
)]
#[test_case(
    r#"{
        "peers": [
            {
                "address":"10.0.0.100",
                "client_is_choked":false,
                "client_is_interested":true,
                "client_name":"\u00b5Torrent 3.5.5",
                "flag_str":"dUEI",
                "is_downloading_from":false,
                "is_encrypted":true,
                "is_incoming":true,
                "is_uploading_to":true,
                "is_utp":false,
                "peer_is_choked":false,
                "peer_is_interested":true,
                "port":55555,
                "progress":0.2641,
                "rate_to_client":0,
                "rate_to_peer":385000
            },
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
    }"# => Torrent {
        peers: Some(vec![
            Peer {
                address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 100)),
                bytes_to_client: 0,
                bytes_to_peer: 0,
                client_is_choked: false,
                client_is_interested: true,
                client_name: "\u{00b5}Torrent 3.5.5".into(),
                flag_str: "dUEI".into(),
                is_downloading_from: false,
                is_encrypted: true,
                is_incoming: true,
                is_uploading_to: true,
                is_utp: false,
                peer_id: "".into(),
                peer_is_choked: false,
                peer_is_interested: true,
                port: 55555,
                progress: 0.2641,
                rate_to_client: 0,
                rate_to_peer: 385000
            },
            Peer {
                address: IpAddr::V6(Ipv6Addr::new(8193, 3512, 34211, 0, 0, 35374, 880, 29492)),
                bytes_to_client: 0,
                bytes_to_peer: 0,
                client_is_choked: false,
                client_is_interested: true,
                client_name: "qBittorrent 4.6.5".into(),
                flag_str: "TDI".into(),
                is_downloading_from: true,
                is_encrypted: false,
                is_incoming: true,
                is_uploading_to: false,
                is_utp: true,
                peer_id: "".into(),
                peer_is_choked: true,
                peer_is_interested: false,
                port: 36667,
                progress: 1.,
                rate_to_client: 8000,
                rate_to_peer: 0
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 peers multiple"
)]

#[test_case(r#"{"peers_connected": 6}"# => Torrent {
        peers_connected: Some(6),
        ..Default::default()
    } ; "semver 6.0.0 peers connected"
)]
#[test_case(r#"{"peers_connected": 0}"# => Torrent {
        peers_connected: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 peers connected zero"
)]
#[test_case(r#"{"peers_connected": -1}"# => ignore["todo: u16"]
    panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 peers connected negative"
)]

#[test_case(
    r#"{
        "peers_from": {
            "from_cache":10,
            "from_dht":11,
            "from_incoming":12,
            "from_lpd":13,
            "from_ltep":14,
            "from_pex":15,
            "from_tracker":16
        }
    }"# => Torrent {
        peers_from: Some(PeersFrom {
            from_cache: 10,
            from_dht: 11,
            from_incoming: 12,
            from_lpd: 13,
            from_ltep: 14,
            from_pex: 15,
            from_tracker: 16,
        }),
        ..Default::default()
    } ; "semver 6.0.0 peers from"
)]

#[test_case(r#"{"peers_getting_from_us": 2}"# => Torrent {
        peers_getting_from_us: Some(2),
        ..Default::default()
    } ; "semver 6.0.0 peers getting from us"
)]
#[test_case(r#"{"peers_getting_from_us": 0}"# => Torrent {
        peers_getting_from_us: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 peers getting from us zero"
)]
#[test_case(r#"{"peers_getting_from_us": -1}"# => ignore["todo: u16"]
    panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 peers getting from us negative"
)]

#[test_case(r#"{"peers_sending_to_us": 2}"# => Torrent {
        peers_sending_to_us: Some(2),
        ..Default::default()
    } ; "semver 6.0.0 peers sending to us"
)]
#[test_case(r#"{"peers_sending_to_us": 0}"# => Torrent {
        peers_sending_to_us: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 peers sending to us zero"
)]
#[test_case(r#"{"peers_sending_to_us": -1}"# => ignore["todo: u16"]
    panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 peers sending to us negative"
)]

#[test_case(r#"{"percent_complete": 0.321}"# => Torrent {
        percent_complete: Some(0.321),
        ..Default::default()
    } ; "semver 6.0.0 percent complete"
)]
#[test_case(r#"{"percent_complete": 0}"# => Torrent {
        percent_complete: Some(0.),
        ..Default::default()
    } ; "semver 6.0.0 percent complete zero"
)]
#[test_case(r#"{"percent_complete": 1}"# => Torrent {
        percent_complete: Some(1.),
        ..Default::default()
    } ; "semver 6.0.0 percent complete one"
)]
#[test_case(r#"{"percent_complete": -1}"# => Torrent {
        percent_complete: Some(-1.),
        ..Default::default()
    } ; "semver 6.0.0 percent complete negative"
)]

#[test_case(r#"{"percent_done": 0.456}"# => Torrent {
        percent_done: Some(0.456),
        ..Default::default()
    } ; "semver 6.0.0 percent done"
)]
#[test_case(r#"{"percent_done": 0}"# => Torrent {
        percent_done: Some(0.),
        ..Default::default()
    } ; "semver 6.0.0 percent done zero"
)]
#[test_case(r#"{"percent_done": 1}"# => Torrent {
        percent_done: Some(1.),
        ..Default::default()
    } ; "semver 6.0.0 percent done one"
)]
#[test_case(r#"{"percent_done": -1}"# => Torrent {
        percent_done: Some(-1.),
        ..Default::default()
    } ; "semver 6.0.0 percent done negative"
)]

#[test_case(r#"{"pieces": "/Pb49/m+8tPzi+Z/e/39"}"# => Torrent {
        pieces: Some([
            0xFC, 0xF6, 0xF8, 0xF7, 0xF9, 0xBE, 0xF2, 0xD3, 0xF3, 0x8B, 0xE6, 0x7F, 0x7B, 0xFD,
            0xFD,
        ]
        .into()),
        ..Default::default()
    } ; "semver 6.0.0 pieces"
)]
#[test_case(
    "{\
        \"pieces\":\"//////////////////////////////////////////////////\
            ////////////////////////////////////////////////////////////////w=\"
    }" => Torrent {
        pieces: Some({
            let mut bits = vec![u8::MAX; 85];
            bits.push(0xFC);
            bits.into()
        }),
        ..Default::default()
    } ; "semver 6.0.0 pieces padded"
)]

#[test_case(r#"{"piece_count":45678}"# => Torrent {
        piece_count: Some(45678),
        ..Default::default()
    } ; "semver 6.0.0 piece count"
)]
#[test_case(r#"{"piece_count":0}"# => Torrent {
        piece_count: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 piece count zero"
)]
#[test_case(r#"{"piece_count":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 piece count negative"
)]

#[test_case(r#"{"piece_size":2097152}"# => Torrent {
        piece_size: Some(2097152),
        ..Default::default()
    } ; "semver 6.0.0 piece size"
)]
#[test_case(r#"{"piece_size":0}"# => Torrent {
        piece_size: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 piece size zero"
)]
#[test_case(r#"{"piece_size":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 piece size negative"
)]

#[test_case(r#"{"primary_mime_type": "application/octet-stream"}"# => Torrent {
        primary_mime_type: Some("application/octet-stream".into()),
        ..Default::default()
    } ; "semver 6.0.0 primary mime type"
)]
#[test_case(r#"{"primary_mime_type": ""}"# => Torrent {
        primary_mime_type: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 primary mime type empty"
)]

#[test_case(r#"{"priorities": [0,1,-1]}"# => Torrent {
        priorities: Some(vec![Priority::Normal, Priority::High, Priority::Low]),
        ..Default::default()
    } ; "semver 6.0.0 priorities"
)]
#[test_case(r#"{"priorities": []}"# => Torrent {
        priorities: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 priorities empty"
)]

#[test_case(r#"{"queue_position":321}"# => Torrent {
        queue_position: Some(321),
        ..Default::default()
    } ; "semver 6.0.0 queue position"
)]
#[test_case(r#"{"queue_position":0}"# => Torrent {
        queue_position: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 queue position zero"
)]
#[test_case(r#"{"queue_position":-1}"# => panics "invalid value: integer `-1`, expected usize"
    ; "semver 6.0.0 queue position negative"
)]

#[test_case(r#"{"rate_download":10000}"# => Torrent {
        rate_download: Some(10000),
        ..Default::default()
    } ; "semver 6.0.0 rate download"
)]
#[test_case(r#"{"rate_download":0}"# => Torrent {
        rate_download: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 rate download zero"
)]
#[test_case(r#"{"rate_download":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 rate download negative"
)]

#[test_case(r#"{"rate_upload":1200}"# => Torrent {
        rate_upload: Some(1200),
        ..Default::default()
    } ; "semver 6.0.0 rate upload"
)]
#[test_case(r#"{"rate_upload":0}"# => Torrent {
        rate_upload: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 rate upload zero"
)]
#[test_case(r#"{"rate_upload":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 rate upload negative"
)]

#[test_case(r#"{"recheck_progress": 0.871}"# => Torrent {
        recheck_progress: Some(0.871),
        ..Default::default()
    } ; "semver 6.0.0 recheck progress"
)]
#[test_case(r#"{"recheck_progress": 0}"# => Torrent {
        recheck_progress: Some(0.),
        ..Default::default()
    } ; "semver 6.0.0 recheck progress zero"
)]
#[test_case(r#"{"recheck_progress": 1}"# => Torrent {
        recheck_progress: Some(1.),
        ..Default::default()
    } ; "semver 6.0.0 recheck progress one"
)]
#[test_case(r#"{"recheck_progress": -1}"# => Torrent {
        recheck_progress: Some(-1.),
        ..Default::default()
    } ; "semver 6.0.0 recheck progress negative"
)]

#[test_case(r#"{"seconds_downloading":41744}"# => Torrent {
        seconds_downloading: Some(41744),
        ..Default::default()
    } ; "semver 6.0.0 seconds downloading"
)]
#[test_case(r#"{"seconds_downloading":0}"# => Torrent {
        seconds_downloading: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 seconds downloading zero"
)]
#[test_case(r#"{"seconds_downloading":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 seconds downloading negative"
)]

#[test_case(r#"{"seconds_seeding":13359445}"# => Torrent {
        seconds_seeding: Some(13359445),
        ..Default::default()
    } ; "semver 6.0.0 seconds seeding"
)]
#[test_case(r#"{"seconds_seeding":0}"# => Torrent {
        seconds_seeding: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 seconds seeding zero"
)]
#[test_case(r#"{"seconds_seeding":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 seconds seeding negative"
)]

#[test_case(r#"{"seed_idle_limit":30}"# => Torrent {
        seed_idle_limit: Some(30),
        ..Default::default()
    } ; "semver 6.0.0 seed idle limit"
)]
#[test_case(r#"{"seed_idle_limit":0}"# => Torrent {
        seed_idle_limit: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 seed idle limit zero"
)]
#[test_case(r#"{"seed_idle_limit":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 seed idle limit negative"
)]

#[test_case(r#"{"seed_idle_mode":0}"# => Torrent {
        seed_idle_mode: Some(IdleMode::Global),
        ..Default::default()
    } ; "semver 6.0.0 seed idle mode"
)]

#[test_case(r#"{"seed_ratio_limit": 3.14}"# => Torrent {
        seed_ratio_limit: Some(3.14),
        ..Default::default()
    } ; "semver 6.0.0 seed ratio limit"
)]
#[test_case(r#"{"seed_ratio_limit": 0}"# => Torrent {
        seed_ratio_limit: Some(0.),
        ..Default::default()
    } ; "semver 6.0.0 seed ratio limit zero"
)]
#[test_case(r#"{"seed_ratio_limit": 1}"# => Torrent {
        seed_ratio_limit: Some(1.),
        ..Default::default()
    } ; "semver 6.0.0 seed ratio limit one"
)]
#[test_case(r#"{"seed_ratio_limit": -1}"# => Torrent {
        seed_ratio_limit: Some(-1.),
        ..Default::default()
    } ; "semver 6.0.0 seed ratio limit negative"
)]

#[test_case(r#"{"seed_ratio_mode":2}"# => Torrent {
        seed_ratio_mode: Some(RatioMode::Unlimited),
        ..Default::default()
    } ; "semver 6.0.0 seed ratio mode"
)]

#[test_case(r#"{"sequential_download": true}"# => Torrent {
        sequential_download: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 sequential download"
)]

#[test_case(r#"{"sequential_download_from_piece": 9001}"# => Torrent {
        sequential_download_from_piece: Some(9001),
        ..Default::default()
    } ; "semver 6.0.0 sequential download from piece"
)]
#[test_case(r#"{"sequential_download_from_piece": 0}"# => Torrent {
        sequential_download_from_piece: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 sequential download from piece zero"
)]
#[test_case(r#"{"sequential_download_from_piece": -1}"#
    => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 sequential download from piece negative"
)]

#[test_case(r#"{"size_when_done":2965366874}"# => Torrent {
        size_when_done: Some(2965366874),
        ..Default::default()
    } ; "semver 6.0.0 size when done"
)]
#[test_case(r#"{"size_when_done":0}"# => Torrent {
        size_when_done: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 size when done zero"
)]
#[test_case(r#"{"size_when_done":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 size when done negative"
)]

#[test_case(r#"{"start_date":1774014859}"# => Torrent {
        start_date: DateTime::parse_from_rfc3339("2026-03-20 13:54:19+00:00")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "semver 6.0.0 start date"
)]
#[test_case(r#"{"start_date":-1}"# => Torrent {
        start_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 start date negative"
)]
#[test_case(r#"{"start_date":0}"# => Torrent {
        start_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "semver 6.0.0 start date zero"
)]

#[test_case(r#"{"status":4}"# => Torrent {
        status: Some(TorrentStatus::Downloading),
        ..Default::default()
    } ; "semver 6.0.0 status"
)]

#[test_case(
    r#"{
        "torrent_file": "/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent"
    }"# => Torrent {
        torrent_file: Some("/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent".into()),
        ..Default::default()
    } ; "semver 6.0.0 torrent file"
)]
#[test_case(r#"{"torrent_file":""}"# => Torrent {
        torrent_file: Some("".into()),
        ..Default::default()
    } ; "semver 6.0.0 torrent file empty"
)]

#[test_case(r#"{"total_size":2050306968}"# => Torrent {
        total_size: Some(2050306968),
        ..Default::default()
    } ; "semver 6.0.0 total size"
)]
#[test_case(r#"{"total_size":0}"# => Torrent {
        total_size: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 total size zero"
)]
#[test_case(r#"{"total_size":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 total size negative"
)]

#[test_case(r#"{
        "trackers": [
            {
                "announce":"https://example.com:4096/announce",
                "id":44023,
                "scrape":"https://example.com:4096/scrape",
                "sitename":"example",
                "tier":0
            }
        ]
    }"# => Torrent {
        trackers: Some(vec![
            Tracker {
                announce: Url::parse("https://example.com:4096/announce")
                    .expect("valid url"),
                id: 44023,
                scrape: Url::parse("https://example.com:4096/scrape")
                    .expect("valid url"),
                sitename: "example".into(),
                tier: 0,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 trackers single"
)]
#[test_case(r#"{
        "trackers": [
            {
                "announce":"https://example.com:1024/announce",
                "id":231,
                "scrape":"https://example.com:1024/scrape",
                "sitename":"example",
                "tier": 0
            },
            {
                "announce":"https://example.com:4096/announce",
                "id":235,
                "scrape":"https://example.com:4096/scrape",
                "sitename":"example",
                "tier": 1
            },
            {
                "announce":"https://example.com:32768/announce",
                "id":242,
                "scrape":"https://example.com:32768/scrape",
                "sitename":"example",
                "tier": 2
            }
        ]
    }"# => Torrent {
        trackers: Some(vec![
            Tracker {
                announce: Url::parse("https://example.com:1024/announce")
                    .expect("valid url"),
                id: 231,
                scrape: Url::parse("https://example.com:1024/scrape")
                    .expect("valid url"),
                sitename: "example".into(),
                tier: 0,
            },
            Tracker {
                announce: Url::parse("https://example.com:4096/announce")
                    .expect("valid url"),
                id: 235,
                scrape: Url::parse("https://example.com:4096/scrape")
                    .expect("valid url"),
                sitename: "example".into(),
                tier: 1,
            },
            Tracker {
                announce: Url::parse("https://example.com:32768/announce")
                    .expect("valid url"),
                id: 242,
                scrape: Url::parse("https://example.com:32768/scrape")
                    .expect("valid url"),
                sitename: "example".into(),
                tier: 2,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 trackers multiple"
)]
#[test_case(r#"{ "trackers": [] }"# => Torrent {
        trackers: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 trackers empty"
)]

#[test_case("{\
        \"tracker_list\":\"http://bt1.archive.org:6969/announce\\n\
        \\n\
        http://bt2.archive.org:6969/announce\\n\"\
    }" => Torrent {
        tracker_list: Some(vec![
            vec![Url::parse("http://bt1.archive.org:6969/announce").expect("valid url")],
            vec![Url::parse("http://bt2.archive.org:6969/announce").expect("valid url")],
        ].into()),
        ..Default::default()
    } ; "semver 6.0.0 tracker list"
)]

#[test_case(r#"{
        "tracker_stats":[
            {
                "announce":"https://example.com/announce",
                "announce_state":1,
                "download_count":245,
                "downloader_count": 123,
                "has_announced":true,
                "has_scraped":true,
                "host":"example.com:8080",
                "id":0,
                "is_backup":false,
                "last_announce_peer_count":86,
                "last_announce_result":"Success",
                "last_announce_start_time":1723614865,
                "last_announce_succeeded":true,
                "last_announce_time":1723614865,
                "last_announce_timed_out":false,
                "last_scrape_result":"Could not connect to tracker",
                "last_scrape_start_time":0,
                "last_scrape_succeeded":false,
                "last_scrape_time":1723614865,
                "last_scrape_timed_out":false,
                "leecher_count":9,
                "next_announce_time":1723618230,
                "next_scrape_time":0,
                "scrape_state":2,
                "scrape":"https://example.com:8080",
                "seeder_count":77,
                "tier":0
            }
        ]
    }"# => Torrent {
        tracker_stats: Some(vec![
            TrackerStat {
                announce: Url::parse("https://example.com/announce").expect("valid url"),
                announce_state: TrackerState::Waiting,
                download_count: 245,
                downloader_count: 123,
                has_announced: true,
                has_scraped: true,
                host: "example.com:8080".into(),
                id: 0,
                is_backup: false,
                last_announce_peer_count: 86,
                last_announce_result: "Success".into(),
                last_announce_start_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_announce_succeeded: true,
                last_announce_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_announce_timed_out: false,
                last_scrape_result: "Could not connect to tracker".into(),
                last_scrape_start_time: DateTime::UNIX_EPOCH,
                last_scrape_succeeded: false,
                last_scrape_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_scrape_timed_out: false,
                leecher_count: 9,
                next_announce_time: DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")
                    .expect("valid date time")
                    .to_utc(),
                next_scrape_time: DateTime::UNIX_EPOCH,
                scrape_state: TrackerState::Queued,
                scrape: Url::parse("https://example.com:8080").expect("valid url"),
                seeder_count: 77,
                sitename: "".into(),
                tier: 0
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 tracker stats single"
)]
#[test_case(r#"{
        "tracker_stats":[
            {
                "announce":"https://example.com/announce",
                "announce_state":1,
                "download_count":245,
                "downloader_count": 0,
                "has_announced":true,
                "has_scraped":true,
                "host":"example.com:8080",
                "id":0,
                "is_backup":false,
                "last_announce_peer_count":86,
                "last_announce_result":"Success",
                "last_announce_start_time":1723614865,
                "last_announce_succeeded":true,
                "last_announce_time":1723614865,
                "last_announce_timed_out":false,
                "last_scrape_result":"Could not connect to tracker",
                "last_scrape_start_time":0,
                "last_scrape_succeeded":false,
                "last_scrape_time":1723614865,
                "last_scrape_timed_out":false,
                "leecher_count":9,
                "next_announce_time":1723618230,
                "next_scrape_time":0,
                "scrape_state":2,
                "scrape":"https://example.com:8080",
                "seeder_count":77,
                "tier":0
            },
            {
                "announce":"http://example.org/foo/announce",
                "announceState":0,
                "download_count":24,
                "downloader_count": -1,
                "has_announced":false,
                "has_scraped":false,
                "host":"example.org:8080",
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
    }"# => Torrent {
        tracker_stats: Some(vec![
            TrackerStat {
                announce: Url::parse("https://example.com/announce").expect("valid url"),
                announce_state: TrackerState::Waiting,
                download_count: 245,
                downloader_count: 0,
                has_announced: true,
                has_scraped: true,
                host: "example.com:8080".into(),
                id: 0,
                is_backup: false,
                last_announce_peer_count: 86,
                last_announce_result: "Success".into(),
                last_announce_start_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_announce_succeeded: true,
                last_announce_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_announce_timed_out: false,
                last_scrape_result: "Could not connect to tracker".into(),
                last_scrape_start_time: DateTime::UNIX_EPOCH,
                last_scrape_succeeded: false,
                last_scrape_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_scrape_timed_out: false,
                leecher_count: 9,
                next_announce_time: DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")
                    .expect("valid date time")
                    .to_utc(),
                next_scrape_time: DateTime::UNIX_EPOCH,
                scrape_state: TrackerState::Queued,
                scrape: Url::parse("https://example.com:8080").expect("valid url"),
                seeder_count: 77,
                sitename: "".into(),
                tier: 0
            },
            TrackerStat {
                announce: Url::parse("http://example.org/foo/announce").expect("valid url"),
                announce_state: TrackerState::Inactive,
                download_count: 24,
                downloader_count: -1,
                has_announced: false,
                has_scraped: false,
                host: "example.org:8080".into(),
                id: 666,
                is_backup: true,
                last_announce_peer_count: 9999,
                last_announce_result: "IPv4 connection failed".into(),
                last_announce_start_time: DateTime::UNIX_EPOCH,
                last_announce_succeeded: false,
                last_announce_time: DateTime::parse_from_rfc3339("2026-03-20 06:54:19+00:00")
                    .expect("valid date time")
                    .to_utc(),
                last_announce_timed_out: false,
                last_scrape_result: "Could not connect to tracker".into(),
                last_scrape_start_time: DateTime::UNIX_EPOCH,
                last_scrape_succeeded: false,
                last_scrape_time: DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")
                    .expect("valid date time")
                    .to_utc(),
                last_scrape_timed_out: false,
                leecher_count: 2,
                next_announce_time: DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")
                    .expect("valid date time")
                    .to_utc(),
                next_scrape_time: DateTime::UNIX_EPOCH,
                scrape_state: TrackerState::Inactive,
                scrape: Url::parse("http://example.org/scrape").expect("valid url"),
                seeder_count: 5,
                sitename: "example".into(),
                tier: 1
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 tracker stats multiple"
)]
#[test_case(r#"{ "tracker_stats":[] }"# => Torrent {
        tracker_stats: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 tracker stats empty"
)]

#[test_case(r#"{"uploaded_ever":1301396208}"# => Torrent {
        uploaded_ever: Some(1301396208),
        ..Default::default()
    } ; "semver 6.0.0 uploaded ever"
)]
#[test_case(r#"{"uploaded_ever":0}"# => Torrent {
        uploaded_ever: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 uploaded ever zero"
)]
#[test_case(r#"{"uploaded_ever":-1}"# => ignore["todo: u64"]
    panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 uploaded ever negative"
)]

#[test_case(r#"{"upload_limit":1024}"# => Torrent {
        upload_limit: Some(1024),
        ..Default::default()
    } ; "semver 6.0.0 uploaded limit"
)]
#[test_case(r#"{"upload_limit":0}"# => Torrent {
        upload_limit: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 uploaded limit zero"
)]
#[test_case(r#"{"upload_limit":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 uploaded limit negative"
)]

#[test_case(r#"{"upload_limited":true}"# => Torrent {
        upload_limited: Some(true),
        ..Default::default()
    } ; "semver 6.0.0 uploaded limited"
)]

#[test_case(r#"{"upload_ratio": 1.23}"# => Torrent {
        upload_ratio: Some(1.23),
        ..Default::default()
    } ; "semver 6.0.0 upload ratio"
)]
#[test_case(r#"{"upload_ratio": 0}"# => Torrent {
        upload_ratio: Some(0.),
        ..Default::default()
    } ; "semver 6.0.0 upload ratio zero"
)]
#[test_case(r#"{"upload_ratio": 1}"# => Torrent {
        upload_ratio: Some(1.),
        ..Default::default()
    } ; "semver 6.0.0 upload ratio one"
)]
#[test_case(r#"{"upload_ratio": -1}"# => Torrent {
        upload_ratio: Some(-1.),
        ..Default::default()
    } ; "semver 6.0.0 upload ratio negative"
)]

#[test_case(r#"{"wanted":[0, 1, 0, 0, 1]}"# => Torrent {
        wanted: Some(vec![false, true, false, false, true]),
        ..Default::default()
    } ; "semver 6.0.0 wanted ints"
)]
#[test_case(r#"{"wanted":[false, true, false, false, true]}"# => Torrent {
        wanted: Some(vec![false, true, false, false, true]),
        ..Default::default()
    } ; "semver 6.0.0 wanted bools"
)]
#[test_case(r#"{"wanted":[]}"# => Torrent {
        wanted: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 wanted empty"
)]
#[test_case(r#"{"wanted":[-1]}"# => panics "failed to deserialize torrent: unexpected number: -1"
    ; "semver 6.0.0 wanted invalid negative"
)]
#[test_case(r#"{"wanted":[2]}"# => panics "failed to deserialize torrent: unexpected number: 2"
    ; "semver 6.0.0 wanted invalid positive"
)]
#[test_case(r#"{"wanted":["foo"]}"# => panics "failed to deserialize torrent: unexpected type"
    ; "semver 6.0.0 wanted invalid type"
)]

#[test_case(r#"{
        "webseeds": [
            "https://cdimage.debian.org/debian-cd/",
            "https://dl.example.com/foo/",
            "https://bar.example.com/"
        ]
    }"# => Torrent {
        webseeds: Some(vec![
            "https://cdimage.debian.org/debian-cd/".into(),
            "https://dl.example.com/foo/".into(),
            "https://bar.example.com/".into(),
        ]),
        ..Default::default()
    } ; "semver 6.0.0 webseeds"
)]
#[test_case(r#"{ "webseeds": [] }"# => Torrent {
        webseeds: Some(vec![]),
        ..Default::default()
    } ; "semver 6.0.0 webseeds empty"
)]
#[test_case(r#"{"webseeds":["malformed"]}"# => ignore["todo: Url"]
    panics "relative URL without a base"
    ; "semver 6.0.0 webseeds malformed"
)]

#[test_case(r#"{
        "webseeds_ex": [
            {
                "url": "https://cdimage.debian.org/debian-cd/",
                "is_downloading": true,
                "download_bytes_per_second": 0
            }
        ]
    }"# => ignore["todo: confirm actual response data"]
    Torrent {
        webseeds_ex: Some(vec![
            WebseedsEx {
                url: "https://cdimage.debian.org/debian-cd/".into(),
                is_downloading: true,
                download_bytes_per_second: 0,
            },
        ]),
        ..Default::default()
    } ; "semver 6.0.0 webseeds ex"
)]

#[test_case(r#"{"webseeds_sending_to_us":1234}"# => Torrent {
        webseeds_sending_to_us: Some(1234),
        ..Default::default()
    } ; "semver 6.0.0 webseeds sending to us"
)]
#[test_case(r#"{"webseeds_sending_to_us":0}"# => Torrent {
        webseeds_sending_to_us: Some(0),
        ..Default::default()
    } ; "semver 6.0.0 webseeds sending to us zero"
)]
#[test_case(r#"{"webseeds_sending_to_us":-1}"#
    => panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 webseeds sending to us negative"
)]

fn torrent_deserialize(tor_data: &str) -> Torrent {
    let mut resp = match torrents_json([tor_data]) {
        Ok(resp) => resp,
        Err(err) => panic!("failed to deserialize torrent: {err}"),
    };

    println!("{resp:#?}");
    assert!(resp.is_ok());
    resp.arguments.torrents
        .pop() // NOTE: This does not test multiple torrent responses.
        .expect("deserialized response should have a single torrent")
}
