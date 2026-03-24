use compat_macros::GenerateCompat;
use serde::Serialize;

/// Defines request arguments for the [`group_set`] method.
///
/// > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17).
///
/// # Constructor
///
/// * [`GroupSetArgs::new`] creates an empty `GroupSetArgs` object with the given group name.
///
/// # Setters
///
/// The following methods are fluent setters, returning a new `GroupSetArgs` instance modifying
/// only the corresponding field while leaving all other fields untouched.
///
/// * [`GroupSetArgs::honors_session_limits`]: Whether the session's upload limits are honored.
/// * [`GroupSetArgs::speed_limit_down_enabled`]: Whether the bandwidth group limits download
/// speed.
/// * [`GroupSetArgs::speed_limit_down`]: Maximum download speed (`KBps`).
/// * [`GroupSetArgs::speed_limit_up_enabled`]: Whether the bandwidth group limits upload speed.
/// * [`GroupSetArgs::speed_limit_up`]: Maximum upload speed (`KBps`).
///
/// # Examples
///
/// With fluent setters:
/// ```
/// use transmission_rpc::types::GroupSetArgs;
///
/// let args = GroupSetArgs()::new("my-bandwidth-group".to_owned())
///                .honors_session_limits(false)
///                .speed_limit_up_enabled(true)
///                .speed_limit_up(500);
/// ```
///
/// Directly setting struct fields:
/// ```
/// use transmission_rpc::types::GroupSetArgs;
///
/// let mut args = GroupSetArgs()::new("my-bandwidth-group".to_owned());
/// args.honors_session_limits = Some(false);
/// args.speed_limit_up_enabled = Some(true);
/// args.speed_limit_up = Some(500);
/// ```
///
/// [`group_set`]: crate::TransClient::group_set
#[derive(GenerateCompat, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GroupSetArgs {
    #[serde(skip_serializing_if = "Option::is_none", rename = "honorsSessionLimits")]
    pub honors_session_limits: Option<bool>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up: Option<u64>,
}

impl GroupSetArgs {
    /// Creates an empty `GroupSetArgs` object with every field except for `name` set to `None`.
    pub fn new(name: String) -> Self {
        Self {
            honors_session_limits: None,
            name,
            speed_limit_down_enabled: None,
            speed_limit_down: None,
            speed_limit_up_enabled: None,
            speed_limit_up: None,
        }
    }

    pub fn honors_session_limits(mut self, honors_session_limits: bool) -> Self {
        self.honors_session_limits = Some(honors_session_limits);
        self
    }

    pub fn speed_limit_down_enabled(mut self, speed_limit_down_enabled: bool) -> Self {
        self.speed_limit_down_enabled = Some(speed_limit_down_enabled);
        self
    }

    pub fn speed_limit_down(mut self, speed_limit_down: u64) -> Self {
        self.speed_limit_down = Some(speed_limit_down);
        self
    }

    pub fn speed_limit_up_enabled(mut self, speed_limit_up_enabled: bool) -> Self {
        self.speed_limit_up_enabled = Some(speed_limit_up_enabled);
        self
    }

    pub fn speed_limit_up(mut self, speed_limit_up: u64) -> Self {
        self.speed_limit_up = Some(speed_limit_up);
        self
    }
}
