#[derive(Debug, Clone)]
pub struct SimRunnerOptions {
    pub max_t: f64,
    pub dt: f64,
    pub threaded: bool,
    pub join: bool,
    pub send_every: usize,
}

impl SimRunnerOptions {
    /// Create a new SimRunnerOptions with threaded and join set to true
    ///
    /// Defaults:
    /// - max_t: 10.0
    /// - dt: 0.01
    /// - threaded: true
    /// - join: true
    pub fn new_threaded(max_t: f64) -> Self {
        SimRunnerOptions {
            max_t: max_t,
            dt: 0.005,
            threaded: true,
            join: true,
            send_every: 5,
        }
    }

    /// Create a new SimRunnerOptions with threaded set to true and join set to false
    ///
    /// Defaults:
    /// - max_t: 10.0
    /// - dt: 0.01
    /// - threaded: true
    /// - join: false
    pub fn new_unjoined(max_t: f64) -> Self {
        SimRunnerOptions {
            max_t: max_t,
            dt: 0.005,
            threaded: true,
            join: false,
            send_every: 5,
        }
    }

    /// Create a new SimRunnerOptions with threaded and join set to false
    ///     
    /// Defaults:
    /// - max_t: 10.0
    /// - dt: 0.01
    /// - threaded: false
    /// - join: false
    pub fn new(max_t: f64) -> Self {
        SimRunnerOptions {
            max_t: max_t,
            dt: 0.005,
            threaded: false,
            join: false,
            send_every: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_threaded() {
        let options = SimRunnerOptions::new_threaded(10.0);
        assert_eq!(options.max_t, 10.0);
        assert_eq!(options.dt, 0.005);
        assert_eq!(options.threaded, true);
        assert_eq!(options.join, true);
    }

    #[test]
    fn test_new_unjoined() {
        let options = SimRunnerOptions::new_unjoined(10.0);
        assert_eq!(options.max_t, 10.0);
        assert_eq!(options.dt, 0.005);
        assert_eq!(options.threaded, true);
        assert_eq!(options.join, false);
    }

    #[test]
    fn test_new() {
        let options = SimRunnerOptions::new(10.0);
        assert_eq!(options.max_t, 10.0);
        assert_eq!(options.dt, 0.005);
        assert_eq!(options.threaded, false);
        assert_eq!(options.join, false);
    }
}
