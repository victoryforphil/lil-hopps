use std::collections::BTreeMap;

use crate::DataPoint;

use self::get_latest::GetLatestQuery;

mod tag_filter;
mod lookup;
mod lookup_range;
mod get_latest;
mod write;

pub use tag_filter::*;
pub use lookup::*;
pub use lookup_range::*;
pub use get_latest::*;
pub use write::*;
#[derive(Debug, Clone)]
pub enum QueryCommand{
    GetLatest(GetLatestQuery),
    Lookup(LookupQuery),
    LookupRange(LookupRangeQuery),
    Write(WriteQuery),
}
#[derive(Debug, Clone)]
pub struct QueryResponseMetadata{
    pub n_results: usize,
    pub was_successful: bool,
}
#[derive(Debug, Clone)]
pub struct QueryResponse{
    pub data: BTreeMap<String, Vec<DataPoint>>,
    pub metadata: QueryResponseMetadata,
}

impl QueryResponse{
    pub fn new() -> QueryResponse{
        QueryResponse{
            data: BTreeMap::new(),
            metadata: QueryResponseMetadata{
                n_results: 0,
                was_successful: false,
            }
        }
    }
}

impl Default for QueryResponse{
    fn default() -> Self{
        QueryResponse::new()
    }
}