//! This file defines [`Torrent`] field sub-type deserialization tests.

use test_case::test_case;

use super::*;

fn deserialize_test<'de, T: Deserialize<'de>>(data: &'de str) -> T {
    match serde_json::from_str(data) {
        Ok(de) => de,
        Err(err) => panic!("{err}"),
    }
}

#[test_case("0" => ErrorType::Ok ; "error type ok")]
#[test_case("1" => ErrorType::TrackerWarning ; "error type tracker warning")]
#[test_case("2" => ErrorType::TrackerError ; "error type tracker error")]
#[test_case("3" => ErrorType::LocalError ; "error type local error")]
#[test_case("4" => panics "invalid value: 4, expected one of: 0, 1, 2, 3" ; "error type too high")]
#[test_case("-1" => panics "invalid value: integer `-1`, expected u8" ; "error type too low")]
fn error_type_deserialize(repr: &str) -> ErrorType {
    deserialize_test(repr)
}

#[test_case(
    r#"{
        "bytesCompleted": 1234,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 1234,
        length: 4567,
        name: "foo.bar.gif".into(),
        begin_piece: None,
        end_piece: None,
    } ; "legacy ok"
)]
#[test_case(
    r#"{
        "bytesCompleted": 0,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 0,
        length: 4567,
        name: "foo.bar.gif".into(),
        begin_piece: None,
        end_piece: None,
    } ; "legacy bytes completed zero"
)]
#[test_case(
    r#"{
        "bytesCompleted": 1234,
        "length": 4567,
        "name": ""
    }"# => File {
        bytes_completed: 1234,
        length: 4567,
        name: "".into(),
        begin_piece: None,
        end_piece: None,
    } ; "legacy name empty"
)]
#[test_case(
    r#"{
        "bytesCompleted": 1234,
        "length": 0,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 1234,
        length: 0,
        name: "foo.bar.gif".into(),
        begin_piece: None,
        end_piece: None,
    } ; "legacy length zero"
)]
#[test_case(
    r#"{
        "bytesCompleted": -1,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy bytes completed negative"
)]
#[test_case(
    r#"{
        "bytesCompleted": 1234,
        "length": -1,
        "name": "foo.bar.gif"
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy length negative"
)]

#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": 1234,
        "end_piece": 42,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 1234,
        length: 4567,
        name: "foo.bar.gif".into(),
        begin_piece: Some(13),
        end_piece: Some(42),
    } ; "semver 6.0.0 ok"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": 0,
        "end_piece": 42,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 0,
        length: 4567,
        name: "foo.bar.gif".into(),
        begin_piece: Some(13),
        end_piece: Some(42),
    } ; "semver 6.0.0 bytes completed zero"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": 1234,
        "end_piece": 42,
        "length": 4567,
        "name": ""
    }"# => File {
        bytes_completed: 1234,
        length: 4567,
        name: "".into(),
        begin_piece: Some(13),
        end_piece: Some(42),
    } ; "semver 6.0.0 name empty"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": 1234,
        "end_piece": 42,
        "length": 0,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 1234,
        length: 0,
        name: "foo.bar.gif".into(),
        begin_piece: Some(13),
        end_piece: Some(42),
    } ; "semver 6.0.0 length zero"
)]
#[test_case(
    r#"{
        "begin_piece": 0,
        "bytes_completed": 1234,
        "end_piece": 42,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 1234,
        length: 4567,
        name: "foo.bar.gif".into(),
        begin_piece: Some(0),
        end_piece: Some(42),
    } ; "semver 6.0.0 begin piece zero"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": 1234,
        "end_piece": 0,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => File {
        bytes_completed: 1234,
        length: 4567,
        name: "foo.bar.gif".into(),
        begin_piece: Some(13),
        end_piece: Some(0),
    } ; "semver 6.0.0 end piece zero"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": -1,
        "end_piece": 42,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 bytes completed negative"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytesCompleted": 1234,
        "end_piece": 42,
        "length": -1,
        "name": "foo.bar.gif"
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 length negative"
)]
#[test_case(
    r#"{
        "begin_piece": -1,
        "bytes_completed": 1234,
        "end_piece": 42,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 begin piece negative"
)]
#[test_case(
    r#"{
        "begin_piece": 13,
        "bytes_completed": 1234,
        "end_piece": -1,
        "length": 4567,
        "name": "foo.bar.gif"
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "semver 6.0.0 end piece negative"
)]

fn file_deserialize(file_data: &str) -> File {
    deserialize_test(file_data)
}

#[test_case(
    r#"{
        "bytesCompleted": 1234,
        "priority": 0,
        "wanted": true
    }"# => FileStat {
        bytes_completed: 1234,
        priority: Priority::Normal,
        wanted: true,
    } ; "legacy ok"
)]
#[test_case(
    r#"{
        "bytesCompleted": -1,
        "priority": 1,
        "wanted": true
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "legacy bytes completed negative"
)]
fn file_stat_deserialize(stat_data: &str) -> FileStat {
    deserialize_test(stat_data)
}

#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "legacy ok"
)]

#[test_case(
    r#"{
        "fromCache":0,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 0,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "legacy from cache zero"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":0,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 0,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "legacy from dht zero"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":0,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 0,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "legacy from incoming zero"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":0,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 0,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "legacy from lpd zero"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":0,
        "fromPex":6,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 0,
        from_pex: 6,
        from_tracker: 7,
    } ; "legacy from ltep zero"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":0,
        "fromTracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 0,
        from_tracker: 7,
    } ; "legacy from pex zero"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":0
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 0,
    } ; "legacy from tracker zero"
)]

#[test_case(
    r#"{
        "fromCache":-1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => panics "invalid value: integer `-1`, expected u16"
    ; "legacy from cache negative"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":-2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => panics "invalid value: integer `-2`, expected u16"
    ; "legacy from dht negative"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":-3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => panics "invalid value: integer `-3`, expected u16"
    ; "legacy from incoming negative"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":-4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":7
    }"# => panics "invalid value: integer `-4`, expected u16"
    ; "legacy from lpd negative"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":-5,
        "fromPex":6,
        "fromTracker":7
    }"# => panics "invalid value: integer `-5`, expected u16"
    ; "legacy from ltep negative"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":-6,
        "fromTracker":7
    }"# => panics "invalid value: integer `-6`, expected u16"
    ; "legacy from pex negative"
)]
#[test_case(
    r#"{
        "fromCache":1,
        "fromDht":2,
        "fromIncoming":3,
        "fromLpd":4,
        "fromLtep":5,
        "fromPex":6,
        "fromTracker":-7
    }"# => panics "invalid value: integer `-7`, expected u16"
    ; "legacy from tracker negative"
)]

#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "semver 6.0.0 ok"
)]

#[test_case(
    r#"{
        "from_cache":0,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 0,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "semver 6.0.0 from cache zero"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":0,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 0,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "semver 6.0.0 from dht zero"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":0,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 0,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "semver 6.0.0 from incoming zero"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":0,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 0,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 7,
    } ; "semver 6.0.0 from lpd zero"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":0,
        "from_pex":6,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 0,
        from_pex: 6,
        from_tracker: 7,
    } ; "semver 6.0.0 from ltep zero"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":0,
        "from_tracker":7
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 0,
        from_tracker: 7,
    } ; "semver 6.0.0 from pex zero"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":0
    }"# => PeersFrom {
        from_cache: 1,
        from_dht: 2,
        from_incoming: 3,
        from_lpd: 4,
        from_ltep: 5,
        from_pex: 6,
        from_tracker: 0,
    } ; "semver 6.0.0 from tracker zero"
)]

#[test_case(
    r#"{
        "from_cache":-1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => panics "invalid value: integer `-1`, expected u16"
    ; "semver 6.0.0 from cache negative"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":-2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => panics "invalid value: integer `-2`, expected u16"
    ; "semver 6.0.0 from dht negative"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":-3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => panics "invalid value: integer `-3`, expected u16"
    ; "semver 6.0.0 from incoming negative"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":-4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":7
    }"# => panics "invalid value: integer `-4`, expected u16"
    ; "semver 6.0.0 from lpd negative"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":-5,
        "from_pex":6,
        "from_tracker":7
    }"# => panics "invalid value: integer `-5`, expected u16"
    ; "semver 6.0.0 from ltep negative"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":-6,
        "from_tracker":7
    }"# => panics "invalid value: integer `-6`, expected u16"
    ; "semver 6.0.0 from pex negative"
)]
#[test_case(
    r#"{
        "from_cache":1,
        "from_dht":2,
        "from_incoming":3,
        "from_lpd":4,
        "from_ltep":5,
        "from_pex":6,
        "from_tracker":-7
    }"# => panics "invalid value: integer `-7`, expected u16"
    ; "semver 6.0.0 from tracker negative"
)]

fn peers_from_deserialize(from_data: &str) -> PeersFrom {
    deserialize_test(from_data)
}

#[test_case(r#"0"# => TorrentStatus::Stopped ; "stopped")]
#[test_case(r#"1"# => TorrentStatus::QueuedToVerify ; "queued to verify")]
#[test_case(r#"2"# => TorrentStatus::Verifying ; "verifying")]
#[test_case(r#"3"# => TorrentStatus::QueuedToDownload ; "queued to download")]
#[test_case(r#"4"# => TorrentStatus::Downloading ; "downloading")]
#[test_case(r#"5"# => TorrentStatus::QueuedToSeed ; "queued to seed")]
#[test_case(r#"6"# => TorrentStatus::Seeding ; "seeding")]
#[test_case(r#"-1"# => panics "invalid value: integer `-1`, expected u8 at line 1 column 2"
    ; "invalid negative")]
#[test_case(r#"7"# => panics "invalid value: 7, expected one of: 0, 1, 2, 3, 4, 5, 6"
    ; "invalid positive")]
fn status_deserialize(status_data: &str) -> TorrentStatus {
    deserialize_test(status_data)
}

#[test_case(r#"{
        "id": 123,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "example",
        "tier": 1
    }"# => Tracker {
        id: 123,
        announce: Url::parse("http://b.example.com:4096/announce")
            .expect("valid url"),
        scrape: Url::parse("http://b.example.com:4096/scrape")
            .expect("valid url"),
        sitename: "example".into(),
        tier: 1,
    } ; "ok"
)]

#[test_case(r#"{
        "id": 0,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "example",
        "tier": 0
    }"# => Tracker {
        id: 0,
        announce: Url::parse("http://b.example.com:4096/announce")
            .expect("valid url"),
        scrape: Url::parse("http://b.example.com:4096/scrape")
            .expect("valid url"),
        sitename: "example".into(),
        tier: 0,
    } ; "id zero"
)]
#[test_case(r#"{
        "id": 123,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "example",
        "tier": 0
    }"# => Tracker {
        id: 123,
        announce: Url::parse("http://b.example.com:4096/announce")
            .expect("valid url"),
        scrape: Url::parse("http://b.example.com:4096/scrape")
            .expect("valid url"),
        sitename: "example".into(),
        tier: 0,
    } ; "tier zero"
)]
#[test_case(r#"{
        "id": 123,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "",
        "tier": 1
    }"# => Tracker {
        id: 123,
        announce: Url::parse("http://b.example.com:4096/announce")
            .expect("valid url"),
        scrape: Url::parse("http://b.example.com:4096/scrape")
            .expect("valid url"),
        sitename: "".into(),
        tier: 1,
    } ; "sitename empty"
)]
#[test_case(r#"{
        "id": 123,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "tier": 1
    }"# => Tracker {
        id: 123,
        announce: Url::parse("http://b.example.com:4096/announce")
            .expect("valid url"),
        scrape: Url::parse("http://b.example.com:4096/scrape")
            .expect("valid url"),
        sitename: "".into(),
        tier: 1,
    } ; "sitename missing"
)]

#[test_case(r#"{
        "id": -1,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "example",
        "tier": 1
    }"# => panics "invalid value: integer `-1`, expected u32"
    ; "id negative"
)]
#[test_case(r#"{
        "id": 123,
        "announce": "malformed",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "example",
        "tier": 1
    }"# => panics "relative URL without a base"
    ; "announce malformed"
)]
#[test_case(r#"{
        "id": 123,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "malformed",
        "sitename": "example",
        "tier": 1
    }"# => panics "relative URL without a base"
    ; "scrape malformed"
)]
#[test_case(r#"{
        "id": 123,
        "announce": "http://b.example.com:4096/announce",
        "scrape": "http://b.example.com:4096/scrape",
        "sitename": "example",
        "tier": -1
    }"# => panics "invalid value: integer `-1`, expected usize"
    ; "tier negative"
)]
fn tracker_deserialize(tracker_data: &str) -> Tracker {
    deserialize_test(tracker_data)
}

#[test_case(r#"0"# => TrackerState::Inactive ; "inactive")]
#[test_case(r#"1"# => TrackerState::Waiting ; "waiting")]
#[test_case(r#"2"# => TrackerState::Queued ; "queued")]
#[test_case(r#"3"# => TrackerState::Active ; "active")]
#[test_case(r#"-1"# => panics "invalid value: integer `-1`, expected u8 at line 1 column 2"
    ; "invalid negative")]
#[test_case(r#"4"# => panics "invalid value: 4, expected one of: 0, 1, 2, 3"
    ; "invalid positive")]
fn tracker_state_deserialize(state_data: &str) -> TrackerState {
    deserialize_test(state_data)
}

#[test_case(r#"{
        "url": "https://iso.example.com:8080/download",
        "is_downloading": true,
        "download_bytes_per_second": 20000
    }"# => WebseedsEx {
        url: Url::parse("https://iso.example.com:8080/download").expect("valid url"),
        is_downloading: true,
        download_bytes_per_second: 20000,
    } ; "ok"
)]
#[test_case(r#"{
        "url": "https://iso.example.com:8080/download",
        "is_downloading": false,
        "download_bytes_per_second": 0
    }"# => WebseedsEx {
        url: Url::parse("https://iso.example.com:8080/download").expect("valid url"),
        is_downloading: false,
        download_bytes_per_second: 0,
    } ; "download bytes per second zero"
)]
#[test_case(r#"{
        "url": "malformed",
        "is_downloading": true,
        "download_bytes_per_second": 0
    }"# => panics "relative URL without a base"
    ; "url malformed"
)]
#[test_case(r#"{
        "url": "https://iso.example.com:8080/download",
        "is_downloading": true,
        "download_bytes_per_second": -1
    }"# => panics "invalid value: integer `-1`, expected u64"
    ; "download bytes per second negative"
)]
fn webseeds_ex_deserialize(webseeds_data: &str) -> WebseedsEx {
    deserialize_test(webseeds_data)
}
