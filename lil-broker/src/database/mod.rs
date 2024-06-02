mod query;
pub use query::*;
use std::collections::BTreeMap;
use tracing::{debug, error, info};

use crate::{Bucket, Tag};

pub struct Database {
    pub buckets: BTreeMap<String, Bucket>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            buckets: BTreeMap::new(),
        }
    }
    pub fn get_keys(&self) -> Vec<String> {
        self.buckets.keys().cloned().collect()
    }
    pub fn query(&mut self, query: QueryCommand) -> Result<QueryResponse, String> {
        match query {
            QueryCommand::GetLatest(query) => self.query_get_latest(query),
            QueryCommand::Write(query) => self.query_write(query),
            QueryCommand::LookupRange(query) => self.query_lookup_range(query),
            _ => Err("Query not implemented".to_string()),
        }
    }

    pub fn add_tag_to_bucket(&mut self, bucket_name: &str, tag: Tag) {
        //Create a new bucket if it doesn't exist
        if !self.buckets.contains_key(bucket_name) {
            info!("Bucket: {} not found, creating new bucket", bucket_name);
            self.new_bucket(bucket_name);
        }

        if let Some(bucket) = self.buckets.get_mut(bucket_name) {
            bucket.add_tag(tag);
        } else {
            error!("Bucket: {} not found", bucket_name);
        }
    }

    pub fn query_batch(
        &mut self,
        queries: Vec<QueryCommand>,
    ) -> Result<Vec<QueryResponse>, String> {
        let mut responses = Vec::new();
        for query in queries {
            responses.push(self.query(query)?);
        }
        Ok(responses)
    }

    pub fn new_bucket(&mut self, name: &str) {
        self.buckets.insert(name.to_string(), Bucket::new(name));
    }
}
