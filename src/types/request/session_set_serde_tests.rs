//! This file defines serde tests for legacy and json-rpc requests.

use serde_json;

use super::*;
use crate::types::{JSON_RPC_VERSION_2_0, Result};

/// Verifies the serialized [`RpcRequest`] matches the `expected` string.
///
/// `jsonrpc` controls the version of the serialized request (eg. pre-semver-6.0.0 or
/// post-semver-6.0.0).
fn verify(args: SessionSetArgs, jsonrpc: Option<&str>, expected: &str) -> Result<()> {
    let mut request = RpcRequest::session_set(args, None);
    request.jsonrpc = jsonrpc.map(str::to_string);
    let ser_request = serde_json::to_string(&request)?;
    println!("{}===== ser_request:\n{ser_request}\n\n",
        match jsonrpc {
            Some(jsonrpc) => format!("[{jsonrpc}] "),
            None => "".to_string(),
        });

    assert_eq!(ser_request, expected);

    Ok(())
}

#[test]
fn request_session_set_legacy_alt_speed_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_down: Some(321),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-down\":321\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_alt_speed_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_down: Some(321),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_down\":321\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_alt_speed_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_alt_speed_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_alt_speed_time_begin() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_begin: Some(123),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-time-begin\":123\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_begin() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_begin: Some(123),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_time_begin\":123\
            }},\
            \"id\":0\
        }}"))
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
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-time-day\":62\
            }\
        }")
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
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_time_day\":62\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_alt_speed_time_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-time-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_time_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_alt_speed_time_end() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_end: Some(666),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-time-end\":666\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_alt_speed_time_end() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_time_end: Some(666),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_time_end\":666\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_alt_speed_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_up: Some(250),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"alt-speed-up\":250\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_alt_speed_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        alt_speed_up: Some(250),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"alt_speed_up\":250\
            }},\
            \"id\":0\
        }}"))
}

/* TODO: needs SessionSetArgs
#[test]
fn request_session_set_semver_600_anti_brute_force_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        anti_brute_force_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"anti_brute_force_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}
*/

#[test]
fn request_session_set_legacy_blocklist_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"blocklist-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_blocklist_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"blocklist_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_blocklist_url() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_url: Some("https://example.com".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"blocklist-url\":\"https://example.com\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_blocklist_url() -> Result<()> {
    let session_set_args = SessionSetArgs {
        blocklist_url: Some("https://example.com".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"blocklist_url\":\"https://example.com\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_cache_size_mb() -> Result<()> {
    let session_set_args = SessionSetArgs {
        cache_size_mb: Some(8),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"cache-size-mb\":8\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_cache_size_mb() -> Result<()> {
    let session_set_args = SessionSetArgs {
        cache_size_mb: Some(8),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"cache_size_mb\":8\
            }},\
            \"id\":0\
        }}"))
}

/* TODO: needs SessionSetArgs
#[test]
fn request_session_set_semver_600_cache_size_mib() -> Result<()> {
    let session_set_args = SessionSetArgs {
        cache_size_mib: Some(32),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"cache_size_mib\":32\
            }},\
            \"id\":0\
        }}"))
}
*/

#[test]
fn request_session_set_legacy_default_trackers() -> Result<()> {
    let session_set_args = SessionSetArgs {
        default_trackers: Some("https://example.com:5555".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"default-trackers\":\"https://example.com:5555\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_default_trackers() -> Result<()> {
    let session_set_args = SessionSetArgs {
        default_trackers: Some("https://example.com:5555".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"default_trackers\":\"https://example.com:5555\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_dht_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        dht_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"dht-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_dht_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        dht_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"dht_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_download_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_dir: Some("/downloads".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"download-dir\":\"/downloads\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_download_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_dir: Some("/downloads".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"download_dir\":\"/downloads\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_download_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"download-queue-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_download_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"download_queue_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_download_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_size: Some(1),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"download-queue-size\":1\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_download_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        download_queue_size: Some(1),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"download_queue_size\":1\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_encryption() -> Result<()> {
    let session_set_args = SessionSetArgs {
        encryption: Some(Encryption::Allowed),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"encryption\":\"tolerated\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_encryption() -> Result<()> {
    let session_set_args = SessionSetArgs {
        encryption: Some(Encryption::Allowed),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"encryption\":\"allowed\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_idle_seeding_limit_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"idle-seeding-limit-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_idle_seeding_limit_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"idle_seeding_limit_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_idle_seeding_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit: Some(4320),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"idle-seeding-limit\":4320\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_idle_seeding_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        idle_seeding_limit: Some(4320),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"idle_seeding_limit\":4320\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_incomplete_dir_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"incomplete-dir-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_incomplete_dir_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"incomplete_dir_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_incomplete_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir: Some("/incomplete".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"incomplete-dir\":\"/incomplete\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_incomplete_dir() -> Result<()> {
    let session_set_args = SessionSetArgs {
        incomplete_dir: Some("/incomplete".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"incomplete_dir\":\"/incomplete\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_lpd_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        lpd_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"lpd-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_lpd_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        lpd_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"lpd_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_peer_limit_global() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_global: Some(120),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"peer-limit-global\":120\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_peer_limit_global() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_global: Some(120),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"peer_limit_global\":120\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_peer_limit_per_torrent() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_per_torrent: Some(10),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"peer-limit-per-torrent\":10\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_peer_limit_per_torrent() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_limit_per_torrent: Some(10),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"peer_limit_per_torrent\":10\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_peer_port_random_on_start() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port_random_on_start: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"peer-port-random-on-start\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_peer_port_random_on_start() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port_random_on_start: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"peer_port_random_on_start\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_peer_port() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port: Some(44556),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"peer-port\":44556\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_peer_port() -> Result<()> {
    let session_set_args = SessionSetArgs {
        peer_port: Some(44556),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"peer_port\":44556\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_pex_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        pex_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"pex-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_pex_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        pex_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"pex_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_port_forwarding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        port_forwarding_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"port-forwarding-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_port_forwarding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        port_forwarding_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"port_forwarding_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

/* TODO: needs SessionSetArgs
#[test]
fn request_session_set_semver_600_preferred_transports() -> Result<()> {
    let session_set_args = SessionSetArgs {
        preferred_transports: Some({
            ["utp", "tcp"]
                .into_iter()
                .map(ToString::to_string)
                .collect()
        }),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"preferred_transports\":["utp","tcp"]\
            }},\
            \"id\":0\
        }}"))
}
*/

#[test]
fn request_session_set_legacy_queue_stalled_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"queue-stalled-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_queue_stalled_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"queue_stalled_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_queue_stalled_minutes() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_minutes: Some(20),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"queue-stalled-minutes\":20\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_queue_stalled_minutes() -> Result<()> {
    let session_set_args = SessionSetArgs {
        queue_stalled_minutes: Some(20),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"queue_stalled_minutes\":20\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_rename_partial_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        rename_partial_files: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"rename-partial-files\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_rename_partial_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        rename_partial_files: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"rename_partial_files\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_reqq() -> Result<()> {
    let session_set_args = SessionSetArgs {
        reqq: Some(2500),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"reqq\":2500\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_reqq() -> Result<()> {
    let session_set_args = SessionSetArgs {
        reqq: Some(2500),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"reqq\":2500\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_script_torrent_added_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"script-torrent-added-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_script_torrent_added_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"script_torrent_added_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_script_torrent_added_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_filename: Some("/scripts/added.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"script-torrent-added-filename\":\"/scripts/added.sh\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_script_torrent_added_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_added_filename: Some("/scripts/added.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"script_torrent_added_filename\":\"/scripts/added.sh\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_script_torrent_done_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"script-torrent-done-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"script_torrent_done_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_script_torrent_done_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_filename: Some("/scripts/done.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"script-torrent-done-filename\":\"/scripts/done.sh\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_filename: Some("/scripts/done.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"script_torrent_done_filename\":\"/scripts/done.sh\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_script_torrent_done_seeding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"script-torrent-done-seeding-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_seeding_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"script_torrent_done_seeding_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_script_torrent_done_seeding_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_filename: Some("/scripts/done_seeding.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"script-torrent-done-seeding-filename\":\"/scripts/done_seeding.sh\"\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_script_torrent_done_seeding_filename() -> Result<()> {
    let session_set_args = SessionSetArgs {
        script_torrent_done_seeding_filename: Some("/scripts/done_seeding.sh".to_string()),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"script_torrent_done_seeding_filename\":\"/scripts/done_seeding.sh\"\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_seed_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"seed-queue-enabled\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_seed_queue_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_enabled: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"seed_queue_enabled\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_seed_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_size: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"seed-queue-size\":1000\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_seed_queue_size() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_queue_size: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"seed_queue_size\":1000\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_seed_ratio_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limit: Some(2.0),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"seedRatioLimit\":2.0\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_seed_ratio_limit() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limit: Some(2.0),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"seed_ratio_limit\":2.0\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_seed_ratio_limited() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limited: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"seedRatioLimited\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_seed_ratio_limited() -> Result<()> {
    let session_set_args = SessionSetArgs {
        seed_ratio_limited: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"seed_ratio_limited\":false\
            }},\
            \"id\":0\
        }}"))
}

/* TODO: needs SessionSetArgs
#[test]
fn request_session_set_semver_600_sequential_download() -> Result<()> {
    let session_set_args = SessionSetArgs {
        sequential_download: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"sequential_download\":false\
            }},\
            \"id\":0\
        }}"))
}
*/

#[test]
fn request_session_set_legacy_speed_limit_down_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"speed-limit-down-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_speed_limit_down_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"speed_limit_down_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_speed_limit_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down: Some(2000),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"speed-limit-down\":2000\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_speed_limit_down() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_down: Some(2000),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"speed_limit_down\":2000\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_speed_limit_up_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"speed-limit-up-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_speed_limit_up_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"speed_limit_up_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_speed_limit_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"speed-limit-up\":1000\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_speed_limit_up() -> Result<()> {
    let session_set_args = SessionSetArgs {
        speed_limit_up: Some(1000),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"speed_limit_up\":1000\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_start_added_torrents() -> Result<()> {
    let session_set_args = SessionSetArgs {
        start_added_torrents: Some(false),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"start-added-torrents\":false\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_start_added_torrents() -> Result<()> {
    let session_set_args = SessionSetArgs {
        start_added_torrents: Some(false),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"start_added_torrents\":false\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_trash_original_torrent_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        trash_original_torrent_files: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"trash-original-torrent-files\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_trash_original_torrent_files() -> Result<()> {
    let session_set_args = SessionSetArgs {
        trash_original_torrent_files: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"trash_original_torrent_files\":true\
            }},\
            \"id\":0\
        }}"))
}

#[test]
fn request_session_set_legacy_utp_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        utp_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, None,
        "{\
            \"method\":\"session-set\",\
            \"arguments\":{\
                \"utp-enabled\":true\
            }\
        }")
}

#[test]
fn request_session_set_semver_600_utp_enabled() -> Result<()> {
    let session_set_args = SessionSetArgs {
        utp_enabled: Some(true),
        ..Default::default()
    };
    verify(session_set_args, Some(JSON_RPC_VERSION_2_0),
        &format!("{{\
            \"jsonrpc\":\"{JSON_RPC_VERSION_2_0}\",\
            \"method\":\"session_set\",\
            \"params\":{{\
                \"utp_enabled\":true\
            }},\
            \"id\":0\
        }}"))
}
