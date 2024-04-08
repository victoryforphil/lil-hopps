mod query;
use std::collections::BTreeMap;

pub use query::*;

use crate::Bucket;


pub struct Database{
    pub buckets: BTreeMap<String, Bucket>
}


impl Database{
    pub fn new() -> Database{
        Database{
            buckets: BTreeMap::new()
        }
    }
    pub fn get_keys(&self) -> Vec<String>{
        self.buckets.keys().cloned().collect()
    }
    pub fn query(&mut self, query: QueryCommand) -> Result<QueryResponse, String>{
       match query{
           QueryCommand::GetLatest(query) => self.query_get_latest(query),
           QueryCommand::Write(query) => self.query_write(query),
           _ => Err("Query not implemented".to_string())
       }

    }

    pub fn query_batch(&mut self, queries: Vec<QueryCommand>) -> Result<Vec<QueryResponse>, String>{
        let mut responses = Vec::new();
        for query in queries{
            responses.push(self.query(query)?);
        }
        Ok(responses)
    }

    pub fn new_bucket(&mut self, name: &str){
        self.buckets.insert(name.to_string(), Bucket::new(name));
    }
}