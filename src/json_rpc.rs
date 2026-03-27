use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{RpcResponseArgument, Tag};

/// Represents a [JSON-RPC] request.
///
/// The `M` and `P` generic arguments are to facilitate [`RpcRequest`] pre- and post- semver-6.0.0
/// compatibility.
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
/// [`RpcRequest`]: crate::types::RpcRequest
#[derive(Serialize, Debug)]
pub(crate) struct JsonRpcRequest<'a, M, P> {
    /// "A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    pub(crate) jsonrpc: &'a str,

    /// "A String containing the name of the method to be invoked. Method names that begin with the
    ///  word rpc followed by a period character (U+002E or ASCII 46) are reserved for rpc-internal
    ///  methods and extensions and MUST NOT be used for anything else."
    pub(crate) method: M,

    /// "A Structured value that holds the parameter values to be used during the invocation of the
    ///  method. This member MAY be omitted."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) params: Option<P>,

    /// "An identifier established by the Client that MUST contain a String, Number, or NULL value
    ///  if included. If it is not included it is assumed to be a notification."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<JsonRpcId>,
}

/// Represents a [JSON-RPC] response.
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
#[derive(Deserialize, Debug)]
pub(crate) struct JsonRpcResponse<T: RpcResponseArgument> {
    /// "A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0"."
    pub(crate) jsonrpc: String,

    /// Either a "result" or "error" as defined in [JSON-RPC]
    ///
    /// [JSON-RPC]: <https://www.jsonrpc.org/specification>
    #[serde(flatten)]
    pub(crate) result: JsonRpcResult<T>,

    /// "This member is REQUIRED.
    ///  It MUST be the same as the value of the id member in the Request Object.
    ///  If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid
    ///  Request), it MUST be Null."
    pub(crate) id: Option<JsonRpcId>,
}

/// "An identifier established by the Client that MUST contain a String, Number, or NULL value if
///  included. If it is not included it is assumed to be a notification. The value SHOULD normally
///  not be Null \[1\] and Numbers SHOULD NOT contain fractional parts \[2\]"
///
/// \[1\] "The use of Null as a value for the id member in a Request object is discouraged, because
///      this specification uses a value of Null for Responses with an unknown id. Also, because
///      JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in
///      handling."
///
/// \[2\] "Fractional parts may be problematic, since many decimal fractions cannot be represented
///      exactly as binary fractions."
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub(crate) enum JsonRpcId {
    Number(i64),
    String(String),
}

impl Display for JsonRpcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

impl<'a> Default for JsonRpcId {
    fn default() -> Self {
        Self::Number(0)
    }
}

impl<'a> From<Tag> for JsonRpcId {
    fn from(value: Tag) -> Self {
        Self::Number(value.0)
    }
}

impl From<JsonRpcId> for Tag {
    fn from(value: JsonRpcId) -> Self {
        match value {
            JsonRpcId::Number(id) => Self(id),
            JsonRpcId::String(s) => {
                let tag = i64::default();
                debug!("Converting string JSON-RPC \"id\" into number tag: \"{}\" -> {}", s, tag);
                Self(tag)
            },
        }
    }
}

impl From<i32> for JsonRpcId {
    fn from(value: i32) -> Self {
        Self::Number(value.into())
    }
}

impl From<i64> for JsonRpcId {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<&str> for JsonRpcId {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<&String> for JsonRpcId {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}

impl From<String> for JsonRpcId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

/// Either a [JSON-RPC] "result" or "error".
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum JsonRpcResult<T: RpcResponseArgument> {
    /// "This member is REQUIRED on success.
    ///  This member MUST NOT exist if there was an error invoking the method.
    ///  The value of this member is determined by the method invoked on the Server."
    Result(T),

    /// "This member is REQUIRED on error.
    ///  This member MUST NOT exist if there was no error triggered during invocation."
    Error(JsonRpcError),
}

/// Represents a [JSON-RPC] error response.
///
/// eg.
/// ```
/// {
///     "jsonrpc": "2.0",
///     "error": {
///         "code": 7,
///         "message": "HTTP error from backend service",
///         "data": {
///             "error_string": "Couldn't test port: No Response (0)",
///             "result": {
///                 "ip_protocol": "ipv6"
///             }
///         }
///     },
///     "id": 912313
/// }
/// ```
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct JsonRpcError {
    /// "A Number that indicates the error type that occurred.
    ///  This MUST be an integer."
    ///
    /// "The error codes from and including -32768 to -32000 are reserved for pre-defined errors.
    ///  Any code within this range \[...\] is reserved for future use. The error codes are nearly
    ///  the same as those suggested for XML-RPC at the following url:
    ///  <http://xmlrpc-epi.sourceforge.net/specs/rfc.fault_codes.php>"
    pub(crate) code: i32,

    /// "A String providing a short description of the error.
    /// " The message SHOULD be limited to a concise single sentence."
    pub(crate) message: String,

    /// "A Primitive or Structured value that contains additional information about the error.
    ///  This may be omitted.
    ///  The value of this member is defined by the Server (e.g. detailed error information, nested
    ///  errors etc.)."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<Value>,
}

#[cfg(test)]
mod json_rpc_tests {
    use serde_json::{self, Map, Value};

    use super::*;
    use crate::types::{
        JSON_RPC_VERSION_2_0, Nothing, Result, SessionGet, SessionGetArgs, SessionGetField,
        request::Method,
    };

    #[test]
    fn json_rpc_request_serialize() -> Result<()> {
        let params: Option<SessionGetArgs> = Some([SessionGetField::Version].into());
        let id: Option<JsonRpcId> = Some(912313.into());

        let request = JsonRpcRequest {
            jsonrpc: JSON_RPC_VERSION_2_0,
            method: Method::SessionGet.into_compat(),
            params,
            id: id,
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
               \"id\":912313\
            }");

        Ok(())
    }

    #[test]
    fn json_rpc_response_deserialize() -> Result<()> {
        let response = r#"
        {
           "jsonrpc": "2.0",
           "result": {
              "version": "4.1.0-dev (ae226418eb)"
           },
           "id": 912313
        }
        "#;
        let de_response = serde_json::from_str::<JsonRpcResponse<SessionGet>>(response)?;
        println!("----- response:\n{response}\n");
        println!("----- de_response:\n\n{de_response:#?}\n");

        let expected = JsonRpcResult::Result({
            let mut session_get = SessionGet::default();
            session_get.version = Some("4.1.0-dev (ae226418eb)".to_string());
            session_get
        });
        assert_eq!(de_response.jsonrpc, JSON_RPC_VERSION_2_0);
        assert_eq!(de_response.result, expected);
        assert_eq!(de_response.id, Some(912313.into()));

        Ok(())
    }

    #[test]
    fn json_rpc_error_deserialize() -> Result<()> {
        let error = r#"
        {
          "jsonrpc": "2.0",
          "error": {
            "code": -32600,
            "data": {
              "error_string": "id type must be String, Number, or Null"
            },
            "message": "Invalid Request"
          },
          "id": null
        }
        "#;
        let de_error = serde_json::from_str::<JsonRpcResponse<Nothing>>(error)?;
        println!("----- error:\n{error}\n");
        println!("----- de_error:\n\n{de_error:#?}\n");

        let expected = JsonRpcResult::<Nothing>::Error(JsonRpcError {
            code: -32600,
            message: "Invalid Request".to_string(),
            data: Some(Value::Object({
                let key = "error_string".to_string();
                let val = "id type must be String, Number, or Null".to_string();
                Map::from_iter([(key, Value::String(val))])
            })),
        });
        assert_eq!(de_error.jsonrpc, JSON_RPC_VERSION_2_0);
        assert_eq!(de_error.result, expected);
        assert_eq!(de_error.id, None);

        Ok(())
    }
}

#[cfg(test)]
mod request_tests {
    use super::*;
    use crate::types::{JSON_RPC_VERSION_2_0, Result, RpcRequest, SessionGetField};

    #[test]
    fn rpc_request_json_rpc_serialize() -> Result<()> {
        let args = [SessionGetField::Version].into();
        let mut request = RpcRequest::session_get(Some(args), None);
        request.jsonrpc = Some(JSON_RPC_VERSION_2_0.to_string());

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
        let args = [SessionGetField::Version].into();
        let request = RpcRequest::session_get(Some(args), Some(Tag(-1234)));

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
        let args = [SessionGetField::Version].into();
        let request = RpcRequest::session_get(Some(args), None);

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
}
