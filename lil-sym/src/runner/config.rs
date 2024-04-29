use lil_broker::Timestamp;

#[derive(Debug, Clone)]
pub struct SimRunnerConfig {
    pub max_t: Timestamp,
    pub dt: Timestamp,
    pub save_every: Timestamp,
}

/// Configuration for the simulation runner
///
/// # Values
///
/// * `max_t` = `Timestamp::from_seconds(5.0)`
/// * `dt` = `Timestamp::from_seconds(0.01)`
/// * `save_every` = `Timestamp::from_seconds(0.01)`
impl Default for SimRunnerConfig {
    fn default() -> Self {
        SimRunnerConfig {
            max_t: Timestamp::from_seconds(5.0),
            dt: Timestamp::from_seconds(0.01),
            save_every: Timestamp::from_seconds(0.01), // save @ 100hz
        }
    }
}

impl SimRunnerConfig {
    /// Create a new SimRunnerConfig with a maximum time
    ///
    /// # Arguments
    ///
    /// * `max_t: Timestamp` - The maximum time for the simulation
    pub fn new(max_t: Timestamp) -> Self {
        SimRunnerConfig {
            max_t,
            dt: Timestamp::from_seconds(0.01),
            save_every: Timestamp::from_seconds(0.01),
        }
    }

    /// Set the rate (Timestamp) at which UAVs should save their state internally
    ///
    /// # Arguments
    ///
    /// * `save_every: Timestamp` - The rate at which UAVs should save their state
    pub fn save_every(self, save_every: Timestamp) -> Self {
        SimRunnerConfig { save_every, ..self }
    }

    /// Set the time step (dt) for the simulation
    ///
    /// # Arguments
    ///
    /// * `dt: Timestamp` - The time step for the simulation
    pub fn dt(self, dt: Timestamp) -> Self {
        SimRunnerConfig { dt, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = SimRunnerConfig::default();
        assert_eq!(config.max_t, Timestamp::from_seconds(5.0));
        assert_eq!(config.dt, Timestamp::from_seconds(0.01));
        assert_eq!(config.save_every, Timestamp::from_seconds(0.01));
    }

    #[test]
    fn test_new() {
        let config = SimRunnerConfig::new(Timestamp::from_seconds(10.0));
        assert_eq!(config.max_t, Timestamp::from_seconds(10.0));
        assert_eq!(config.dt, Timestamp::from_seconds(0.01));
        assert_eq!(config.save_every, Timestamp::from_seconds(0.01));
    }

    #[test]
    fn test_dt_and_save_every() {
        let config = SimRunnerConfig::new(Timestamp::from_seconds(10.0))
            .dt(Timestamp::from_seconds(0.1))
            .save_every(Timestamp::from_seconds(0.1));
        assert_eq!(config.max_t, Timestamp::from_seconds(10.0));
        assert_eq!(config.dt, Timestamp::from_seconds(0.1));
        assert_eq!(config.save_every, Timestamp::from_seconds(0.1));
    }
}