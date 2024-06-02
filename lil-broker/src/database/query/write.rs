use crate::{Bucket, DataPoint, Database, Primatives, QueryCommand, QueryResponse, Tag, Timestamp};

use flatten_json_object::{ArrayFormatting, Flattener};
use json_unflattening::flattening::flatten;
use serde::de::value;
use serde_json::Value;
use tracing::{error, info};
#[derive(Debug, Clone)]
pub struct WriteQuery {
    pub topic: String,
    pub data: Primatives,
    pub timestamp: Timestamp,
    pub tags: Vec<Tag>,
}

impl WriteQuery {
    pub fn new(topic: String, data: Primatives, timestamp: Timestamp) -> WriteQuery {
        WriteQuery {
            topic,
            data,
            timestamp,
            tags: Vec::new(),
        }
    }

    pub fn from_json_batch(json: Value, timestamp: Timestamp, prefix: String) -> Vec<QueryCommand> {
        let mut queries = Vec::new();

        let out = flatten(&json).unwrap();

        for (key, val) in out.iter() {
            if let Some(supported_type) = Primatives::from_value(val.clone()) {
                // can only be 1 unique item per mapped value.
                let write_query =
                    WriteQuery::new(format!("{}/{}", prefix, key), supported_type, timestamp);
                queries.push(write_query.into());
            }
        }
        queries
    }
}
// Conversion from WriteQuery to QueryCommands::Write(WriteQuery)
impl From<WriteQuery> for QueryCommand {
    fn from(query: WriteQuery) -> QueryCommand {
        QueryCommand::Write(query)
    }
}

impl From<QueryCommand> for WriteQuery {
    fn from(query: QueryCommand) -> WriteQuery {
        match query {
            QueryCommand::Write(query) => query,
            _ => panic!("Not a WriteQuery"),
        }
    }
}

impl Database {
    pub fn query_write(&mut self, query: WriteQuery) -> Result<QueryResponse, String> {
        let mut response = QueryResponse::default();

        // If no bucket exists create one
        if !self.buckets.contains_key(&query.topic) {
            self.buckets
                .insert(query.topic.clone(), Bucket::new(&query.topic));
            info!("Created new bucket for topic: {}", query.topic);
        }

        if let Some(bucket) = self.buckets.get_mut(&query.topic) {
            let datapoint = DataPoint::new(query.timestamp, query.data);
            bucket.add_data_point(datapoint.clone());
            if response.data.contains_key(&query.topic) {
                response.data.get_mut(&query.topic).unwrap().push(datapoint);
            } else {
                response.data.insert(query.topic.clone(), vec![datapoint]);
            }

            response.metadata.n_results += 1;
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    #[test]
    fn test_write_query_basic() {
        let mut db = Database::new();
        let query = WriteQuery::new("test".into(), Primatives::Number(7.0), Timestamp::new(0));

        let response = db.query(query.into()).unwrap();

        assert_eq!(response.metadata.n_results, 1);
        assert_eq!(response.data.len(), 1);

        let bucket = db.buckets.get("test").unwrap();
        let data = bucket.get_latest();
        assert_eq!(data.unwrap().data, Primatives::Number(7.0));
    }

    #[test]
    fn test_write_query_from_json_simple() {
        let json = json!({
            "test": 7.0
        });
        let queries = WriteQuery::from_json_batch(json, Timestamp::new(0), "test".to_string());
        assert_eq!(queries.len(), 1);
        match &queries[0] {
            QueryCommand::Write(query) => {
                assert_eq!(query.topic, "test/test");
                assert_eq!(query.data, Primatives::Number(7.0));
            }
            _ => panic!("Not a WriteQuery"),
        }
    }

    #[test]
    fn test_write_query_from_json_complex() {
        env_logger::init();

        let json = json!(
        {
            "test": 7.0,
            "test2": "test",
            "test3": [1,2,3],
            "test4": [1.0, 2.0, 3.0],
            "test5": [true, false, true],
            "test_nested": {"test6": 7.0, "test7": "test"}
        });
        let queries = WriteQuery::from_json_batch(json, Timestamp::new(0), "test".to_string());
        info!("{:#?}", queries);
        assert_eq!(queries.len(), 13);

        match &queries[0] {
            QueryCommand::Write(query) => {
                assert_eq!(query.topic, "test/test");
                assert_eq!(query.data, Primatives::Number(7.0));
            }
            _ => panic!("Not a WriteQuery"),
        }
    }
}
