use super::tag_filter::TagFilter;

#[derive(Debug, Clone)]
pub struct LookupRangeQuery{
    pub topics: Vec<String>,
    pub timestamp_start: Timestamp,
    pub timestamp_end: Timestamp,
    pub ack_topics: Vec<String>,
    pub tag_filters: Vec<TagFilter>,
    pub direction_before: bool,
}

impl Default for LookupQuery{
    fn default() -> Self{
        LookupQuery{
            topics: Vec::new(),
            timestamp: Timestamp::zero(),
            timestamp_end: Timestamp::seconds(1.0),
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
            direction_before: true,
        }
    }
}