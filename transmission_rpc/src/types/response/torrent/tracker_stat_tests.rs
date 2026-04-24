//! This file defines [`TrackerStat`] field sub-type deserialization tests.

use test_case::test_case;

use super::*;

#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy ok"
)]

#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 0,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,

        download_count: 0,
     
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy download count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,

        host: "".into(),

        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy host empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 0,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),

        id: 0,

        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy id zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 0,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,

        last_announce_peer_count: 0,

        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce peer count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": -1,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,

        last_announce_peer_count: -1,

        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce peer count negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,

        last_announce_result: "".into(),

        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce result empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,

        last_scrape_result: "".into(),

        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last scrape result empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 0,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),

        last_announce_start_time: DateTime::UNIX_EPOCH,

        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce start time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": -1,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),

        last_announce_start_time: DateTime::UNIX_EPOCH,

        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce start time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 0,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),

        last_scrape_start_time: DateTime::UNIX_EPOCH,

        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last scrape start time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": -1,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),

        last_scrape_start_time: DateTime::UNIX_EPOCH,

        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last scrape start time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 0,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,

        last_announce_time: DateTime::UNIX_EPOCH,

        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": -1,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,

        last_announce_time: DateTime::UNIX_EPOCH,

        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last announce time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 0,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,

        last_scrape_time: DateTime::UNIX_EPOCH,

        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last scrape time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": -1,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,

        last_scrape_time: DateTime::UNIX_EPOCH,

        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy last scrape time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": 0,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,

        leecher_count: 0,

        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy leecher count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 0,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,

        next_announce_time: DateTime::UNIX_EPOCH,

        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy next announce time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": -1,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,

        next_announce_time: DateTime::UNIX_EPOCH,

        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy next announce time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 0,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),

        next_scrape_time: DateTime::UNIX_EPOCH,

        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy next scrape time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": -1,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),

        next_scrape_time: DateTime::UNIX_EPOCH,

        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "legacy next scrape time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 0,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),

        seeder_count: 0,

        sitename: "example".into(),
        tier: 1,
    } ; "legacy seeder count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": -1,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),

        seeder_count: -1,

        sitename: "example".into(),
        tier: 1,
    } ; "legacy seeder count negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,

        sitename: "".into(),

        tier: 1,
    } ; "legacy sitename empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,

        sitename: "".into(),

        tier: 1,
    } ; "legacy sitename missing"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 0
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: -1, // Doesn't exist pre- semver-6.0.0
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 0,
    } ; "legacy tier zero"
)]

#[test_case(
    r#"{
        "announce": "malformed",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => panics "relative URL without a base"
    ; "legacy announce malformed"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": -1,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => panics "invalid value: integer `-1`, expected u32"
    ; "legacy id negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "malformed",
        "seederCount": 4,
        "sitename": "example",
        "tier": 1
    }"# => panics "relative URL without a base"
    ; "legacy scrape malformed"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announceState": 1,
        "downloadCount": 123,
        "hasAnnounced": true,
        "hasScraped": false,
        "host": "example.com:1234",
        "id": 666,
        "isBackup": true,
        "lastAnnouncePeerCount": 4,
        "lastAnnounceResult": "Success",
        "lastAnnounceStartTime": 1774990953,
        "lastAnnounceSucceeded": true,
        "lastAnnounceTime": 1774990954,
        "lastAnnounceTimedOut": false,
        "lastScrapeResult": "Tracker did not respond",
        "lastScrapeStartTime": 1774990954,
        "lastScrapeSucceeded": true,
        "lastScrapeTime": 1774990999,
        "lastScrapeTimedOut": false,
        "leecherCount": -1,
        "nextAnnounceTime": 1774994554,
        "nextScrapeTime": 1774994599,
        "scrapeState": 1,
        "scrape": "https://scrape.example.com:1234",
        "seederCount": 4,
        "sitename": "example",
        "tier": -1
    }"# => panics "invalid value: integer `-1`, expected usize"
    ; "legacy tier negative"
)]

#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 ok"
)]

#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 0,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,

        download_count: 0,
     
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 download count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 0,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,

        downloader_count: 0,

        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 downloader count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": -1,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,

        downloader_count: -1,

        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 downloader count negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,

        host: "".into(),

        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 host empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 0,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),

        id: 0,

        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 id zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 0,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,

        last_announce_peer_count: 0,

        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce peer count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": -1,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,

        last_announce_peer_count: -1,

        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce peer count negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,

        last_announce_result: "".into(),

        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce result empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,

        last_scrape_result: "".into(),

        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last scrape result empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 0,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),

        last_announce_start_time: DateTime::UNIX_EPOCH,

        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce start time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": -1,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),

        last_announce_start_time: DateTime::UNIX_EPOCH,

        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::UNIX_EPOCH,
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce start time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 0,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),

        last_scrape_start_time: DateTime::UNIX_EPOCH,

        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last scrape start time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": -1,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),

        last_scrape_start_time: DateTime::UNIX_EPOCH,

        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last scrape start time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 0,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,

        last_announce_time: DateTime::UNIX_EPOCH,

        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": -1,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,

        last_announce_time: DateTime::UNIX_EPOCH,

        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last announce time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 0,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,

        last_scrape_time: DateTime::UNIX_EPOCH,

        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last scrape time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": -1,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,

        last_scrape_time: DateTime::UNIX_EPOCH,

        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 last scrape time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": 0,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,

        leecher_count: 0,

        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 leecher count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 0,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,

        next_announce_time: DateTime::UNIX_EPOCH,

        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 next announce time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": -1,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,

        next_announce_time: DateTime::UNIX_EPOCH,

        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 next announce time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 0,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),

        next_scrape_time: DateTime::UNIX_EPOCH,

        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 next scrape time zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": -1,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),

        next_scrape_time: DateTime::UNIX_EPOCH,

        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 next scrape time negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 0,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),

        seeder_count: 0,

        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 seeder count zero"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": -1,
        "sitename": "example",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),

        seeder_count: -1,

        sitename: "example".into(),
        tier: 1,
    } ; "semver 6.0.0 seeder count negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "",
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,

        sitename: "".into(),

        tier: 1,
    } ; "semver 6.0.0 sitename empty"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "tier": 1
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,

        sitename: "".into(),

        tier: 1,
    } ; "semver 6.0.0 sitename missing"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 0
    }"# => TrackerStat {
        announce: Url::parse("https://announce.example.com:1234").expect("valid url"),
        announce_state: TrackerState::Waiting,
        download_count: 123,
        downloader_count: 23,
        has_announced: true,
        has_scraped: false,
        host: "example.com:1234".into(),
        id: 666,
        is_backup: true,
        last_announce_peer_count: 4,
        last_announce_result: "Success".into(),
        last_announce_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:33+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_succeeded: true,
        last_announce_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_announce_timed_out: false,
        last_scrape_result: "Tracker did not respond".into(),
        last_scrape_start_time: DateTime::parse_from_rfc3339("2026-03-31 21:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_succeeded: true,
        last_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 21:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        last_scrape_timed_out: false,
        leecher_count: -1,
        next_announce_time: DateTime::parse_from_rfc3339("2026-03-31 22:02:34+00:00")
            .expect("valid datetime")
            .to_utc(),
        next_scrape_time: DateTime::parse_from_rfc3339("2026-03-31 22:03:19+00:00")
            .expect("valid datetime")
            .to_utc(),
        scrape_state: TrackerState::Waiting,
        scrape: Url::parse("https://scrape.example.com:1234").expect("valid url"),
        seeder_count: 4,
        sitename: "example".into(),
        tier: 0,
    } ; "semver 6.0.0 tier zero"
)]

#[test_case(
    r#"{
        "announce": "malformed",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => panics "relative URL without a base"
    ; "semver 6.0.0 announce malformed"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": -1,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => panics "invalid value: integer `-1`, expected u32"
    ; "semver 6.0.0 id negative"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "malformed",
        "seeder_count": 4,
        "sitename": "example",
        "tier": 1
    }"# => panics "relative URL without a base"
    ; "semver 6.0.0 scrape malformed"
)]
#[test_case(
    r#"{
        "announce": "https://announce.example.com:1234",
        "announce_state": 1,
        "download_count": 123,
        "downloader_count": 23,
        "has_announced": true,
        "has_scraped": false,
        "host": "example.com:1234",
        "id": 666,
        "is_backup": true,
        "last_announce_peer_count": 4,
        "last_announce_result": "Success",
        "last_announce_start_time": 1774990953,
        "last_announce_succeeded": true,
        "last_announce_time": 1774990954,
        "last_announce_timed_out": false,
        "last_scrape_result": "Tracker did not respond",
        "last_scrape_start_time": 1774990954,
        "last_scrape_succeeded": true,
        "last_scrape_time": 1774990999,
        "last_scrape_timed_out": false,
        "leecher_count": -1,
        "next_announce_time": 1774994554,
        "next_scrape_time": 1774994599,
        "scrape_state": 1,
        "scrape": "https://scrape.example.com:1234",
        "seeder_count": 4,
        "sitename": "example",
        "tier": -1
    }"# => panics "invalid value: integer `-1`, expected usize"
    ; "semver 6.0.0 tier negative"
)]

fn tracker_stat_deserialize(stat_data: &str) -> TrackerStat {
    match serde_json::from_str(stat_data) {
        Ok(stat) => stat,
        Err(err) => panic!("{err}"),
    }
}
