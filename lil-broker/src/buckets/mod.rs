use crate::{DataPoint, Primatives, Tag, Timestamp};
use std::collections::{BTreeMap, BTreeSet};
use tracing::debug;

mod querying;

pub struct Bucket {
    pub name: String,
    pub bucket_tags: BTreeSet<Tag>,
    pub values: BTreeMap<Timestamp, DataPoint>,
}

impl Bucket {
    pub fn new(name: &str) -> Bucket {
        Bucket {
            name: name.to_string(),
            bucket_tags: BTreeSet::new(),
            values: BTreeMap::new(),
        }
    }

    pub fn add_primative(&mut self, timestamp: Timestamp, value: Primatives) -> &mut Self {
        let data_point = DataPoint::new(timestamp, value);
        self.values.insert(timestamp, data_point);
        self
    }

    pub fn add_tag(&mut self, tag: Tag) -> &mut Self {
        debug!("Adding tag: {:#?} to bucket: {}", tag, self.name);
        self.bucket_tags.insert(tag);
        self
    }

    pub fn add_data_point(&mut self, data_point: DataPoint) -> &mut Self {
        self.values.insert(data_point.timestamp, data_point);
        self
    }

    fn apply_global_tags(&self, data_point: &mut DataPoint) {
        for tag in self.bucket_tags.iter() {
            data_point.add_tag(tag.clone());
        }
    }
}
