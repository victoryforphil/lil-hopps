use lil_broker::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimRunnerStatus {
    Init,
    Running,
    Completed,
    Error,
}
#[derive(Debug, Clone)]
pub struct SimRunnerState {
    pub t: Timestamp,
    pub state: SimRunnerStatus,
}

impl Default for SimRunnerState {
    fn default() -> Self {
        SimRunnerState {
            t: Timestamp::from_seconds(0.0),
            state: SimRunnerStatus::Init,
        }
    }
}

impl SimRunnerState {
    pub fn new(t: Timestamp, state: SimRunnerStatus) -> Self {
        SimRunnerState { t, state }
    }
}