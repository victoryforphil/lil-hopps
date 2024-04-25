use std::collections::BTreeMap;

use crate::DataPoint;

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
pub use lookup::*;
pub use lookup_range::*;
pub use tag_filter::*;

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

        let unflattened_json = unflatten(&values).unwrap();
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
