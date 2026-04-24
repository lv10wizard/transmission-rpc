//! This file defines legacy (pre- semver-6.0.0) [`Torrent`] deserialization tests.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use chrono::DateTime;
use serde_json::Result as SerdeResult;
use test_case::test_case;
use url::Url;

use crate::types::{
    ErrorType, File, FileStat, IdleMode, Peer, PeersFrom, Priority, RatioMode, Result, RpcResponse,
    Torrent, Torrents, TorrentStatus, Tracker, TrackerStat, TrackerState,
};

/// Deserializes any [`IntoIterator`] (eg. `Vec<_>` or `[_]`) of torrent response data from a
/// legacy response formatted json string.
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
    serde_json::from_str(&format!("{{\
        \"arguments\":{{\
            \"torrents\":[\
                {data}\
            ]\
        }},\
        \"result\":\"success\"\
    }}"))
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

#[test_case(r#"{"activityDate":1718947434}"# => Torrent {
        activity_date: DateTime::parse_from_rfc3339("2024-06-21T05:23:54Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy activity date"
)]
#[test_case(r#"{"activityDate":-1}"# => Torrent {
        activity_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy activity date negative"
)]
#[test_case(r#"{"activityDate":0}"# => Torrent {
        activity_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy activity date zero"
)]

#[test_case(r#"{"addedDate":1670612948}"# => Torrent {
        added_date: DateTime::parse_from_rfc3339("2022-12-09T19:09:08Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy added date"
)]
#[test_case(r#"{"addedDate":-1}"# => Torrent {
        added_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy added date negative"
)]
#[test_case(r#"{"addedDate":0}"# => Torrent {
        added_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy added date zero"
)]

#[test_case(r#"{"availability":[-1,0,1,2,3,10,-1]}"# => Torrent {
        availability: Some(vec![-1,0,1,2,3,10,-1]),
        ..Default::default()
    } ; "legacy availability"
)]
#[test_case(r#"{"availability":[]}"# => Torrent {
        availability: Some(vec![]),
        ..Default::default()
    } ; "legacy availability empty"
)]

#[test_case(r#"{"bandwidthPriority":0}"# => Torrent {
        bandwidth_priority: Some(Priority::Normal),
        ..Default::default()
    } ; "legacy bandwidth priority normal"
)]

#[test_case(r#"{"bytesCompleted":[]}"# => Torrent {
        bytes_completed: Some(vec![]),
        ..Default::default()
    } ; "legacy bytes completed empty"
)]
#[test_case(r#"{"bytesCompleted":[790626304]}"# => Torrent {
        bytes_completed: Some(vec![790626304]),
        ..Default::default()
    } ; "legacy bytes completed single"
)]
#[test_case(r#"{"bytesCompleted":[1234,567890,443,8080]}"# => Torrent {
        bytes_completed: Some(vec![1234,567890,443,8080]),
        ..Default::default()
    } ; "legacy bytes completed multiple"
)]
#[test_case(r#"{"bytesCompleted":[0]}"# => Torrent {
        bytes_completed: Some(vec![0]),
        ..Default::default()
    } ; "legacy bytes completed zero"
)]
#[test_case(r#"{"bytesCompleted":[-1]}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy bytes completed negative"
)]

#[test_case(r#"{"comment":"lorem ipsum"}"# => Torrent {
        comment: Some("lorem ipsum".into()),
        ..Default::default()
    } ; "legacy comment"
)]
#[test_case(r#"{"comment":""}"# => Torrent {
        comment: Some("".into()),
        ..Default::default()
    } ; "legacy comment empty"
)]

#[test_case(r#"{"corruptEver":4096}"# => Torrent {
        corrupt_ever: Some(4096),
        ..Default::default()
    } ; "legacy corrupt ever"
)]
#[test_case(r#"{"corruptEver":0}"# => Torrent {
        corrupt_ever: Some(0),
        ..Default::default()
    } ; "legacy corrupt ever zero"
)]
#[test_case(r#"{"corruptEver":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy corrupt ever negative"
)]

#[test_case(r#"{"creator":"mktorrent 1.1"}"# => Torrent {
        creator: Some("mktorrent 1.1".into()),
        ..Default::default()
    } ; "legacy creator"
)]
#[test_case(r#"{"creator":""}"# => Torrent {
        creator: Some("".into()),
        ..Default::default()
    } ; "legacy creator empty"
)]

#[test_case(r#"{"dateCreated":1592962706}"# => Torrent {
        date_created: DateTime::parse_from_rfc3339("2020-06-24T01:38:26Z")
            .ok()
            .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy date created"
)]
#[test_case(r#"{"dateCreated":0}"# => Torrent {
        date_created: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy date created zero"
)]
#[test_case(r#"{"dateCreated":-1}"# => Torrent {
        date_created: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy date created negative"
)]

#[test_case(r#"{"desiredAvailable":20162576}"# => Torrent {
        desired_available: Some(20162576),
        ..Default::default()
    } ; "legacy desired available"
)]
#[test_case(r#"{"desiredAvailable":0}"# => Torrent {
        desired_available: Some(0),
        ..Default::default()
    } ; "legacy desired available zero"
)]
#[test_case(r#"{"desiredAvailable":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy desired available negative"
)]

#[test_case(r#"{"doneDate":1672060369}"# => Torrent {
        done_date: DateTime::parse_from_rfc3339("2022-12-26T13:12:49Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy done date"
)]
#[test_case(r#"{"doneDate":-1}"# => Torrent {
        done_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy done date negative"
)]
#[test_case(r#"{"doneDate":0}"# => Torrent {
        done_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy done date zero"
)]

#[test_case(r#"{"downloadDir":"/downloads/iso/"}"# => Torrent {
        download_dir: Some("/downloads/iso/".into()),
        ..Default::default()
    } ; "legacy download dir"
)]
#[test_case(r#"{"downloadDir":""}"# => Torrent {
        download_dir: Some("".into()),
        ..Default::default()
    } ; "legacy download dir empty"
)]

#[test_case(r#"{"downloadedEver":1340189370}"# => Torrent {
        downloaded_ever: Some(1340189370),
        ..Default::default()
    } ; "legacy downloaded ever"
)]
#[test_case(r#"{"downloadedEver":0}"# => Torrent {
        downloaded_ever: Some(0),
        ..Default::default()
    } ; "legacy downloaded ever zero"
)]
#[test_case(r#"{"downloadedEver":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy downloaded ever negative"
)]

#[test_case(r#"{"downloadLimit":2048}"# => Torrent {
        download_limit: Some(2048),
        ..Default::default()
    } ; "legacy downloaded limit"
)]
#[test_case(r#"{"downloadLimit":0}"# => Torrent {
        download_limit: Some(0),
        ..Default::default()
    } ; "legacy downloaded limit zero"
)]
#[test_case(r#"{"downloadLimit":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy downloaded limit negative"
)]

#[test_case(r#"{"downloadLimited":true}"# => Torrent {
        download_limited: Some(true),
        ..Default::default()
    } ; "legacy downloaded limit true"
)]
#[test_case(r#"{"downloadLimited":false}"# => Torrent {
        download_limited: Some(false),
        ..Default::default()
    } ; "legacy downloaded limit false"
)]

#[test_case(r#"{"editDate":1723512675}"# => Torrent {
        edit_date: DateTime::parse_from_rfc3339("2024-08-13T01:31:15Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy edit date"
)]
#[test_case(r#"{"editDate":0}"# => Torrent {
        edit_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy edit date zero"
)]
#[test_case(r#"{"editDate":-1}"# => Torrent {
        edit_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy edit date negative"
)]

#[test_case(r#"{"error":0}"# => Torrent {
        error: Some(ErrorType::Ok),
        ..Default::default()
    } ; "legacy error ok"
)]

#[test_case(r#"{"errorString":"Unregistered torrent"}"# => Torrent {
        error_string: Some("Unregistered torrent".into()),
        ..Default::default()
    } ; "legacy error string unregistered torrent"
)]
#[test_case(r#"{"errorString":""}"# => Torrent {
        error_string: Some("".into()),
        ..Default::default()
    } ; "legacy error string empty"
)]

#[test_case(r#"{"eta":82112}"# => Torrent {
        eta: Some(82112),
        ..Default::default()
    } ; "legacy eta"
)]
#[test_case(r#"{"eta":0}"# => Torrent {
        eta: Some(0),
        ..Default::default()
    } ; "legacy eta zero"
)]
#[test_case(r#"{"eta":-1}"# => Torrent {
        eta: Some(-1),
        ..Default::default()
    } ; "legacy eta negative"
)]

#[test_case(r#"{"etaIdle":1234}"# => Torrent {
        eta_idle: Some(1234),
        ..Default::default()
    } ; "legacy eta idle"
)]
#[test_case(r#"{"etaIdle":0}"# => Torrent {
        eta_idle: Some(0),
        ..Default::default()
    } ; "legacy eta idle zero"
)]
#[test_case(r#"{"etaIdle":-1}"# => Torrent {
        eta_idle: Some(-1),
        ..Default::default()
    } ; "legacy eta idle negative"
)]

#[test_case(r#"{"file-count":420}"# => Torrent {
        file_count: Some(420),
        ..Default::default()
    } ; "legacy file count"
)]
#[test_case(r#"{"file-count":0}"# => Torrent {
        file_count: Some(0),
        ..Default::default()
    } ; "legacy file count zero"
)]
#[test_case(r#"{"file-count":-1}"# => panics "invalid value: integer `-1`, expected usize"
    ; "legacy file count negative"
)]

#[test_case(r#"{ "files": [] }"#
    => Torrent {
        files: Some(vec![]),
        ..Default::default()
    } ; "legacy files empty"
)]
#[test_case(
    r#"{
        "files": [
            {
                "bytesCompleted":172415250,
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
    } ; "legacy files single"
)]
#[test_case(
    r#"{
        "files": [
            {
                "bytesCompleted":172415250,
                "length":3994091520,
                "name":"debian-12.6.0-amd64-DVD-1.iso"
            },
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
    } ; "legacy files multiple"
)]

#[test_case(r#"{ "fileStats": [] }"#
    => Torrent {
        file_stats: Some(vec![]),
        ..Default::default()
    } ; "legacy file stats empty"
)]
#[test_case(
    r#"{
        "fileStats": [
            {
                "bytesCompleted": 0,
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
    } ; "legacy file stats single"
)]
#[test_case(
    r#"{
        "fileStats": [
            {
                "bytesCompleted": 1,
                "priority": -1,
                "wanted": false
            },
            {
                "bytesCompleted": 2,
                "priority": 0,
                "wanted": true
            },
            {
                "bytesCompleted": 300,
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
    } ; "legacy file stats multiple"
)]

#[test_case(r#"{"group":"foo bar"}"# => Torrent {
        group: Some("foo bar".into()),
        ..Default::default()
    } ; "legacy group"
)]
#[test_case(r#"{"group":""}"# => Torrent {
        group: Some("".into()),
        ..Default::default()
    } ; "legacy group empty"
)]

#[test_case(r#"{"hashString":"e08c426aab2cc58649ae5e73690e3747117b3470"}"# => Torrent {
        hash_string: Some("e08c426aab2cc58649ae5e73690e3747117b3470".into()),
        ..Default::default()
    } ; "legacy hash string"
)]
#[test_case(r#"{"hashString":""}"# => Torrent {
        hash_string: Some("".into()),
        ..Default::default()
    } ; "legacy hash string empty"
)]

#[test_case(r#"{"haveUnchecked":39813}"# => Torrent {
        have_unchecked: Some(39813),
        ..Default::default()
    } ; "legacy have unchecked"
)]
#[test_case(r#"{"haveUnchecked":0}"# => Torrent {
        have_unchecked: Some(0),
        ..Default::default()
    } ; "legacy have unchecked zero"
)]
#[test_case(r#"{"haveUnchecked":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy have unchecked negative"
)]

#[test_case(r#"{"haveValid":39813}"# => Torrent {
        have_valid: Some(39813),
        ..Default::default()
    } ; "legacy have valid"
)]
#[test_case(r#"{"haveValid":0}"# => Torrent {
        have_valid: Some(0),
        ..Default::default()
    } ; "legacy have valid zero"
)]
#[test_case(r#"{"haveValid":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy have valid negative"
)]

#[test_case(r#"{"honorsSessionLimits":true}"# => Torrent {
        honors_session_limits: Some(true),
        ..Default::default()
    } ; "legacy honors session limits true"
)]
#[test_case(r#"{"honorsSessionLimits":false}"# => Torrent {
        honors_session_limits: Some(false),
        ..Default::default()
    } ; "legacy honors session limits false"
)]

#[test_case(r#"{"id":1}"# => Torrent {
        id: Some(1),
        ..Default::default()
    } ; "legacy id"
)]
#[test_case(r#"{"id":0}"# => Torrent {
        id: Some(0),
        ..Default::default()
    } ; "legacy id zero"
)]
#[test_case(r#"{"id":-1}"# => Torrent {
        id: Some(-1),
        ..Default::default()
    } ; "legacy id negative"
)]

#[test_case(r#"{"isFinished":true}"# => Torrent {
        is_finished: Some(true),
        ..Default::default()
    } ; "legacy is finished true"
)]
#[test_case(r#"{"isFinished":false}"# => Torrent {
        is_finished: Some(false),
        ..Default::default()
    } ; "legacy is finished false"
)]

#[test_case(r#"{"isPrivate":true}"# => Torrent {
        is_private: Some(true),
        ..Default::default()
    } ; "legacy is private true"
)]
#[test_case(r#"{"isPrivate":false}"# => Torrent {
        is_private: Some(false),
        ..Default::default()
    } ; "legacy is private false"
)]

#[test_case(r#"{"isStalled":true}"# => Torrent {
        is_stalled: Some(true),
        ..Default::default()
    } ; "legacy is stalled true"
)]
#[test_case(r#"{"isStalled":false}"# => Torrent {
        is_stalled: Some(false),
        ..Default::default()
    } ; "legacy is stalled false"
)]

#[test_case(r#"{"labels":["foo"]}"# => Torrent {
        labels: Some(vec!["foo".into()]),
        ..Default::default()
    } ; "legacy labels single"
)]
#[test_case(r#"{"labels":["bar", "baz", "qux"]}"# => Torrent {
        labels: Some(vec![
            "bar".into(),
            "baz".into(),
            "qux".into(),
        ]),
        ..Default::default()
    } ; "legacy labels multiple"
)]
#[test_case(r#"{"labels":[]}"# => Torrent {
        labels: Some(vec![]),
        ..Default::default()
    } ; "legacy labels empty"
)]

#[test_case(r#"{"leftUntilDone":2138956824}"# => Torrent {
        left_until_done: Some(2138956824),
        ..Default::default()
    } ; "legacy left until done"
)]
#[test_case(r#"{"leftUntilDone":0}"# => Torrent {
        left_until_done: Some(0),
        ..Default::default()
    } ; "legacy left until done zero"
)]
#[test_case(r#"{"leftUntilDone":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy left until done negative"
)]

#[test_case("{\
    \"magnetLink\":\"magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&\
        dn=archlinux-2024.08.01-x86_64.iso\"\
    }" => Torrent {
        magnet_link: Some(
            "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810\
            &dn=archlinux-2024.08.01-x86_64.iso".into()
        ),
        ..Default::default()
    } ; "legacy magnet link"
)]
#[test_case(r#"{"magnetLink":""}"# => Torrent {
        magnet_link: Some("".into()),
        ..Default::default()
    } ; "legacy magnet link empty"
)]

#[test_case(r#"{"manualAnnounceTime":1723512975}"# => Torrent {
        manual_announce_time: DateTime::parse_from_rfc3339("2024-08-13T01:36:15Z")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy manual announce time"
)]
#[test_case(r#"{"manualAnnounceTime":-1}"# => Torrent {
        manual_announce_time: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy manual announce time negative"
)]
#[test_case(r#"{"manualAnnounceTime":0}"# => Torrent {
        manual_announce_time: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy manual announce time zero"
)]

#[test_case(r#"{"maxConnectedPeers":101}"# => Torrent {
        max_connected_peers: Some(101),
        ..Default::default()
    } ; "legacy max connected peers"
)]
#[test_case(r#"{"maxConnectedPeers":0}"# => Torrent {
        max_connected_peers: Some(0),
        ..Default::default()
    } ; "legacy max connected peers zero"
)]
#[test_case(r#"{"maxConnectedPeers":-1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy max connected peers negative"
)]

#[test_case(r#"{"metadataPercentComplete":0.5284}"# => Torrent {
        metadata_percent_complete: Some(0.5284),
        ..Default::default()
    } ; "legacy metadata percent complete"
)]
#[test_case(r#"{"metadataPercentComplete":0}"# => Torrent {
        metadata_percent_complete: Some(0.),
        ..Default::default()
    } ; "legacy metadata percent complete zero"
)]
#[test_case(r#"{"metadataPercentComplete":1}"# => Torrent {
        metadata_percent_complete: Some(1.),
        ..Default::default()
    } ; "legacy metadata percent complete one"
)]
#[test_case(r#"{"metadataPercentComplete":-1}"# => Torrent {
        metadata_percent_complete: Some(-1.),
        ..Default::default()
    } ; "legacy metadata percent complete negative"
)]

#[test_case(r#"{"name":"debian-12.6.0-amd64-DVD-1.iso"}"# => Torrent {
        name: Some("debian-12.6.0-amd64-DVD-1.iso".into()),
        ..Default::default()
    } ; "legacy name"
)]
#[test_case(r#"{"name":""}"# => Torrent {
        name: Some("".into()),
        ..Default::default()
    } ; "legacy name empty"
)]

#[test_case(r#"{"peer-limit":55}"# => Torrent {
        peer_limit: Some(55),
        ..Default::default()
    } ; "legacy peer limit"
)]
#[test_case(r#"{"peer-limit":0}"# => Torrent {
        peer_limit: Some(0),
        ..Default::default()
    } ; "legacy peer limit zero"
)]
#[test_case(r#"{"peer-limit":-1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy peer limit negative"
)]

#[test_case(r#"{ "peers": [] }"# => Torrent {
        peers: Some(vec![]),
        ..Default::default()
    } ; "legacy peers empty"
)]
#[test_case(
    r#"{
        "peers": [
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
    } ; "legacy peers single"
)]
#[test_case(
    r#"{
        "peers": [
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
                "clientIsChoked":false,
                "clientIsInterested":true,
                "clientName":"qBittorrent 4.6.5",
                "flagStr":"TDI",
                "isDownloadingFrom":true,
                "isEncrypted":false,
                "isIncoming":true,
                "isUploadingTo":false,
                "isUtp":true,
                "peerIsChoked":true,
                "peerIsInterested":false,
                "port":36667,
                "progress":1,
                "rateToClient":8000,
                "rateToPeer":0
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
    } ; "legacy peers multiple"
)]

#[test_case(r#"{"peersConnected": 6}"# => Torrent {
        peers_connected: Some(6),
        ..Default::default()
    } ; "legacy peers connected"
)]
#[test_case(r#"{"peersConnected": 0}"# => Torrent {
        peers_connected: Some(0),
        ..Default::default()
    } ; "legacy peers connected zero"
)]
#[test_case(r#"{"peersConnected": -1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy peers connected negative"
)]

#[test_case(
    r#"{
        "peersFrom": {
            "fromCache":10,
            "fromDht":11,
            "fromIncoming":12,
            "fromLpd":13,
            "fromLtep":14,
            "fromPex":15,
            "fromTracker":16
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
    } ; "legacy peers from"
)]

#[test_case(r#"{"peersGettingFromUs": 2}"# => Torrent {
        peers_getting_from_us: Some(2),
        ..Default::default()
    } ; "legacy peers getting from us"
)]
#[test_case(r#"{"peersGettingFromUs": 0}"# => Torrent {
        peers_getting_from_us: Some(0),
        ..Default::default()
    } ; "legacy peers getting from us zero"
)]
#[test_case(r#"{"peersGettingFromUs": -1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy peers getting from us negative"
)]

#[test_case(r#"{"peersSendingToUs": 2}"# => Torrent {
        peers_sending_to_us: Some(2),
        ..Default::default()
    } ; "legacy peers sending to us"
)]
#[test_case(r#"{"peersSendingToUs": 0}"# => Torrent {
        peers_sending_to_us: Some(0),
        ..Default::default()
    } ; "legacy peers sending to us zero"
)]
#[test_case(r#"{"peersSendingToUs": -1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy peers sending to us negative"
)]

#[test_case(r#"{"percentComplete": 0.321}"# => Torrent {
        percent_complete: Some(0.321),
        ..Default::default()
    } ; "legacy percent complete"
)]
#[test_case(r#"{"percentComplete": 0}"# => Torrent {
        percent_complete: Some(0.),
        ..Default::default()
    } ; "legacy percent complete zero"
)]
#[test_case(r#"{"percentComplete": 1}"# => Torrent {
        percent_complete: Some(1.),
        ..Default::default()
    } ; "legacy percent complete one"
)]
#[test_case(r#"{"percentComplete": -1}"# => Torrent {
        percent_complete: Some(-1.),
        ..Default::default()
    } ; "legacy percent complete negative"
)]

#[test_case(r#"{"percentDone": 0.456}"# => Torrent {
        percent_done: Some(0.456),
        ..Default::default()
    } ; "legacy percent done"
)]
#[test_case(r#"{"percentDone": 0}"# => Torrent {
        percent_done: Some(0.),
        ..Default::default()
    } ; "legacy percent done zero"
)]
#[test_case(r#"{"percentDone": 1}"# => Torrent {
        percent_done: Some(1.),
        ..Default::default()
    } ; "legacy percent done one"
)]
#[test_case(r#"{"percentDone": -1}"# => Torrent {
        percent_done: Some(-1.),
        ..Default::default()
    } ; "legacy percent done negative"
)]

#[test_case(r#"{"pieces": "/Pb49/m+8tPzi+Z/e/39"}"# => Torrent {
        pieces: Some([
            0xFC, 0xF6, 0xF8, 0xF7, 0xF9, 0xBE, 0xF2, 0xD3, 0xF3, 0x8B, 0xE6, 0x7F, 0x7B, 0xFD,
            0xFD,
        ]
        .into()),
        ..Default::default()
    } ; "legacy pieces"
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
    } ; "legacy pieces padded"
)]

#[test_case(r#"{"pieceCount":45678}"# => Torrent {
        piece_count: Some(45678),
        ..Default::default()
    } ; "legacy piece count"
)]
#[test_case(r#"{"pieceCount":0}"# => Torrent {
        piece_count: Some(0),
        ..Default::default()
    } ; "legacy piece count zero"
)]
#[test_case(r#"{"pieceCount":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy piece count negative"
)]

#[test_case(r#"{"pieceSize":2097152}"# => Torrent {
        piece_size: Some(2097152),
        ..Default::default()
    } ; "legacy piece size"
)]
#[test_case(r#"{"pieceSize":0}"# => Torrent {
        piece_size: Some(0),
        ..Default::default()
    } ; "legacy piece size zero"
)]
#[test_case(r#"{"pieceSize":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy piece size negative"
)]

#[test_case(r#"{"primary-mime-type": "application/octet-stream"}"# => Torrent {
        primary_mime_type: Some("application/octet-stream".into()),
        ..Default::default()
    } ; "legacy primary mime type"
)]
#[test_case(r#"{"primary-mime-type": ""}"# => Torrent {
        primary_mime_type: Some("".into()),
        ..Default::default()
    } ; "legacy primary mime type empty"
)]

#[test_case(r#"{"priorities": [0,1,-1]}"# => Torrent {
        priorities: Some(vec![Priority::Normal, Priority::High, Priority::Low]),
        ..Default::default()
    } ; "legacy priorities"
)]
#[test_case(r#"{"priorities": []}"# => Torrent {
        priorities: Some(vec![]),
        ..Default::default()
    } ; "legacy priorities empty"
)]

#[test_case(r#"{"queuePosition":321}"# => Torrent {
        queue_position: Some(321),
        ..Default::default()
    } ; "legacy queue position"
)]
#[test_case(r#"{"queuePosition":0}"# => Torrent {
        queue_position: Some(0),
        ..Default::default()
    } ; "legacy queue position zero"
)]
#[test_case(r#"{"queuePosition":-1}"# => panics "invalid value: integer `-1`, expected usize"
    ; "legacy queue position negative"
)]

#[test_case(r#"{"rateDownload":10000}"# => Torrent {
        rate_download: Some(10000),
        ..Default::default()
    } ; "legacy rate download"
)]
#[test_case(r#"{"rateDownload":0}"# => Torrent {
        rate_download: Some(0),
        ..Default::default()
    } ; "legacy rate download zero"
)]
#[test_case(r#"{"rateDownload":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy rate download negative"
)]

#[test_case(r#"{"rateUpload":1200}"# => Torrent {
        rate_upload: Some(1200),
        ..Default::default()
    } ; "legacy rate upload"
)]
#[test_case(r#"{"rateUpload":0}"# => Torrent {
        rate_upload: Some(0),
        ..Default::default()
    } ; "legacy rate upload zero"
)]
#[test_case(r#"{"rateUpload":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy rate upload negative"
)]

#[test_case(r#"{"recheckProgress": 0.871}"# => Torrent {
        recheck_progress: Some(0.871),
        ..Default::default()
    } ; "legacy recheck progress"
)]
#[test_case(r#"{"recheckProgress": 0}"# => Torrent {
        recheck_progress: Some(0.),
        ..Default::default()
    } ; "legacy recheck progress zero"
)]
#[test_case(r#"{"recheckProgress": 1}"# => Torrent {
        recheck_progress: Some(1.),
        ..Default::default()
    } ; "legacy recheck progress one"
)]
#[test_case(r#"{"recheckProgress": -1}"# => Torrent {
        recheck_progress: Some(-1.),
        ..Default::default()
    } ; "legacy recheck progress negative"
)]

#[test_case(r#"{"secondsDownloading":41744}"# => Torrent {
        seconds_downloading: Some(41744),
        ..Default::default()
    } ; "legacy seconds downloading"
)]
#[test_case(r#"{"secondsDownloading":0}"# => Torrent {
        seconds_downloading: Some(0),
        ..Default::default()
    } ; "legacy seconds downloading zero"
)]
#[test_case(r#"{"secondsDownloading":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy seconds downloading negative"
)]

#[test_case(r#"{"secondsSeeding":13359445}"# => Torrent {
        seconds_seeding: Some(13359445),
        ..Default::default()
    } ; "legacy seconds seeding"
)]
#[test_case(r#"{"secondsSeeding":0}"# => Torrent {
        seconds_seeding: Some(0),
        ..Default::default()
    } ; "legacy seconds seeding zero"
)]
#[test_case(r#"{"secondsSeeding":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy seconds seeding negative"
)]

#[test_case(r#"{"seedIdleLimit":30}"# => Torrent {
        seed_idle_limit: Some(30),
        ..Default::default()
    } ; "legacy seed idle limit"
)]
#[test_case(r#"{"seedIdleLimit":0}"# => Torrent {
        seed_idle_limit: Some(0),
        ..Default::default()
    } ; "legacy seed idle limit zero"
)]
#[test_case(r#"{"seedIdleLimit":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy seed idle limit negative"
)]

#[test_case(r#"{"seedIdleMode":0}"# => Torrent {
        seed_idle_mode: Some(IdleMode::Global),
        ..Default::default()
    } ; "legacy seed idle mode"
)]

#[test_case(r#"{"seedRatioLimit": 3.14}"# => Torrent {
        seed_ratio_limit: Some(3.14),
        ..Default::default()
    } ; "legacy seed ratio limit"
)]
#[test_case(r#"{"seedRatioLimit": 0}"# => Torrent {
        seed_ratio_limit: Some(0.),
        ..Default::default()
    } ; "legacy seed ratio limit zero"
)]
#[test_case(r#"{"seedRatioLimit": 1}"# => Torrent {
        seed_ratio_limit: Some(1.),
        ..Default::default()
    } ; "legacy seed ratio limit one"
)]
#[test_case(r#"{"seedRatioLimit": -1}"# => Torrent {
        seed_ratio_limit: Some(-1.),
        ..Default::default()
    } ; "legacy seed ratio limit negative"
)]

#[test_case(r#"{"seedRatioMode":2}"# => Torrent {
        seed_ratio_mode: Some(RatioMode::Unlimited),
        ..Default::default()
    } ; "legacy seed ratio mode"
)]

// NOTE: No legacy sequential_download test because it doesn't exist pre- semver-6.0.0.
// NOTE: No legacy sequential_download_from_piece test because it doesn't exist pre- semver-6.0.0.

#[test_case(r#"{"sizeWhenDone":2965366874}"# => Torrent {
        size_when_done: Some(2965366874),
        ..Default::default()
    } ; "legacy size when done"
)]
#[test_case(r#"{"sizeWhenDone":0}"# => Torrent {
        size_when_done: Some(0),
        ..Default::default()
    } ; "legacy size when done zero"
)]
#[test_case(r#"{"sizeWhenDone":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy size when done negative"
)]

#[test_case(r#"{"startDate":1774014859}"# => Torrent {
        start_date: DateTime::parse_from_rfc3339("2026-03-20 13:54:19+00:00")
                .ok()
                .map(|dt| dt.to_utc()),
        ..Default::default()
    } ; "legacy start date"
)]
#[test_case(r#"{"startDate":-1}"# => Torrent {
        start_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy start date negative"
)]
#[test_case(r#"{"startDate":0}"# => Torrent {
        start_date: Some(DateTime::UNIX_EPOCH),
        ..Default::default()
    } ; "legacy start date zero"
)]

#[test_case(r#"{"status":4}"# => Torrent {
        status: Some(TorrentStatus::Downloading),
        ..Default::default()
    } ; "legacy status"
)]

#[test_case(
    r#"{
        "torrentFile": "/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent"
    }"# => Torrent {
        torrent_file: Some("/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent".into()),
        ..Default::default()
    } ; "legacy torrent file"
)]
#[test_case(r#"{"torrentFile":""}"# => Torrent {
        torrent_file: Some("".into()),
        ..Default::default()
    } ; "legacy torrent file empty"
)]

#[test_case(r#"{"totalSize":2050306968}"# => Torrent {
        total_size: Some(2050306968),
        ..Default::default()
    } ; "legacy total size"
)]
#[test_case(r#"{"totalSize":0}"# => Torrent {
        total_size: Some(0),
        ..Default::default()
    } ; "legacy total size zero"
)]
#[test_case(r#"{"totalSize":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy total size negative"
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
    } ; "legacy trackers single"
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
    } ; "legacy trackers multiple"
)]
#[test_case(r#"{ "trackers": [] }"# => Torrent {
        trackers: Some(vec![]),
        ..Default::default()
    } ; "legacy trackers empty"
)]

#[test_case("{\
        \"trackerList\":\"http://bt1.archive.org:6969/announce\\n\
        \\n\
        http://bt2.archive.org:6969/announce\\n\"\
    }" => Torrent {
        tracker_list: Some(vec![
            vec![Url::parse("http://bt1.archive.org:6969/announce").expect("valid url")],
            vec![Url::parse("http://bt2.archive.org:6969/announce").expect("valid url")],
        ].into()),
        ..Default::default()
    } ; "legacy tracker list"
)]

#[test_case(r#"{
        "trackerStats":[
            {
                "announce":"https://example.com/announce",
                "announceState":1,
                "downloadCount":245,
                "hasAnnounced":true,
                "hasScraped":true,
                "host":"example.com:8080",
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
                "scrape":"https://example.com:8080",
                "seederCount":77,
                "tier":0
            }
        ]
    }"# => Torrent {
        tracker_stats: Some(vec![
            TrackerStat {
                announce: Url::parse("https://example.com/announce").expect("valid url"),
                announce_state: TrackerState::Waiting,
                download_count: 245,
                downloader_count: -1, // Doesn't exist pre- semver-6.0.0
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
    } ; "legacy tracker stats single"
)]
#[test_case(r#"{
        "trackerStats":[
            {
                "announce":"https://example.com/announce",
                "announceState":1,
                "downloadCount":245,
                "hasAnnounced":true,
                "hasScraped":true,
                "host":"example.com:8080",
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
                "scrape":"https://example.com:8080",
                "seederCount":77,
                "tier":0
            },
            {
                "announce":"http://example.org/foo/announce",
                "announceState":0,
                "downloadCount":24,
                "hasAnnounced":false,
                "hasScraped":false,
                "host":"example.org:8080",
                "id":666,
                "isBackup":true,
                "lastAnnouncePeerCount":9999,
                "lastAnnounceResult":"IPv4 connection failed",
                "lastAnnounceStartTime":0,
                "lastAnnounceSucceeded":false,
                "lastAnnounceTime":1773989659,
                "lastAnnounceTimedOut":false,
                "lastScrapeResult":"Could not connect to tracker",
                "lastScrapeStartTime":0,
                "lastScrapeSucceeded":false,
                "lastScrapeTime":1723614865,
                "lastScrapeTimedOut":false,
                "leecherCount":2,
                "nextAnnounceTime":1723618230,
                "nextScrapeTime":0,
                "scrapeState":0,
                "scrape":"http://example.org/scrape",
                "seederCount":5,
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
                downloader_count: -1, // Doesn't exist pre- semver-6.0.0
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
                downloader_count: -1, // Doesn't exist pre- semver-6.0.0
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
    } ; "legacy tracker stats multiple"
)]
#[test_case(r#"{ "trackerStats":[] }"# => Torrent {
        tracker_stats: Some(vec![]),
        ..Default::default()
    } ; "legacy tracker stats empty"
)]

#[test_case(r#"{"uploadedEver":1301396208}"# => Torrent {
        uploaded_ever: Some(1301396208),
        ..Default::default()
    } ; "legacy uploaded ever"
)]
#[test_case(r#"{"uploadedEver":0}"# => Torrent {
        uploaded_ever: Some(0),
        ..Default::default()
    } ; "legacy uploaded ever zero"
)]
#[test_case(r#"{"uploadedEver":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy uploaded ever negative"
)]

#[test_case(r#"{"uploadLimit":1024}"# => Torrent {
        upload_limit: Some(1024),
        ..Default::default()
    } ; "legacy uploaded limit"
)]
#[test_case(r#"{"uploadLimit":0}"# => Torrent {
        upload_limit: Some(0),
        ..Default::default()
    } ; "legacy uploaded limit zero"
)]
#[test_case(r#"{"uploadLimit":-1}"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy uploaded limit negative"
)]

#[test_case(r#"{"uploadLimited":true}"# => Torrent {
        upload_limited: Some(true),
        ..Default::default()
    } ; "legacy uploaded limited"
)]

#[test_case(r#"{"uploadRatio": 1.23}"# => Torrent {
        upload_ratio: Some(1.23),
        ..Default::default()
    } ; "legacy upload ratio"
)]
#[test_case(r#"{"uploadRatio": 0}"# => Torrent {
        upload_ratio: Some(0.),
        ..Default::default()
    } ; "legacy upload ratio zero"
)]
#[test_case(r#"{"uploadRatio": 1}"# => Torrent {
        upload_ratio: Some(1.),
        ..Default::default()
    } ; "legacy upload ratio one"
)]
#[test_case(r#"{"uploadRatio": -1}"# => Torrent {
        upload_ratio: Some(-1.),
        ..Default::default()
    } ; "legacy upload ratio negative"
)]

#[test_case(r#"{"wanted":[0, 1, 0, 0, 1]}"# => Torrent {
        wanted: Some(vec![false, true, false, false, true]),
        ..Default::default()
    } ; "legacy wanted ints"
)]
#[test_case(r#"{"wanted":[false, true, false, false, true]}"# => Torrent {
        wanted: Some(vec![false, true, false, false, true]),
        ..Default::default()
    } ; "legacy wanted bools"
)]
#[test_case(r#"{"wanted":[]}"# => Torrent {
        wanted: Some(vec![]),
        ..Default::default()
    } ; "legacy wanted empty"
)]
#[test_case(r#"{"wanted":[-1]}"# => panics "failed to deserialize torrent: unexpected number: -1"
    ; "legacy wanted invalid negative"
)]
#[test_case(r#"{"wanted":[2]}"# => panics "failed to deserialize torrent: unexpected number: 2"
    ; "legacy wanted invalid positive"
)]
#[test_case(r#"{"wanted":["foo"]}"# => panics "failed to deserialize torrent: unexpected type"
    ; "legacy wanted invalid type"
)]

#[test_case(r#"{
        "webseeds": [
            "https://cdimage.debian.org/debian-cd/",
            "https://dl.example.com/foo/",
            "https://bar.example.com/"
        ]
    }"# => Torrent {
        webseeds: Some(vec![
            Url::parse("https://cdimage.debian.org/debian-cd/").expect("valid url"),
            Url::parse("https://dl.example.com/foo/").expect("valid url"),
            Url::parse("https://bar.example.com/").expect("valid url"),
        ]),
        ..Default::default()
    } ; "legacy webseeds"
)]
#[test_case(r#"{ "webseeds": [] }"# => Torrent {
        webseeds: Some(vec![]),
        ..Default::default()
    } ; "legacy webseeds empty"
)]
#[test_case(r#"{"webseeds":["malformed"]}"# => panics "relative URL without a base"
    ; "legacy webseeds malformed"
)]

// NOTE: No legacy webseeds_ex test because it doesn't exist pre- semver-6.0.0.

#[test_case(r#"{"webseedsSendingToUs":1234}"# => Torrent {
        webseeds_sending_to_us: Some(1234),
        ..Default::default()
    } ; "legacy webseeds sending to us"
)]
#[test_case(r#"{"webseedsSendingToUs":0}"# => Torrent {
        webseeds_sending_to_us: Some(0),
        ..Default::default()
    } ; "legacy webseeds sending to us zero"
)]
#[test_case(r#"{"webseedsSendingToUs":-1}"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy webseeds sending to us negative"
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
