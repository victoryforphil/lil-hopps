use crate::{Database, QueryCommand, QueryResponse};

use super::tag_filter::TagFilter;
use tracing::{debug, info};
#[derive(Debug, Clone)]
pub struct GetLatestQuery {
    pub topics: Vec<String>,
    pub ack_topics: Vec<String>,
    pub tag_filters: Vec<TagFilter>,
}

/// Conversion Function from GetLatestQuery to QueryCommand
impl From<GetLatestQuery> for QueryCommand {
    fn from(query: GetLatestQuery) -> QueryCommand {
        QueryCommand::GetLatest(query)
    }
}

impl From<Vec<String>> for GetLatestQuery {
    fn from(topics: Vec<String>) -> GetLatestQuery {
        GetLatestQuery {
            topics,
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        }
    }
}

impl From<QueryCommand> for GetLatestQuery {
    fn from(command: QueryCommand) -> GetLatestQuery {
        match command {
            QueryCommand::GetLatest(query) => query,
            _ => GetLatestQuery::default(),
        }
    }
}

impl Default for GetLatestQuery {
    fn default() -> Self {
        GetLatestQuery {
            topics: Vec::new(),
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        }
    }
}

impl Database {
    fn read_bucket(
        &mut self,
        matching_keys: Vec<String>,
        query: &GetLatestQuery,
        response: &mut QueryResponse,
    ) -> Option<Vec<String>> {
        let mut addtional_keys = Vec::new();
        for bucket_key in matching_keys {
            if let Some(bucket) = self.buckets.get_mut(&bucket_key) {
                let data = bucket.get_latest_mut();
                if data.is_none() {
                    continue;
                }
                let data = data.unwrap();
                let mut passed_filters = true;
                for filter in query.tag_filters.iter() {
                    if !filter.is_valid(&data) {
                        passed_filters = false;
                        break;
                    }
                }

                // Check to see if we should ack the data
                if query.ack_topics.contains(&bucket_key) {
                    debug!("Acking data for topic: {}", &bucket_key);
                    data.add_tag("sys/acknowledged".into());
                }

                if passed_filters {
                    match &data.data {
                        crate::Primatives::ArrayRef(array_topics) => {
                            for topic in array_topics {
                                if !response.data.contains_key(topic) {
                                    addtional_keys.push(topic.clone());
                                }
                            }
                        }
                        _ => {}
                    }

                    response.data.insert(bucket_key.clone(), vec![data.clone()]);
                    response.metadata.n_results += 1;
                }
            }
        }
        if addtional_keys.len() > 0 {
            return Some(addtional_keys);
        }
        None
    }

    pub fn query_get_latest(&mut self, query: GetLatestQuery) -> Result<QueryResponse, String> {
        let mut response = QueryResponse::default();
        debug!("Querying for latest data: {:?}", query);
        let all_bucket_keys = self.get_keys().into_iter();
        let matching_keys = all_bucket_keys.filter(|key| {
            for topic in &query.topics {
                if key.starts_with(topic) {
                    return true;
                }
            }
            false
        });
        // Read the matching keys
        let addtional_keys = self.read_bucket(matching_keys.collect(), &query, &mut response);

        if let Some(addtional_keys) = addtional_keys {
            debug!("Additional keys to read: {:?}", addtional_keys);
            self.read_bucket(addtional_keys, &query, &mut response);
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Primatives, Tag, Timestamp, WriteQuery};
    use pretty_assertions::{assert_eq, assert_ne};
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_latest_basic() {
        let mut db = Database::new();
        let query1 = WriteQuery::new("test".into(), 7.0.into(), Timestamp::from_seconds(10.0));
        let query2 = WriteQuery::new("test".into(), 10.0.into(), Timestamp::from_seconds(5.0));

        let _write_res = db.query_batch(vec![query1.into(), query2.into()]).unwrap();

        let read_query = GetLatestQuery {
            topics: vec!["test".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();

        assert_eq!(read_res.metadata.n_results, 1);
        assert_eq!(read_res.data.len(), 1);

        let bucket = db.buckets.get("test").unwrap();
        let data = bucket.get_latest();
        assert_eq!(data.unwrap().data, Primatives::Number(7.0));
    }

    #[test]
    fn test_get_latest_json_struct() {
        env_logger::init();
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

        let data = TestData {
            a: 7.0,
            b: true,
            c: "test".to_string(),
            d: TestData2 {
                d: 7.0,
                e: true,
                f: "test".to_string(),
            },
        };

        let queries =
            WriteQuery::from_json_batch(json!(data), Timestamp::new(0), "test".to_string());

        let _write_res = db.query_batch(queries).unwrap();

        let read_query = GetLatestQuery {
            topics: vec!["test".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();

        let json_out = read_res.to_json("test/");
        info!(
            "Json Out: {}",
            serde_json::to_string_pretty(&json_out).unwrap()
        );
        let struct_out = serde_json::from_value::<TestData>(json_out).unwrap();
        assert_eq!(struct_out, data);
    }

    #[test]
    fn test_get_latest_wildcard() {
        let mut db = Database::new();
        let query1 = WriteQuery::new("test/a/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));
        let query2 = WriteQuery::new("test/a/2".into(), 2.0.into(), Timestamp::from_seconds(1.0));
        let query3 = WriteQuery::new("test/a/3".into(), 3.0.into(), Timestamp::from_seconds(1.0));
        let query4 = WriteQuery::new("test/b/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));

        let _write_res = db
            .query_batch(vec![
                query1.into(),
                query2.into(),
                query3.into(),
                query4.into(),
            ])
            .unwrap();

        let read_query = GetLatestQuery {
            topics: vec!["test/a/".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();

        assert_eq!(read_res.metadata.n_results, 3);
        assert_eq!(read_res.data.len(), 3);

        assert_eq!(
            read_res.data.keys().into_iter().collect::<Vec<&String>>(),
            vec!["test/a/1", "test/a/2", "test/a/3"]
        );
    }

    #[test]
    fn test_get_latest_query_ack() {
        env_logger::init();
        let mut db = Database::new();
        let query1 = WriteQuery::new("test/a/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));
        let _write_res = db.query_batch(vec![query1.into()]).unwrap();

        let read_query = GetLatestQuery {
            topics: vec!["test/a/".into()],
            ack_topics: vec!["test/a/1".into()],
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();
        debug!("Read Response 1: {:#?}", read_res);
        assert_eq!(read_res.metadata.n_results, 1);
        assert_eq!(read_res.data.len(), 1);

        let data = read_res.data.get("test/a/1").unwrap();
        let data = data.get(0).unwrap();
        assert_eq!(data.tags.contains(&"sys/acknowledged".into()), true);

        let read_query = GetLatestQuery {
            topics: vec!["test/a/".into()],
            ack_topics: Vec::new(),
            tag_filters: vec![TagFilter::new("sys/acknowledged".into()).exclude()],
        };

        let read_res = db.query(read_query.into()).unwrap();
        debug!("Read Res after ack: {:#?}", read_res);
        assert_eq!(read_res.metadata.n_results, 0);
    }

    #[test]
    fn test_get_latest_query_bucket_tags() {
        env_logger::init();
        let mut db = Database::new();
        let query1 = WriteQuery::new("test/a/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));
        db.add_tag_to_bucket("test/a/1", "user/test_tag".into());
        let _write_res = db.query_batch(vec![query1.into()]).unwrap();

        let read_query = GetLatestQuery {
            topics: vec!["test/a/".into()],
            ack_topics: vec!["test/a/1".into()],
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();
        debug!("Read Response 1: {:#?}", read_res);
        assert_eq!(read_res.metadata.n_results, 1);
        assert_eq!(read_res.data.len(), 1);

        let tags = read_res.data.get("test/a/1").unwrap()[0].tags.clone();
        assert_eq!(tags.contains(&"user/test_tag".into()), true);
    }

    #[test]
    fn test_get_latest_query_additonial() {
        env_logger::init();
        let mut db = Database::new();

        let queries = vec![
            WriteQuery::new(
                "test/a/element_1".into(),
                1.0.into(),
                Timestamp::from_seconds(1.0),
            )
            .into(),
            WriteQuery::new(
                "test/a/element_2".into(),
                2.0.into(),
                Timestamp::from_seconds(1.0),
            )
            .into(),
            WriteQuery::new(
                "test/a/element_3".into(),
                3.0.into(),
                Timestamp::from_seconds(1.0),
            )
            .into(),
            WriteQuery::new(
                "test/a/element_4".into(),
                4.0.into(),
                Timestamp::from_seconds(1.0),
            )
            .into(),
            WriteQuery::new(
                "test/a/parent".into(),
                Primatives::ArrayRef(vec![
                    "test/a/element_1".to_string(),
                    "test/a/element_3".to_string(),
                    "test/a/element_4".to_string(),
                ]),
                Timestamp::from_seconds(1.0),
            )
            .into(),
        ];
        let _write_res = db.query_batch(queries).unwrap();

        let read_query = GetLatestQuery {
            topics: vec!["test/a/parent".into()],
            ack_topics: vec![],
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();
        debug!("Read Response 1: {:#?}", read_res);
        // assert_eq!(read_res.metadata.n_results, 4);
        // assert_eq!(read_res.data.len(), 4);
        assert_eq!(
            vec![
                "test/a/element_1",
                "test/a/element_3",
                "test/a/element_4",
                "test/a/parent",
            ],
            read_res.data.keys().into_iter().collect::<Vec<&String>>(),
        );
    }
}
