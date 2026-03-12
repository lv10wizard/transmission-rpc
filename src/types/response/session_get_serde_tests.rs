//! This file defines session-get response serde tests.

use serde_json;

use crate::types::response::{SessionGet, SessionGetUnits};
use crate::types::{AltSpeedDay, Encryption, Result, RpcResponse};

#[test]
fn session_get_v300() -> Result<()> {
    let resp = serde_json::from_str::<RpcResponse<SessionGet>>(
        r#"
        {
          "arguments": {
            "alt-speed-down": 500,
            "alt-speed-enabled": false,
            "alt-speed-time-begin": 540,
            "alt-speed-time-day": 0,
            "alt-speed-time-enabled": false,
            "alt-speed-time-end": 1020,
            "alt-speed-up": 500,
            "blocklist-enabled": false,
            "blocklist-size": 0,
            "blocklist-url": "http://www.example.com/blocklist",
            "cache-size-mb": 16,
            "config-dir": "/config",
            "dht-enabled": false,
            "download-dir": "/downloads",
            "download-dir-free-space": 123456789,
            "download-queue-enabled": true,
            "download-queue-size": 1,
            "encryption": "allowed",
            "idle-seeding-limit": 30,
            "idle-seeding-limit-enabled": false,
            "incomplete-dir": "/incomplete",
            "incomplete-dir-enabled": false,
            "lpd-enabled": false,
            "peer-limit-global": 100,
            "peer-limit-per-torrent": 20,
            "peer-port": 55555,
            "peer-port-random-on-start": false,
            "pex-enabled": false,
            "port-forwarding-enabled": false,
            "queue-stalled-enabled": true,
            "queue-stalled-minutes": 30,
            "rename-partial-files": false,
            "rpc-version": 16,
            "rpc-version-minimum": 1,
            "script-torrent-done-enabled": true,
            "script-torrent-done-filename": "/usr/bin/test",
            "seed-queue-enabled": false,
            "seed-queue-size": 500,
            "seedRatioLimit": 2,
            "seedRatioLimited": false,
            "session-id": "w2oQxnkhWlut8omPLCDxgGR3g0pDX7gEan4Xz4QHqUlnlBEu",
            "speed-limit-down": 1000,
            "speed-limit-down-enabled": true,
            "speed-limit-up": 2000,
            "speed-limit-up-enabled": true,
            "start-added-torrents": false,
            "trash-original-torrent-files": true,
            "units": {
              "memory-bytes": 1024,
              "memory-units": [
                "KiB",
                "MiB",
                "GiB",
                "TiB"
              ],
              "size-bytes": 1000,
              "size-units": [
                "kB",
                "MB",
                "GB",
                "TB"
              ],
              "speed-bytes": 1000,
              "speed-units": [
                "kB/s",
                "MB/s",
                "GB/s",
                "TB/s"
              ]
            },
            "utp-enabled": true,
            "version": "3.00 (bb6b5a062e)"
          },
          "result": "success"
        }
        "#,
    )?;

    println!("{resp:#?}");
    assert!(resp.is_ok());

    assert_eq!(resp.arguments.alt_speed_down, Some(500));
    assert_eq!(resp.arguments.alt_speed_enabled, Some(false));
    assert_eq!(resp.arguments.alt_speed_time_begin, Some(540));
    assert_eq!(resp.arguments.alt_speed_time_day, Some(AltSpeedDay::empty()));
    assert_eq!(resp.arguments.alt_speed_time_enabled, Some(false));
    assert_eq!(resp.arguments.alt_speed_time_end, Some(1020));
    assert_eq!(resp.arguments.alt_speed_up, Some(500));

    assert_eq!(resp.arguments.anti_brute_force_enabled, None);
    assert_eq!(resp.arguments.anti_brute_force_threshold, None);

    assert_eq!(resp.arguments.blocklist_enabled, Some(false));
    assert_eq!(resp.arguments.blocklist_size, Some(0));
    assert_eq!(resp.arguments.blocklist_url, Some("http://www.example.com/blocklist".to_string()));

    assert_eq!(resp.arguments.cache_size_mb, Some(16));
    assert_eq!(resp.arguments.config_dir, Some("/config".to_string()));
    assert_eq!(resp.arguments.default_trackers, None);
    assert_eq!(resp.arguments.dht_enabled, Some(false));

    assert_eq!(resp.arguments.download_dir, Some("/downloads".to_string()));
    assert_eq!(resp.arguments.download_dir_free_space, Some(123456789));
    assert_eq!(resp.arguments.download_queue_enabled, Some(true));
    assert_eq!(resp.arguments.download_queue_size, Some(1));

    assert_eq!(resp.arguments.encryption, Some(Encryption::Allowed));
    assert_eq!(resp.arguments.idle_seeding_limit_enabled, Some(false));
    assert_eq!(resp.arguments.idle_seeding_limit, Some(30));

    assert_eq!(resp.arguments.incomplete_dir_enabled, Some(false));
    assert_eq!(resp.arguments.incomplete_dir, Some("/incomplete".to_string()));

    assert_eq!(resp.arguments.lpd_enabled, Some(false));
    assert_eq!(resp.arguments.peer_limit_global, Some(100));
    assert_eq!(resp.arguments.peer_limit_per_torrent, Some(20));
    assert_eq!(resp.arguments.peer_port_random_on_start, Some(false));
    assert_eq!(resp.arguments.peer_port, Some(55555));
    assert_eq!(resp.arguments.pex_enabled, Some(false));
    assert_eq!(resp.arguments.port_forwarding_enabled, Some(false));
    assert_eq!(resp.arguments.preferred_transports, None);
    assert_eq!(resp.arguments.queue_stalled_enabled, Some(true));
    assert_eq!(resp.arguments.queue_stalled_minutes, Some(30));
    assert_eq!(resp.arguments.rename_partial_files, Some(false));
    assert_eq!(resp.arguments.reqq, None);

    assert_eq!(resp.arguments.rpc_version_minimum, Some(1));
    assert_eq!(resp.arguments.rpc_version_semver, None);
    assert_eq!(resp.arguments.rpc_version, Some(16));

    assert_eq!(resp.arguments.script_torrent_added_enabled, None);
    assert_eq!(resp.arguments.script_torrent_added_filename, None);
    assert_eq!(resp.arguments.script_torrent_done_enabled, Some(true));
    assert_eq!(resp.arguments.script_torrent_done_filename, Some("/usr/bin/test".to_string()));
    assert_eq!(resp.arguments.script_torrent_done_seeding_enabled, None);
    assert_eq!(resp.arguments.script_torrent_done_seeding_filename, None);

    assert_eq!(resp.arguments.seed_queue_enabled, Some(false));
    assert_eq!(resp.arguments.seed_queue_size, Some(500));
    assert_eq!(resp.arguments.seed_ratio_limit, Some(2.0));
    assert_eq!(resp.arguments.seed_ratio_limited, Some(false));

    assert_eq!(resp.arguments.sequential_download, None);
    let session_id = "w2oQxnkhWlut8omPLCDxgGR3g0pDX7gEan4Xz4QHqUlnlBEu".to_string();
    assert_eq!(resp.arguments.session_id, Some(session_id));

    assert_eq!(resp.arguments.speed_limit_down_enabled, Some(true));
    assert_eq!(resp.arguments.speed_limit_down, Some(1000));
    assert_eq!(resp.arguments.speed_limit_up_enabled, Some(true));
    assert_eq!(resp.arguments.speed_limit_up, Some(2000));

    assert_eq!(resp.arguments.start_added_torrents, Some(false));
    assert_eq!(resp.arguments.tcp_enabled, None);
    assert_eq!(resp.arguments.trash_original_torrent_files, Some(true));

    let units = SessionGetUnits {
        memory_bytes: 1024,
        memory_units: ["KiB", "MiB", "GiB", "TiB"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        size_bytes: 1000,
        size_units: ["kB", "MB", "GB", "TB"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        speed_bytes: 1000,
        speed_units: ["kB/s", "MB/s", "GB/s", "TB/s"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    };
    assert_eq!(resp.arguments.units, Some(units));

    assert_eq!(resp.arguments.utp_enabled, Some(true));
    assert_eq!(resp.arguments.version, Some("3.00 (bb6b5a062e)".to_string()));

    Ok(())
}

// ---------------------------------------------------------------------

#[test]
fn session_get_v411() -> Result<()> {
    // TODO: change to JSON-RPC response
    let resp = serde_json::from_str::<RpcResponse<SessionGet>>(
        r#"
        {
          "arguments": {
            "alt-speed-down": 500,
            "alt-speed-enabled": false,
            "alt-speed-time-begin": 540,
            "alt-speed-time-day": 0,
            "alt-speed-time-enabled": false,
            "alt-speed-time-end": 1020,
            "alt-speed-up": 500,
            "anti-brute-force-enabled": false,
            "anti-brute-force-threshold": 100,
            "blocklist-enabled": false,
            "blocklist-size": 0,
            "blocklist-url": "http://www.example.com/blocklist",
            "cache-size-mb": 16,
            "config-dir": "/config",
            "default-trackers": "",
            "dht-enabled": false,
            "download-dir": "/downloads",
            "download-dir-free-space": 123456789,
            "download-queue-enabled": true,
            "download-queue-size": 2,
            "encryption": "preferred",
            "idle-seeding-limit": 30,
            "idle-seeding-limit-enabled": false,
            "incomplete-dir": "/incomplete",
            "incomplete-dir-enabled": false,
            "lpd-enabled": false,
            "peer-limit-global": 100,
            "peer-limit-per-torrent": 20,
            "peer-port": 55555,
            "peer-port-random-on-start": false,
            "pex-enabled": false,
            "port-forwarding-enabled": false,
            "preferred_transports": [
              "utp",
              "tcp"
            ],
            "queue-stalled-enabled": true,
            "queue-stalled-minutes": 30,
            "rename-partial-files": false,
            "reqq": 2000,
            "rpc-version": 19,
            "rpc-version-minimum": 14,
            "rpc-version-semver": "6.0.1",
            "script-torrent-added-enabled": false,
            "script-torrent-added-filename": "",
            "script-torrent-done-enabled": true,
            "script-torrent-done-filename": "/usr/bin/test",
            "script-torrent-done-seeding-enabled": false,
            "script-torrent-done-seeding-filename": "",
            "seed-queue-enabled": false,
            "seed-queue-size": 500,
            "seedRatioLimit": 2.0,
            "seedRatioLimited": false,
            "sequential_download": false,
            "session-id": "Yl4AAweXcSRDT4vHb7BccFVXFbGa4N75ypQI7hTZ5Lqt6wPO",
            "speed-limit-down": 2000,
            "speed-limit-down-enabled": true,
            "speed-limit-up": 1000,
            "speed-limit-up-enabled": true,
            "start-added-torrents": false,
            "tcp-enabled": true,
            "trash-original-torrent-files": true,
            "units": {
              "memory-bytes": 1024,
              "memory-units": [
                "B",
                "KiB",
                "MiB",
                "GiB",
                "TiB"
              ],
              "size-bytes": 1000,
              "size-units": [
                "B",
                "kB",
                "MB",
                "GB",
                "TB"
              ],
              "speed-bytes": 1000,
              "speed-units": [
                "B/s",
                "kB/s",
                "MB/s",
                "GB/s",
                "TB/s"
              ]
            },
            "utp-enabled": true,
            "version": "4.1.1 (56442e2929)"
          },
          "result": "success"
        }
        "#,
    )?;

    println!("{resp:#?}");
    assert!(resp.is_ok());

    assert_eq!(resp.arguments.alt_speed_down, Some(500));
    assert_eq!(resp.arguments.alt_speed_enabled, Some(false));
    assert_eq!(resp.arguments.alt_speed_time_begin, Some(540));
    assert_eq!(resp.arguments.alt_speed_time_day, Some(AltSpeedDay::empty()));
    assert_eq!(resp.arguments.alt_speed_time_enabled, Some(false));
    assert_eq!(resp.arguments.alt_speed_time_end, Some(1020));
    assert_eq!(resp.arguments.alt_speed_up, Some(500));

    assert_eq!(resp.arguments.anti_brute_force_enabled, Some(false));
    assert_eq!(resp.arguments.anti_brute_force_threshold, Some(100));

    assert_eq!(resp.arguments.blocklist_enabled, Some(false));
    assert_eq!(resp.arguments.blocklist_size, Some(0));
    assert_eq!(resp.arguments.blocklist_url, Some("http://www.example.com/blocklist".to_string()));

    // Supposedly renamed to `cache_size_mib` in 4.1.0, but my `4.1.1 (56442e2929)` instance still
    // returns `cache-size-mb`.
    assert_eq!(resp.arguments.cache_size_mb, Some(16));
    assert_eq!(resp.arguments.config_dir, Some("/config".to_string()));
    assert_eq!(resp.arguments.default_trackers, Some("".to_string()));
    assert_eq!(resp.arguments.dht_enabled, Some(false));

    assert_eq!(resp.arguments.download_dir, Some("/downloads".to_string()));
    assert_eq!(resp.arguments.download_dir_free_space, Some(123456789));
    assert_eq!(resp.arguments.download_queue_enabled, Some(true));
    assert_eq!(resp.arguments.download_queue_size, Some(2));

    assert_eq!(resp.arguments.encryption, Some(Encryption::Preferred));
    assert_eq!(resp.arguments.idle_seeding_limit_enabled, Some(false));
    assert_eq!(resp.arguments.idle_seeding_limit, Some(30));

    assert_eq!(resp.arguments.incomplete_dir_enabled, Some(false));
    assert_eq!(resp.arguments.incomplete_dir, Some("/incomplete".to_string()));

    assert_eq!(resp.arguments.lpd_enabled, Some(false));
    assert_eq!(resp.arguments.peer_limit_global, Some(100));
    assert_eq!(resp.arguments.peer_limit_per_torrent, Some(20));
    assert_eq!(resp.arguments.peer_port_random_on_start, Some(false));
    assert_eq!(resp.arguments.peer_port, Some(55555));
    assert_eq!(resp.arguments.pex_enabled, Some(false));
    assert_eq!(resp.arguments.port_forwarding_enabled, Some(false));
    let transports = ["utp", "tcp"]
        .into_iter()
        .map(str::to_string)
        .collect();
    assert_eq!(resp.arguments.preferred_transports, Some(transports));
    assert_eq!(resp.arguments.queue_stalled_enabled, Some(true));
    assert_eq!(resp.arguments.queue_stalled_minutes, Some(30));
    assert_eq!(resp.arguments.rename_partial_files, Some(false));
    assert_eq!(resp.arguments.reqq, Some(2000));

    assert_eq!(resp.arguments.rpc_version_minimum, Some(14));
    assert_eq!(resp.arguments.rpc_version_semver, Some("6.0.1".to_string()));
    assert_eq!(resp.arguments.rpc_version, Some(19));

    assert_eq!(resp.arguments.script_torrent_added_enabled, Some(false));
    assert_eq!(resp.arguments.script_torrent_added_filename, Some("".to_string()));
    assert_eq!(resp.arguments.script_torrent_done_enabled, Some(true));
    assert_eq!(resp.arguments.script_torrent_done_filename, Some("/usr/bin/test".to_string()));
    assert_eq!(resp.arguments.script_torrent_done_seeding_enabled, Some(false));
    assert_eq!(resp.arguments.script_torrent_done_seeding_filename, Some("".to_string()));

    assert_eq!(resp.arguments.seed_queue_enabled, Some(false));
    assert_eq!(resp.arguments.seed_queue_size, Some(500));
    assert_eq!(resp.arguments.seed_ratio_limit, Some(2.0));
    assert_eq!(resp.arguments.seed_ratio_limited, Some(false));

    assert_eq!(resp.arguments.sequential_download, Some(false));
    let session_id = "Yl4AAweXcSRDT4vHb7BccFVXFbGa4N75ypQI7hTZ5Lqt6wPO".to_string();
    assert_eq!(resp.arguments.session_id, Some(session_id));

    assert_eq!(resp.arguments.speed_limit_down_enabled, Some(true));
    assert_eq!(resp.arguments.speed_limit_down, Some(2000));
    assert_eq!(resp.arguments.speed_limit_up_enabled, Some(true));
    assert_eq!(resp.arguments.speed_limit_up, Some(1000));

    assert_eq!(resp.arguments.start_added_torrents, Some(false));
    assert_eq!(resp.arguments.tcp_enabled, Some(true));
    assert_eq!(resp.arguments.trash_original_torrent_files, Some(true));

    let units = SessionGetUnits {
        memory_bytes: 1024,
        memory_units: ["B", "KiB", "MiB", "GiB", "TiB"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        size_bytes: 1000,
        size_units: ["B", "kB", "MB", "GB", "TB"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        speed_bytes: 1000,
        speed_units: ["B/s", "kB/s", "MB/s", "GB/s", "TB/s"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    };
    assert_eq!(resp.arguments.units, Some(units));

    assert_eq!(resp.arguments.utp_enabled, Some(true));
    assert_eq!(resp.arguments.version, Some("4.1.1 (56442e2929)".to_string()));

    Ok(())
}
