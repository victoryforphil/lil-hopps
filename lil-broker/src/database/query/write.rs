use crate::{Primatives, Timestamp};

use super::tag_filter::TagFilter;

#[derive(Debug, Clone)]
pub struct WriteQuery{
    pub topic: String,
    pub data: Primatives.
    pub timestamp: Timestamp,
    pub tags: Vec<Tag>,
}

impl WriteQuery{
    pub fn new(topic: String, data: Primatives, timestamp: Timestamp) -> WriteQuery{
        WriteQuery{
            topic,
            data,
            timestamp,
            tags: Vec::new(),
        }
    }

}