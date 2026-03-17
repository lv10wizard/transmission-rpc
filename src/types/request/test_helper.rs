//! This file defines helper functions for tests.

use crate::types::Result;
use super::RpcRequest;

/// Verifies the serialized [`RpcRequest`] matches the `expected` string.
///
/// `jsonrpc` controls the version of the serialized request (eg. pre-semver-6.0.0 or
/// post-semver-6.0.0).
pub(crate) fn verify<T>(args: T, jsonrpc: Option<&str>, expected: &str)
    -> Result<()>
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

    assert_eq!(ser_request, expected);

    Ok(())
}
