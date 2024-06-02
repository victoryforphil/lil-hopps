use lil_broker::Timestamp;

#[derive(Debug, Clone)]
pub struct UAVRunnerConfig {
    pub max_t: Timestamp,
    pub dt: Timestamp,
    pub external_tick: bool,
    pub wait: bool,
}

/// Configuration for the UAVulation runner
///
/// # Values
///
/// * `max_t` = `Timestamp::from_seconds(5.0)`
/// * `dt` = `Timestamp::from_seconds(0.01)`
/// * `external_tick` = `false`
impl Default for UAVRunnerConfig {
    fn default() -> Self {
        UAVRunnerConfig {
            max_t: Timestamp::from_seconds(5.0),
            dt: Timestamp::from_seconds(0.01),
            external_tick: false,
            wait: false,
        }
    }
}

impl UAVRunnerConfig {
    /// Create a new UAVRunnerConfig with a maximum time
    ///
    /// # Arguments
    ///
    /// * `max_t: Timestamp` - The maximum time for the UAVulation
    pub fn new(max_t: Timestamp) -> Self {
        UAVRunnerConfig {
            max_t,
            dt: Timestamp::from_seconds(0.01),
            external_tick: false,
            wait: false,
        }
    }

    /// Set the time step (dt) for the UAVulation
    ///
    /// # Arguments
    ///
    /// * `dt: Timestamp` - The time step for the UAVulation
    pub fn dt(self, dt: Timestamp) -> Self {
        UAVRunnerConfig { dt, ..self }
    }

    /// Set the UAV to use external tick
    pub fn set_external_tick(self) -> Self {
        UAVRunnerConfig {
            external_tick: true,
            ..self
        }
    }

    /// Set the UAV to wait for external tick
    pub fn set_wait(self) -> Self {
        UAVRunnerConfig { wait: true, ..self }
    }

    pub fn set_max_t(self, max_t: Timestamp) -> Self {
        UAVRunnerConfig { max_t, ..self }
    }

    pub fn set_dt(self, dt: Timestamp) -> Self {
        UAVRunnerConfig { dt, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = UAVRunnerConfig::default();
        assert_eq!(config.max_t, Timestamp::from_seconds(5.0));
        assert_eq!(config.dt, Timestamp::from_seconds(0.01));
        assert_eq!(config.external_tick, false);
    }

    #[test]
    fn test_new() {
        let config = UAVRunnerConfig::new(Timestamp::from_seconds(10.0));
        assert_eq!(config.max_t, Timestamp::from_seconds(10.0));
        assert_eq!(config.dt, Timestamp::from_seconds(0.01));
        assert_eq!(config.external_tick, false);
    }

    #[test]
    fn test_dt_and_set_external_tick() {
        let config = UAVRunnerConfig::new(Timestamp::from_seconds(10.0))
            .dt(Timestamp::from_seconds(0.1))
            .set_external_tick();
        assert_eq!(config.max_t, Timestamp::from_seconds(10.0));
        assert_eq!(config.dt, Timestamp::from_seconds(0.1));
        assert_eq!(config.external_tick, true);
    }
}
