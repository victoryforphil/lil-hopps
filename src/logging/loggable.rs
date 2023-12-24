use super::LogEntry;

pub trait Loggable {
    fn log(&self, t: f64) -> Vec<LogEntry>;
}
