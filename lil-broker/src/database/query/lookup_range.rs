use super::tag_filter::TagFilter;
use crate::{Database, QueryCommand, QueryResponse, Timestamp};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct LookupRangeQuery {
    pub topics: Vec<String>,
    pub timestamp_start: Timestamp,
    pub timestamp_end: Timestamp,
    pub ack_topics: Vec<String>,
    pub tag_filters: Vec<TagFilter>,
    pub direction_before: bool,
}

impl Default for LookupRangeQuery {
    fn default() -> Self {
        LookupRangeQuery {
            topics: Vec::new(),
            timestamp_start: Timestamp::zero(),
            timestamp_end: Timestamp::from_seconds(1.0),
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
            direction_before: true,
        }
    }
}

/// Conversion Function from LookupRangeQuery to QueryCommand
impl From<LookupRangeQuery> for QueryCommand {
    fn from(query: LookupRangeQuery) -> QueryCommand {
        QueryCommand::LookupRange(query)
    }
}

impl From<QueryCommand> for LookupRangeQuery {
    fn from(command: QueryCommand) -> LookupRangeQuery {
        match command {
            QueryCommand::LookupRange(query) => query,
            _ => LookupRangeQuery::default(),
        }
    }
}

impl Database {
    pub fn query_lookup_range(&mut self, query: LookupRangeQuery) -> Result<QueryResponse, String> {
        let mut response = QueryResponse::default();

        let all_bucket_keys = self.get_keys().into_iter();
        let matching_keys = all_bucket_keys.filter(|key| {
            for topic in &query.topics {
                if key.starts_with(topic) {
                    return true;
                }
            }
            false
        });
        for bucket_key in matching_keys {
            if let Some(bucket) = self.buckets.get_mut(&bucket_key) {
                let mut data = bucket.range(query.timestamp_start, query.timestamp_end);

                for data_point in &mut data {
                    let mut passed_filters = true;
                    for filter in query.tag_filters.iter() {
                        if !filter.is_valid(&data_point) {
                            passed_filters = false;
                            break;
                        }
                    }
                    // Check to see if we should ack the data
                    if query.ack_topics.contains(&bucket_key) {
                        debug!("Acking data for topic: {}", &bucket_key);
                        data_point.add_tag("sys/acknowledged".into());
                    }

                    if passed_filters {
                        // Insert if not found else add to the existing data
                        if response.data.contains_key(&bucket_key) {
                            response
                                .data
                                .get_mut(&bucket_key)
                                .unwrap()
                                .push(data_point.clone());
                        } else {
                            response
                                .data
                                .insert(bucket_key.clone(), vec![data_point.clone()]);
                        }
                        response.metadata.n_results += 1;
                    }
                }
            }
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tracing::info;

    use crate::{Timestamp, WriteQuery};

    use super::*;

    pub fn generate_time_data(name: &str) -> Vec<QueryCommand> {
        let mut queries = Vec::new();
        for i in 0..50 {
            let query = WriteQuery::new(
                name.into(),
                (i as f64).into(),
                Timestamp::from_seconds(i as f32),
            );
            queries.push(query.into());
        }
        queries
    }
    fn generate_data() -> Database {
        let mut db = Database::new();

        let topic_a_queries = generate_time_data("test/a");
        let topic_b_queries = generate_time_data("test/b");

        let all = vec![topic_a_queries, topic_b_queries].concat();

        db.query_batch(all).unwrap();

        db
    }
    #[test]
    fn test_lookup_range_basic() {
       
        let mut db = generate_data();

        let read_query = LookupRangeQuery {
            topics: vec!["test/a".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
            timestamp_start: Timestamp::from_seconds(3.0),
            timestamp_end: Timestamp::from_seconds(7.0),
            direction_before: true,
        };
        let read_res = db.query(read_query.into()).unwrap();
        debug!("{:?}", read_res);
        assert_eq!(read_res.metadata.n_results, 4);
        let topic_a = read_res.data.get("test/a").unwrap();
        assert_eq!(topic_a.len(), 4);
    }

    #[test]
    fn test_get_latest_json_struct() {
       
        let mut db = Database::new();

        #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
        struct TestData2 {
            d: f64,
            e: bool,
            f: String,
        }
        #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
        struct TestData {
            a: f64,
            b: bool,
            c: String,
            //#[serde(flatten)]
            d: TestData2,
        }

        let mut data = TestData {
            a: 7.0,
            b: true,
            c: "test".to_string(),
            d: TestData2 {
                d: 7.0,
                e: true,
                f: "test".to_string(),
            },
        };

        let queries = WriteQuery::from_json_batch(
            json!(data),
            Timestamp::from_seconds(5.0),
            "test".to_string(),
        );

        let _write_res = db.query_batch(queries).unwrap();
        data.a = 8.0;
        let queries = WriteQuery::from_json_batch(
            json!(data),
            Timestamp::from_seconds(6.0),
            "test".to_string(),
        );

        let _write_res = db.query_batch(queries).unwrap();

        let read_query = LookupRangeQuery {
            topics: vec!["test".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
            timestamp_start: Timestamp::from_seconds(3.0),
            timestamp_end: Timestamp::from_seconds(7.0),
            direction_before: true,
        };
        let read_res = db.query(read_query.into()).unwrap();

        let json_out = read_res.to_json_timestamped("test/");
        info!(
            "Json Out: {}",
            serde_json::to_string_pretty(&json_out).unwrap()
        );
    }

    #[test]
    fn test_lookup_range_wild_card() {
       
        let mut db = generate_data();

        let read_query = LookupRangeQuery {
            topics: vec!["test".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
            timestamp_start: Timestamp::from_seconds(3.0),
            timestamp_end: Timestamp::from_seconds(7.0),
            direction_before: true,
        };
        let read_res = db.query(read_query.into()).unwrap();
        debug!("{:?}", read_res);
        assert_eq!(read_res.metadata.n_results, 8);
        assert_eq!(read_res.data.len(), 2);
        let topic_a = read_res.data.get("test/a").unwrap();
        assert_eq!(topic_a.len(), 4);
        let topic_b = read_res.data.get("test/b").unwrap();
        assert_eq!(topic_b.len(), 4);
    }
}
