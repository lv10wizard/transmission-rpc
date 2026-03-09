use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::JSON_RPC_VERSION_2_0;

/// Represents a [JSON-RPC] request.
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
#[derive(Serialize)]
pub(crate) struct JsonRpcRequest<'a, T> {
    /// "A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    pub(crate) jsonrpc: &'a str,

    /// "A String containing the name of the method to be invoked. Method names that begin with the
    ///  word rpc followed by a period character (U+002E or ASCII 46) are reserved for rpc-internal
    ///  methods and extensions and MUST NOT be used for anything else."
    pub(crate) method: &'a str,

    /// "A Structured value that holds the parameter values to be used during the invocation of the
    ///  method. This member MAY be omitted."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) params: &'a Option<T>,

    /// "An identifier established by the Client that MUST contain a String, Number, or NULL value
    ///  if included. If it is not included it is assumed to be a notification."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<JsonRpcId<'a>>,
}

/// Represents a [JSON-RPC] response.
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
#[derive(Deserialize)]
pub(crate) struct JsonRpcResponse<'a> {
    /// "A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0"."
    jsonrpc: &'a str,

    /// Either a "result" or "error" as defined in [JSON-RPC]
    ///
    /// [JSON-RPC]: <https://www.jsonrpc.org/specification>
    #[serde(flatten)]
    result: JsonRpcResult,

    /// "This member is REQUIRED.
    ///  It MUST be the same as the value of the id member in the Request Object.
    ///  If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid
    ///  Request), it MUST be Null."
    id: Option<JsonRpcId<'a>>,
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
pub(crate) enum JsonRpcId<'a> {
    Number(i64),
    String(&'a str),
}

impl<'a> Default for JsonRpcId<'a> {
    fn default() -> Self {
        Self::Number(0)
    }
}

/// Either a [JSON-RPC] "result" or "error".
///
/// [JSON-RPC]: <https://www.jsonrpc.org/specification>
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum JsonRpcResult {
    /// "This member is REQUIRED on success.
    ///  This member MUST NOT exist if there was an error invoking the method.
    ///  The value of this member is determined by the method invoked on the Server."
    Result(Value),

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
