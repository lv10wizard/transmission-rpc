use serde_json;

use super::*;
use crate::types::{JSON_RPC_VERSION_2_0, Result, SessionGetArgs, SessionGetField, Tag};

#[test]
fn rpc_request_json_rpc_serialize() -> Result<()> {
    let args: SessionGetArgs = [SessionGetField::Version].into();
    let request = RpcRequest {
        method: Method::SessionGet,
        arguments: Some(args.into()),
        tag: None,
        jsonrpc: Some(JSON_RPC_VERSION_2_0.to_string()),
    };

    let ser_request = serde_json::to_string(&request)?;
    println!("----- request:\n\n{ser_request}\n");

    assert_eq!(ser_request, 
        "{\
           \"jsonrpc\":\"2.0\",\
           \"method\":\"session_get\",\
           \"params\":{\
             \"fields\":[\
               \"version\"\
             ]\
           },\
           \"id\":0\
        }");

    Ok(())
}

#[test]
fn rpc_request_legacy_tagged_serialize() -> Result<()> {
    let args: SessionGetArgs = [SessionGetField::Version].into();
    let request = RpcRequest {
        method: Method::SessionGet,
        arguments: Some(args.into()),
        tag: Some(Tag(-1234)),
        jsonrpc: None,
    };

    let ser_request = serde_json::to_string(&request)?;
    println!("----- request:\n\n{ser_request}\n");

    assert_eq!(ser_request, 
        "{\
           \"method\":\"session-get\",\
           \"arguments\":{\
             \"fields\":[\
               \"version\"\
             ]\
           },\
           \"tag\":-1234\
        }");

    Ok(())
}

#[test]
fn rpc_request_legacy_no_tag_serialize() -> Result<()> {
    let args: SessionGetArgs = [SessionGetField::Version].into();
    let request = RpcRequest {
        method: Method::SessionGet,
        arguments: Some(args.into()),
        tag: None,
        jsonrpc: None,
    };

    let ser_request = serde_json::to_string(&request)?;
    println!("----- request:\n\n{ser_request}\n");

    assert_eq!(ser_request, 
        "{\
           \"method\":\"session-get\",\
           \"arguments\":{\
             \"fields\":[\
               \"version\"\
             ]\
           }\
        }");

    Ok(())
}
