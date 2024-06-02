use std::{
    fmt::Display,
    ops::{Add, Sub},
    time::Duration,
};

use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, Eq, Ord)]
pub struct Timestamp {
    pub tick_ms: u32,
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp::zero()
    }
}

impl Timestamp {
    pub fn new(tick_ms: u32) -> Timestamp {
        Timestamp { tick_ms }
    }

    pub fn zero() -> Timestamp {
        Timestamp::new(0)
    }

    pub fn from_seconds(seconds: f32) -> Timestamp {
        Timestamp::new((seconds * 1000.0) as u32)
    }

    pub fn from_hz(hz: f32) -> Timestamp {
        Timestamp::new((1000.0 / hz) as u32)
    }

    pub fn hz(&self) -> f32 {
        1000.0 / self.tick_ms as f32
    }

    pub fn seconds(&self) -> f64 {
        self.tick_ms as f64 / 1000.0
    }

    pub fn minutes(&self) -> f64 {
        self.seconds() / 60.0
    }

    pub fn hours(&self) -> f64 {
        self.minutes() / 60.0
    }
}

// Convert from Duration
impl From<Duration> for Timestamp {
    fn from(duration: Duration) -> Self {
        Timestamp::new(duration.as_millis() as u32)
    }
}

// Convert to Duration
impl From<Timestamp> for Duration {
    fn from(timestamp: Timestamp) -> Self {
        Duration::from_millis(timestamp.tick_ms as u64)
    }
}

// Convert from u32
impl From<u32> for Timestamp {
    fn from(tick_ms: u32) -> Self {
        Timestamp::new(tick_ms)
    }
}
// Convert to u32
impl From<Timestamp> for u32 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.tick_ms
    }
}

// Convert from u64
impl From<u64> for Timestamp {
    fn from(tick_ms: u64) -> Self {
        Timestamp::new(tick_ms as u32)
    }
}

// Convert to u64
impl From<Timestamp> for u64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.tick_ms as u64
    }
}

impl Add for Timestamp {
    type Output = Timestamp;

    fn add(self, other: Timestamp) -> Timestamp {
        Timestamp::new(self.tick_ms + other.tick_ms)
    }
}

impl Sub for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: Timestamp) -> Timestamp {
        Timestamp::new(self.tick_ms - other.tick_ms)
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.tick_ms == other.tick_ms
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        self.tick_ms.partial_cmp(&other.tick_ms)
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms", self.tick_ms)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_timestamp_default() {
        let timestamp = Timestamp::default();
        assert_eq!(timestamp.tick_ms, 0);
    }

    #[test]
    fn test_timestamp_new() {
        let timestamp = Timestamp::new(1000);
        assert_eq!(timestamp.tick_ms, 1000);
    }

    #[test]
    fn test_timestamp_from_seconds() {
        let timestamp = Timestamp::from_seconds(1.0);
        assert_eq!(timestamp.tick_ms, 1000);
    }

    #[test]
    fn test_timestamp_from_hz() {
        let timestamp = Timestamp::from_hz(1.0);
        assert_eq!(timestamp.tick_ms, 1000);
    }

    #[test]
    fn test_timestamp_hz() {
        let timestamp = Timestamp::new(1000);
        assert_eq!(timestamp.hz(), 1.0);
    }

    #[test]
    fn test_timestamp_seconds() {
        let timestamp = Timestamp::new(1000);
        assert_eq!(timestamp.seconds(), 1.0);
    }

    #[test]
    fn test_timestamp_minutes() {
        let timestamp = Timestamp::new(60000);
        assert_eq!(timestamp.minutes(), 1.0);
    }

    #[test]

    fn test_timestamp_hours() {
        let timestamp = Timestamp::new(3600000);
        assert_eq!(timestamp.hours(), 1.0);
    }

    #[test]

    fn test_timestamp_from_duration() {
        let timestamp = Timestamp::from(Duration::from_millis(1000));
        assert_eq!(timestamp.tick_ms, 1000);
    }

    #[test]

    fn test_timestamp_to_duration() {
        let timestamp = Timestamp::new(1000);
        assert_eq!(Duration::from(timestamp).as_millis(), 1000);
    }

    #[test]

    fn test_timestamp_from_u32() {
        let timestamp = Timestamp::from(1000_u32);
        assert_eq!(timestamp.tick_ms, 1000);
    }

    #[test]

    fn test_timestamp_to_u32() {
        let timestamp = Timestamp::new(1000);
        assert_eq!(u32::from(timestamp), 1000);
    }

    #[test]

    fn test_timestamp_from_u64() {
        let timestamp = Timestamp::from(1000_u64);
        assert_eq!(timestamp.tick_ms, 1000);
    }

    #[test]

    fn test_timestamp_to_u64() {
        let timestamp = Timestamp::new(1000);
        assert_eq!(u64::from(timestamp), 1000);
    }

    #[test]

    fn test_timestamp_add() {
        let timestamp = Timestamp::new(1000);
        let timestamp2 = Timestamp::new(1000);
        assert_eq!(timestamp + timestamp2, Timestamp::new(2000));
    }

    #[test]

    fn test_timestamp_sub() {
        let timestamp = Timestamp::new(1000);
        let timestamp2 = Timestamp::new(1000);
        assert_eq!(timestamp - timestamp2, Timestamp::new(0));
    }

    #[test]

    fn test_timestamp_eq() {
        let timestamp = Timestamp::new(1000);
        let timestamp2 = Timestamp::new(1000);
        assert_eq!(timestamp, timestamp2);
    }

    #[test]

    fn test_timestamp_partial_ord() {
        let timestamp = Timestamp::new(1000);
        let timestamp2 = Timestamp::new(1000);
        assert_eq!(
            timestamp.partial_cmp(&timestamp2),
            Some(std::cmp::Ordering::Equal)
        );
    }

    #[test]

    fn test_timestamp_partial_ord_greater() {
        let timestamp = Timestamp::new(2000);
        let timestamp2 = Timestamp::new(1000);
        assert_eq!(
            timestamp.partial_cmp(&timestamp2),
            Some(std::cmp::Ordering::Greater)
        );
    }

    #[test]

    fn test_timestamp_partial_ord_less() {
        let timestamp = Timestamp::new(1000);
        let timestamp2 = Timestamp::new(2000);
        assert_eq!(
            timestamp.partial_cmp(&timestamp2),
            Some(std::cmp::Ordering::Less)
        );
    }
}
