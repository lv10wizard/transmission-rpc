//! This file defines helper functions for tests.

use serde_json;

use crate::types::Result;
use super::RpcRequest;

/// Verifies that the [`RpcRequest`] serialized with `args` matches the `expected_args` string.
///
/// `jsonrpc` controls the version of the serialized request (eg. pre-semver-6.0.0 or
/// post-semver-6.0.0).
pub(crate) fn verify<T>(args: T, jsonrpc: Option<&str>, expected_args: &str) -> Result<()>
where
    T: Into<RpcRequest>,
{
    let mut request: RpcRequest = args.into();
    request.jsonrpc = jsonrpc.map(str::to_string);

    let ser_request = serde_json::to_string(&request)?;
    println!("{}===== ser_request:\n{ser_request}\n\n",
        match jsonrpc {
            Some(jsonrpc) => format!("[{jsonrpc}] "),
            None => "".to_string(),
        });

    let expected = match jsonrpc {
        // Legacy request.
        None => {
            let method = &request.method;
            format!("{{\
                \"method\":{method},\
                \"arguments\":{{\
                    {expected_args}\
                }}\
            }}")
        },

        // JSON-RPC (post- semver-6.0.0) request.
        Some(version) => {
            // semver 6.0.0+ strings should be snake_case.
            let method = request.method.into_compat();
            format!("{{\
                \"jsonrpc\":\"{version}\",\
                \"method\":{method},\
                \"params\":{{\
                    {expected_args}\
                }},\
                \"id\":0\
            }}")
        },
    };

    assert_eq!(ser_request, expected);

    Ok(())
}
