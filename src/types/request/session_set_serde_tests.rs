//! This file defines serde tests for legacy and json-rpc requests.

use super::{*, test_helper::verify};
use crate::types::{JSON_RPC_VERSION_2_0, Result, Transport};

#[test]
fn request_session_set_legacy_alt_speed_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_down: Some(321),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-down\":321")
}

#[test]
fn request_session_set_semver_600_alt_speed_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_down: Some(321),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_down\":321")
}

#[test]
fn request_session_set_legacy_alt_speed_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-enabled\":true")
}

#[test]
fn request_session_set_semver_600_alt_speed_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_enabled\":false")
}

#[test]
fn request_session_set_legacy_alt_speed_time_begin() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_begin: Some(123),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-time-begin\":123")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_begin() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_begin: Some(123),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_begin\":123")
}

#[test]
fn request_session_set_legacy_alt_speed_time_day() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_day: Some({
            AltSpeedDay::MONDAY | AltSpeedDay::TUESDAY | AltSpeedDay::WEDNESDAY
                | AltSpeedDay::THURSDAY | AltSpeedDay::FRIDAY
        }),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-time-day\":62")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_day() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_day: Some({
            AltSpeedDay::MONDAY | AltSpeedDay::TUESDAY | AltSpeedDay::WEDNESDAY
                | AltSpeedDay::THURSDAY | AltSpeedDay::FRIDAY
        }),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_day\":62")
}

#[test]
fn request_session_set_legacy_alt_speed_time_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-time-enabled\":true")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_enabled\":true")
}

#[test]
fn request_session_set_legacy_alt_speed_time_end() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_end: Some(666),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-time-end\":666")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_end() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_end: Some(666),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_time_end\":666")
}

#[test]
fn request_session_set_legacy_alt_speed_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_up: Some(250),
        ..Default::default()
    };
    verify(session_set_args, None, "\"alt-speed-up\":250")
}

#[test]
fn request_session_set_semver_600_alt_speed_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_up: Some(250),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"alt_speed_up\":250")
}

#[test]
fn request_session_set_legacy_anti_brute_force_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        anti_brute_force_enabled: Some(false),
        ..Default::default()
    };
    // `anti_brute_force_enabled` only exists post- semver-6.0.0.
    verify(session_set_args, None, "")
}

#[test]
fn request_session_set_semver_600_anti_brute_force_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        anti_brute_force_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"anti_brute_force_enabled\":false")
}

#[test]
fn request_session_set_legacy_anti_brute_force_threshold() -> Result<()> {
    let session_set_args = SessionSetArgs {
        anti_brute_force_threshold: Some(101),
        ..Default::default()
    };
    // `anti_brute_force_threshold` only exists post- semver-6.0.0.
    verify(session_set_args, None, "")
}

#[test]
fn request_session_set_semver_600_anti_brute_force_threshold() -> Result<()> {
    let session_set_args = SessionSetArgs {
        anti_brute_force_threshold: Some(101),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"anti_brute_force_threshold\":101")
}

#[test]
fn request_session_set_legacy_blocklist_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"blocklist-enabled\":false")
}

#[test]
fn request_session_set_semver_600_blocklist_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"blocklist_enabled\":false")
}

#[test]
fn request_session_set_legacy_blocklist_url() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_url: Some("https://example.com".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None, "\"blocklist-url\":\"https://example.com\"")
}

#[test]
fn request_session_set_semver_600_blocklist_url() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_url: Some("https://example.com".to_string()),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"blocklist_url\":\"https://example.com\"")
}

#[test]
fn request_session_set_legacy_cache_size_mb() -> Result<()> {
    let session_set_args = SessionSetArgs {
        cache_size_mb: Some(8),
        ..Default::default()
    };
    verify(session_set_args, None, "\"cache-size-mb\":8")
}

#[test]
fn request_session_set_semver_600_cache_size_mb() -> Result<()> {
    let session_set_args = SessionSetArgs {
        cache_size_mb: Some(8),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"cache_size_mib\":8")
}

#[test]
fn request_session_set_legacy_default_trackers() -> Result<()> {
    let session_set_args = SessionSetArgs {
        default_trackers: Some("https://example.com:5555".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None, "\"default-trackers\":\"https://example.com:5555\"")
}

#[test]
fn request_session_set_semver_600_default_trackers() -> Result<()> {
    let session_set_args = SessionSetArgs {
        default_trackers: Some("https://example.com:5555".to_string()),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"default_trackers\":\"https://example.com:5555\"")
}

#[test]
fn request_session_set_legacy_dht_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        dht_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"dht-enabled\":true")
}

#[test]
fn request_session_set_semver_600_dht_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        dht_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"dht_enabled\":true")
}

#[test]
fn request_session_set_legacy_download_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_dir: Some("/downloads".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None, "\"download-dir\":\"/downloads\"")
}

#[test]
fn request_session_set_semver_600_download_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_dir: Some("/downloads".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"download_dir\":\"/downloads\"")
}

#[test]
fn request_session_set_legacy_download_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"download-queue-enabled\":true")
}

#[test]
fn request_session_set_semver_600_download_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"download_queue_enabled\":true")
}

#[test]
fn request_session_set_legacy_download_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_size: Some(1),
        ..Default::default()
    };
    verify(session_set_args, None, "\"download-queue-size\":1")
}

#[test]
fn request_session_set_semver_600_download_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_size: Some(1),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"download_queue_size\":1")
}

#[test]
fn request_session_set_legacy_encryption() -> Result<()> {
    let session_set_args = SessionSetArgs {
        encryption: Some(Encryption::Tolerated),
        ..Default::default()
    };
    verify(session_set_args, None, "\"encryption\":\"tolerated\"")
}

#[test]
fn request_session_set_semver_600_encryption() -> Result<()> {
    let session_set_args = SessionSetArgs {
        encryption: Some(Encryption::Tolerated),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"encryption\":\"allowed\"")
}

#[test]
fn request_session_set_legacy_idle_seeding_limit_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"idle-seeding-limit-enabled\":false")
}

#[test]
fn request_session_set_semver_600_idle_seeding_limit_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"idle_seeding_limit_enabled\":false")
}

#[test]
fn request_session_set_legacy_idle_seeding_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit: Some(4320),
        ..Default::default()
    };
    verify(session_set_args, None, "\"idle-seeding-limit\":4320")
}

#[test]
fn request_session_set_semver_600_idle_seeding_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit: Some(4320),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"idle_seeding_limit\":4320")
}

#[test]
fn request_session_set_legacy_incomplete_dir_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"incomplete-dir-enabled\":false")
}

#[test]
fn request_session_set_semver_600_incomplete_dir_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"incomplete_dir_enabled\":false")
}

#[test]
fn request_session_set_legacy_incomplete_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir: Some("/incomplete".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None, "\"incomplete-dir\":\"/incomplete\"")
}

#[test]
fn request_session_set_semver_600_incomplete_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir: Some("/incomplete".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"incomplete_dir\":\"/incomplete\"")
}

#[test]
fn request_session_set_legacy_lpd_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        lpd_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"lpd-enabled\":false")
}

#[test]
fn request_session_set_semver_600_lpd_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        lpd_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"lpd_enabled\":false")
}

#[test]
fn request_session_set_legacy_peer_limit_global() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_global: Some(120),
        ..Default::default()
    };
    verify(session_set_args, None, "\"peer-limit-global\":120")
}

#[test]
fn request_session_set_semver_600_peer_limit_global() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_global: Some(120),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_limit_global\":120")
}

#[test]
fn request_session_set_legacy_peer_limit_per_torrent() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_per_torrent: Some(10),
        ..Default::default()
    };
    verify(session_set_args, None, "\"peer-limit-per-torrent\":10")
}

#[test]
fn request_session_set_semver_600_peer_limit_per_torrent() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_per_torrent: Some(10),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_limit_per_torrent\":10")
}

#[test]
fn request_session_set_legacy_peer_port_random_on_start() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port_random_on_start: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"peer-port-random-on-start\":false")
}

#[test]
fn request_session_set_semver_600_peer_port_random_on_start() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port_random_on_start: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_port_random_on_start\":false")
}

#[test]
fn request_session_set_legacy_peer_port() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port: Some(44556),
        ..Default::default()
    };
    verify(session_set_args, None, "\"peer-port\":44556")
}

#[test]
fn request_session_set_semver_600_peer_port() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port: Some(44556),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"peer_port\":44556")
}

#[test]
fn request_session_set_legacy_pex_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        pex_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"pex-enabled\":false")
}

#[test]
fn request_session_set_semver_600_pex_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        pex_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"pex_enabled\":false")
}

#[test]
fn request_session_set_legacy_port_forwarding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        port_forwarding_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"port-forwarding-enabled\":false")
}

#[test]
fn request_session_set_semver_600_port_forwarding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        port_forwarding_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"port_forwarding_enabled\":false")
}

#[test]
fn request_session_set_legacy_preferred_transports() -> Result<()> {
    let session_set_args = SessionSetArgs {
        preferred_transports: Some(vec![Transport::UTP, Transport::TCP]),
        ..Default::default()
    };
    // `preferred_transports` only exists post- semver-6.0.0.
    verify(session_set_args, None, "")
}

#[test]
fn request_session_set_semver_600_preferred_transports() -> Result<()> {
    let session_set_args = SessionSetArgs {
        preferred_transports: Some(vec![Transport::UTP, Transport::TCP]),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"preferred_transports\":[\"utp\",\"tcp\"]")
}

#[test]
fn request_session_set_legacy_queue_stalled_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"queue-stalled-enabled\":true")
}

#[test]
fn request_session_set_semver_600_queue_stalled_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"queue_stalled_enabled\":true")
}

#[test]
fn request_session_set_legacy_queue_stalled_minutes() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_minutes: Some(20),
        ..Default::default()
    };
    verify(session_set_args, None, "\"queue-stalled-minutes\":20")
}

#[test]
fn request_session_set_semver_600_queue_stalled_minutes() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_minutes: Some(20),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"queue_stalled_minutes\":20")
}

#[test]
fn request_session_set_legacy_rename_partial_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        rename_partial_files: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"rename-partial-files\":false")
}

#[test]
fn request_session_set_semver_600_rename_partial_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        rename_partial_files: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"rename_partial_files\":false")
}

#[test]
fn request_session_set_legacy_reqq() -> Result<()> {
    let session_set_args = SessionSetArgs {
        reqq: Some(2500),
        ..Default::default()
    };
    verify(session_set_args, None, "\"reqq\":2500")
}

#[test]
fn request_session_set_semver_600_reqq() -> Result<()> {
    let session_set_args = SessionSetArgs {
        reqq: Some(2500),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"reqq\":2500")
}

#[test]
fn request_session_set_legacy_script_torrent_added_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"script-torrent-added-enabled\":false")
}

#[test]
fn request_session_set_semver_600_script_torrent_added_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"script_torrent_added_enabled\":false")
}

#[test]
fn request_session_set_legacy_script_torrent_added_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_filename: Some("/scripts/added.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None, "\"script-torrent-added-filename\":\"/scripts/added.sh\"")
}

#[test]
fn request_session_set_semver_600_script_torrent_added_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_filename: Some("/scripts/added.sh".to_string()),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"script_torrent_added_filename\":\"/scripts/added.sh\"")
}

#[test]
fn request_session_set_legacy_script_torrent_done_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"script-torrent-done-enabled\":true")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"script_torrent_done_enabled\":true")
}

#[test]
fn request_session_set_legacy_script_torrent_done_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_filename: Some("/scripts/done.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None, "\"script-torrent-done-filename\":\"/scripts/done.sh\"")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_filename: Some("/scripts/done.sh".to_string()),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"script_torrent_done_filename\":\"/scripts/done.sh\"")
}

#[test]
fn request_session_set_legacy_script_torrent_done_seeding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"script-torrent-done-seeding-enabled\":false")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_seeding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_enabled: Some(true),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"script_torrent_done_seeding_enabled\":true")
}

#[test]
fn request_session_set_legacy_script_torrent_done_seeding_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_filename: Some("/scripts/done_seeding.sh".to_string()),
        ..Default::default()
    };
    verify(
        session_set_args,
        None,
        "\"script-torrent-done-seeding-filename\":\"/scripts/done_seeding.sh\"")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_seeding_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_filename: Some("/scripts/done_seeding.sh".to_string()),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"script_torrent_done_seeding_filename\":\"/scripts/done_seeding.sh\"")
}

#[test]
fn request_session_set_legacy_seed_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"seed-queue-enabled\":false")
}

#[test]
fn request_session_set_semver_600_seed_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_queue_enabled\":false")
}

#[test]
fn request_session_set_legacy_seed_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_size: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, None, "\"seed-queue-size\":1000")
}

#[test]
fn request_session_set_semver_600_seed_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_size: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_queue_size\":1000")
}

#[test]
fn request_session_set_legacy_seed_ratio_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limit: Some(2.0),
        ..Default::default()
    };
    verify(session_set_args, None, "\"seedRatioLimit\":2.0")
}

#[test]
fn request_session_set_semver_600_seed_ratio_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limit: Some(2.0),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_ratio_limit\":2.0")
}

#[test]
fn request_session_set_legacy_seed_ratio_limited() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limited: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"seedRatioLimited\":false")
}

#[test]
fn request_session_set_semver_600_seed_ratio_limited() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limited: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"seed_ratio_limited\":false")
}

#[test]
fn request_session_set_legacy_sequential_download() -> Result<()> {
    let session_set_args = SessionSetArgs {
        sequential_download: Some(false),
        ..Default::default()
    };
    // `sequential_download` only exists post- semver-6.0.0.
    verify(session_set_args, None, "")
}

#[test]
fn request_session_set_semver_600_sequential_download() -> Result<()> {
    let session_set_args = SessionSetArgs {
        sequential_download: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"sequential_download\":false")
}

#[test]
fn request_session_set_legacy_sequential_download_from_piece() -> Result<()> {
    let session_set_args = SessionSetArgs {
        sequential_download_from_piece: Some(321),
        ..Default::default()
    };
    // `sequential_download_from_piece` only exists post- semver-6.0.0.
    verify(session_set_args, None, "")
}

#[test]
fn request_session_set_semver_600_sequential_download_from_piece() -> Result<()> {
    let session_set_args = SessionSetArgs {
        sequential_download_from_piece: Some(403),
        ..Default::default()
    };
    verify(
        session_set_args,
        Some(JSON_RPC_VERSION_2_0),
        "\"sequential_download_from_piece\":403")
}

#[test]
fn request_session_set_legacy_speed_limit_down_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"speed-limit-down-enabled\":true")
}

#[test]
fn request_session_set_semver_600_speed_limit_down_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_down_enabled\":true")
}

#[test]
fn request_session_set_legacy_speed_limit_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down: Some(2000),
        ..Default::default()
    };
    verify(session_set_args, None, "\"speed-limit-down\":2000")
}

#[test]
fn request_session_set_semver_600_speed_limit_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down: Some(2000),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_down\":2000")
}

#[test]
fn request_session_set_legacy_speed_limit_up_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"speed-limit-up-enabled\":true")
}

#[test]
fn request_session_set_semver_600_speed_limit_up_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_up_enabled\":true")
}

#[test]
fn request_session_set_legacy_speed_limit_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, None, "\"speed-limit-up\":1000")
}

#[test]
fn request_session_set_semver_600_speed_limit_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"speed_limit_up\":1000")
}

#[test]
fn request_session_set_legacy_start_added_torrents() -> Result<()> {
    let session_set_args = SessionSetArgs {
        start_added_torrents: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None, "\"start-added-torrents\":false")
}

#[test]
fn request_session_set_semver_600_start_added_torrents() -> Result<()> {
    let session_set_args = SessionSetArgs {
        start_added_torrents: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"start_added_torrents\":false")
}

#[test]
fn request_session_set_legacy_trash_original_torrent_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        trash_original_torrent_files: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"trash-original-torrent-files\":true")
}

#[test]
fn request_session_set_semver_600_trash_original_torrent_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        trash_original_torrent_files: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"trash_original_torrent_files\":true")
}

#[test]
fn request_session_set_legacy_utp_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        utp_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None, "\"utp-enabled\":true")
}

#[test]
fn request_session_set_semver_600_utp_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        utp_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0), "\"utp_enabled\":true")
}
