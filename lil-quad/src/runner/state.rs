use lil_broker::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UAVRunnerStatus {
    Init,
    Running,
    Completed,
    Error,
}
#[derive(Debug, Clone, Copy)]
pub struct UAVRunnerState {
    pub t: Timestamp,
    pub state: UAVRunnerStatus,
    pub ticks: u64,
}

impl Default for UAVRunnerState {
    fn default() -> Self {
        UAVRunnerState {
            t: Timestamp::from_seconds(0.0),
            state: UAVRunnerStatus::Init,
            ticks: 0,
        }
    }
}

impl UAVRunnerState {
    pub fn new(t: Timestamp, state: UAVRunnerStatus) -> Self {
        UAVRunnerState { t, state, ticks: 0 }
    }
}
