use crate::Timestamp;

use super::tag_filter::TagFilter;

#[derive(Debug, Clone)]
pub struct LookupQuery {
    pub topics: Vec<String>,
    pub timestamp: Timestamp,
    pub ack_topics: Vec<String>,
    pub tag_filters: Vec<TagFilter>,
    pub direction_before: bool,
}

impl Default for LookupQuery {
    fn default() -> Self {
        LookupQuery {
            topics: Vec::new(),
            timestamp: Timestamp::zero(),
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
            direction_before: true,
        }
    }
}
