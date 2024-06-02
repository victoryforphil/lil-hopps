mod primatives;
use std::collections::{BTreeMap, BTreeSet};

use json_unflattening::flattening::flatten;
pub use primatives::Primatives;
use tracing::warn;

use crate::{Tag, Timestamp};

#[derive(Debug, Clone, PartialEq)]
/// Stpres a single data point with a timestamp, data, and tags
/// - timestamp: The time the data point was recorded
/// - data: The data point itself
/// - tags: A set of tags associated with the data point
pub struct DataPoint {
    pub timestamp: Timestamp,
    pub data: Primatives,
    pub tags: BTreeSet<Tag>, // Set of tags hashed by name (value is not checked and should only be read from the tag)
}

impl DataPoint {
    pub fn new(timestamp: Timestamp, data: Primatives) -> DataPoint {
        DataPoint {
            timestamp,
            data,
            tags: BTreeSet::new(),
        }
    }
    pub fn json_to_datapoints(timestamp: Timestamp, json: serde_json::Value) -> BTreeMap<String, DataPoint> {
        let mut map = BTreeMap::new();
        // Flatten JSON
        let json = flatten(&json).unwrap();
        for (key, val) in json {
            if let Some(supported_type) = Primatives::from_value(val.clone()) {
                let dp = DataPoint::new(timestamp.clone(), supported_type);
                map.insert(key, dp);
            }else{
                warn!("Unsupported type for datapoint: {}", val);
            }
        }

        map
    }
    ///Builder function to add a tag to the DataPoint
    pub fn tag(mut self, tag: Tag) -> Self {
        self.tags.insert(tag);
        self
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_datapoint_new() {
        let timestamp = Timestamp::new(0);
        let data = Primatives::Number(0.0);
        let dp = DataPoint::new(timestamp, data.clone());

        assert_eq!(dp.timestamp, timestamp);
        assert_eq!(dp.data, data);
        assert_eq!(dp.tags.len(), 0);
    }

    #[test]
    fn test_datapoint_new_with_tags() {
        let timestamp = Timestamp::new(0);
        let data = Primatives::Number(0.0);
        let dp = DataPoint::new(timestamp, data.clone()).tag("test".into());

        assert_eq!(dp.timestamp, timestamp);
        assert_eq!(dp.data, data);
        assert_eq!(dp.tags.len(), 1);
        assert_eq!(dp.tags.iter().next().unwrap().name, "test");
        assert_eq!(dp.tags.iter().next().unwrap().value, None);
    }
}
