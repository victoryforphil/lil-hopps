use std::collections::BTreeMap;

use crate::{DataPoint, Primatives, Timestamp};

mod get_latest;
mod lookup;
mod lookup_range;
mod tag_filter;
mod write;

pub use get_latest::*;
use json_unflattening::unflattening::unflatten;
pub use lookup::*;

pub use lookup_range::*;

use serde_json::{json, Value};
pub use tag_filter::*;

use tracing::debug;
pub use write::*;
#[derive(Debug, Clone)]
pub enum QueryCommand {
    GetLatest(GetLatestQuery),
    Lookup(LookupQuery),
    LookupRange(LookupRangeQuery),
    Write(WriteQuery),
}
#[derive(Debug, Clone)]
pub struct QueryResponseMetadata {
    pub n_results: usize,
    pub was_successful: bool,
}
#[derive(Debug, Clone)]
pub struct QueryResponse {
    pub data: BTreeMap<String, Vec<DataPoint>>,
    pub metadata: QueryResponseMetadata,
}

impl QueryResponse {
    pub fn new() -> QueryResponse {
        QueryResponse {
            data: BTreeMap::new(),
            metadata: QueryResponseMetadata {
                n_results: 0,
                was_successful: false,
            },
        }
    }

    pub fn from_data(data: BTreeMap<String, Vec<DataPoint>>) -> QueryResponse {
        let len = data.len();
        QueryResponse {
            data,
            metadata: QueryResponseMetadata {
                n_results: len,
                was_successful: true,
            },
        }
    }

    pub fn from_json(json: Value) -> QueryResponse {
        let mut data = BTreeMap::new();
        for (key, value) in json.as_object().unwrap() {
            let mut data_points = Vec::new();
            for (timestamp, data) in value.as_object().unwrap() {
                let timestamp = Timestamp::new(timestamp.parse().unwrap());
                let data = Primatives::from_value(data.clone()).unwrap();
                data_points.push(DataPoint::new(timestamp, data));
            }
            data.insert(key.to_string(), data_points);
        }
        QueryResponse::from_data(data)
    }

    pub fn to_json(&self, prefix: &str) -> Value {
        let mut values: serde_json::Map<String, Value> = serde_json::Map::new();
        for (key, data) in self.data.iter() {
            let mut data_points = Vec::new();
            for point in data.iter() {
                data_points.push(point.data.to_value());
            }
            // strip prefix
            let key = key.strip_prefix(prefix).unwrap_or(key);
            values.insert(key.to_string(), json!(data_points.last()));
        }
        // Strip Prefix
        debug!("{:?}", values);

        let unflattened_json = unflatten(&values).unwrap_or_default();
        unflattened_json
    }

    pub fn to_json_timestamped(&self, prefix: &str) -> Value {
        let mut values: serde_json::Map<String, Value> = serde_json::Map::new();
        for (key, data) in self.data.iter() {
            let mut data_points = Vec::new();
            for point in data.iter() {
                data_points.push(point.data.to_value());
            }
            // strip prefix
            let key = key.strip_prefix(prefix).unwrap_or(key);

            let mut data_points = Vec::new();
            for point in data.iter() {
                data_points.push(json!({
                    point.timestamp.tick_ms.to_string(): point.data.to_value()
                }));
            }
            values.insert(key.to_string(), json!(data_points));
        }
        // Strip Prefix

        let unflattened_json = unflatten(&values).unwrap();
        unflattened_json
    }
}

impl Default for QueryResponse {
    fn default() -> Self {
        QueryResponse::new()
    }
}
