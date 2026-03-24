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
/// The following methods are fluent setters. They consume the `GroupSetArgs` instance, modify the
/// corresponding field, and return the `GroupSetArgs` instance while leaving the other fields
/// untouched.
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
    /// True if session upload limits are honored.
    #[serde(skip_serializing_if = "Option::is_none", rename = "honorsSessionLimits")]
    pub honors_session_limits: Option<bool>,

    /// The bandwidth group name.
    pub name: String,

    /// Max global download speed (kB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down: Option<u64>,

    /// True means enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down_enabled: Option<bool>,

    /// Max global upload speed (kB/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up: Option<u64>,

    /// True means enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up_enabled: Option<bool>,
}

impl GroupSetArgs {
    /// Creates an empty `GroupSetArgs` object with every field except for `name` set to `None`.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            honors_session_limits: None,
            name: name.as_ref().to_string(),
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

#[cfg(test)]
mod setter_tests {
    use super::*;

    #[test]
    fn group_set_args_honors_session_limits() {
        let args = GroupSetArgs::new("slow")
            .honors_session_limits(true);
        assert_eq!(args, {
            let mut expected = GroupSetArgs::new("slow");
            expected.honors_session_limits = Some(true);
            expected
        });
    }

    #[test]
    fn group_set_args_speed_limit_down() {
        let args = GroupSetArgs::new("fast")
            .speed_limit_down(12345);
        assert_eq!(args, {
            let mut expected = GroupSetArgs::new("fast");
            expected.speed_limit_down = Some(12345);
            expected
        });
    }

    #[test]
    fn group_set_args_speed_limit_down_enabled() {
        let args = GroupSetArgs::new("fast")
            .speed_limit_down_enabled(false);
        assert_eq!(args, {
            let mut expected = GroupSetArgs::new("fast");
            expected.speed_limit_down_enabled = Some(false);
            expected
        });
    }

    #[test]
    fn group_set_args_speed_limit_up() {
        let args = GroupSetArgs::new("trickle")
            .speed_limit_up(15);
        assert_eq!(args, {
            let mut expected = GroupSetArgs::new("trickle");
            expected.speed_limit_up = Some(15);
            expected
        });
    }

    #[test]
    fn group_set_args_speed_limit_up_enabled() {
        let args = GroupSetArgs::new("trickle")
            .speed_limit_up_enabled(true);
        assert_eq!(args, {
            let mut expected = GroupSetArgs::new("trickle");
            expected.speed_limit_up_enabled = Some(true);
            expected
        });
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::types::{JSON_RPC_VERSION_2_0, Result, request::test_helper::verify};
    use super::*;

    #[test]
    fn group_set_args_legacy_honors_session_limits() -> Result<()> {
        let args = GroupSetArgs::new("foo")
            .honors_session_limits(true);
        verify(args, None, "\
            \"honorsSessionLimits\":true,\
            \"name\":\"foo\"\
        ")
    }

    #[test]
    fn group_set_args_semver_600_honors_session_limits() -> Result<()> {
        let args = GroupSetArgs::new("foo")
            .honors_session_limits(false);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\
            \"honors_session_limits\":false,\
            \"name\":\"foo\"\
        ")
    }

    #[test]
    fn group_set_args_legacy_speed_limit_down() -> Result<()> {
        let args = GroupSetArgs::new("asdf")
            .speed_limit_down(5000);
        verify(args, None, "\
            \"name\":\"asdf\",\
            \"speed-limit-down\":5000\
        ")
    }

    #[test]
    fn group_set_args_semver_600_speed_limit_down() -> Result<()> {
        let args = GroupSetArgs::new("asdf")
            .speed_limit_down(123);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\
            \"name\":\"asdf\",\
            \"speed_limit_down\":123\
        ")
    }

    #[test]
    fn group_set_args_legacy_speed_limit_down_enabled() -> Result<()> {
        let args = GroupSetArgs::new("lorem")
            .speed_limit_down_enabled(false);
        verify(args, None, "\
            \"name\":\"lorem\",\
            \"speed-limit-down-enabled\":false\
        ")
    }

    #[test]
    fn group_set_args_semver_600_speed_limit_down_enabled() -> Result<()> {
        let args = GroupSetArgs::new("lorem")
            .speed_limit_down_enabled(true);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\
            \"name\":\"lorem\",\
            \"speed_limit_down_enabled\":true\
        ")
    }

    #[test]
    fn group_set_args_legacy_speed_limit_up() -> Result<()> {
        let args = GroupSetArgs::new("ipsum")
            .speed_limit_up(2112);
        verify(args, None, "\
            \"name\":\"ipsum\",\
            \"speed-limit-up\":2112\
        ")
    }

    #[test]
    fn group_set_args_semver_600_speed_limit_up() -> Result<()> {
        let args = GroupSetArgs::new("bar")
            .speed_limit_up(11234);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\
            \"name\":\"bar\",\
            \"speed_limit_up\":11234\
        ")
    }

    #[test]
    fn group_set_args_legacy_speed_limit_up_enabled() -> Result<()> {
        let args = GroupSetArgs::new("misc")
            .speed_limit_up_enabled(true);
        verify(args, None, "\
            \"name\":\"misc\",\
            \"speed-limit-up-enabled\":true\
        ")
    }

    #[test]
    fn group_set_args_semver_600_speed_limit_up_enabled() -> Result<()> {
        let args = GroupSetArgs::new("etc")
            .speed_limit_up_enabled(false);
        verify(args, Some(JSON_RPC_VERSION_2_0), "\
            \"name\":\"etc\",\
            \"speed_limit_up_enabled\":false\
        ")
    }
}
