use super::tag_filter::TagFilter;

#[derive(Debug, Clone)]
pub struct GetLatestQuery{
    pub topics: Vec<String>,
    pub ack_topics: Vec<String>,
    pub tag_filters: Vec<TagFilter>,
}

impl Default for GetLatestQuery{
    fn default() -> Self{
        GetLatestQuery{
            topics: Vec::new(),
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        }
    }
}