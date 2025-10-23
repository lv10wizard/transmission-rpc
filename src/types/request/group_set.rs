use super::GroupSetArgs;

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
