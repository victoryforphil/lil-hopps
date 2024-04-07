use crate::{Database, QueryCommand, QueryResponse};

use super::tag_filter::TagFilter;

#[derive(Debug, Clone)]
pub struct GetLatestQuery{
    pub topics: Vec<String>,
    pub ack_topics: Vec<String>,
    pub tag_filters: Vec<TagFilter>,
}

/// Conversion Function from GetLatestQuery to QueryCommand
impl From<GetLatestQuery> for QueryCommand{
    fn from(query: GetLatestQuery) -> QueryCommand{
        QueryCommand::GetLatest(query)
    }
}

impl From<QueryCommand> for GetLatestQuery{
    fn from(command: QueryCommand) -> GetLatestQuery{
        match command{
            QueryCommand::GetLatest(query) => query,
            _ => GetLatestQuery::default()
        }
    }
}

impl Default for GetLatestQuery{
    fn default() -> Self{
        GetLatestQuery{
            topics: Vec::new(),
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        }
    }
}

impl Database{
    pub fn query_get_latest(&mut self, query: GetLatestQuery) -> Result<QueryResponse, String>{
        let mut response = QueryResponse::default();

        for topic in query.topics{
            if let Some(bucket) = self.buckets.get_mut(&topic){
                let data = bucket.get_latest();
                if data.is_none(){
                    continue;
                }
                let data = data.unwrap();
                let mut passed_filters = true;
                for filter in query.tag_filters.iter(){
                    if !filter.is_valid(&data){
                        passed_filters = false;
                        break;
                    }
                }

                if passed_filters{
                    response.data.insert(topic.clone(), data.clone());
                    response.metadata.n_results += 1;
                }
            }
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests{
    use crate::{Primatives, Timestamp, WriteQuery};

    use super::*;

    #[test]
    fn test_write_query_basic(){
        let mut db = Database::new();
        let query1 = WriteQuery::new("test".into(), Primatives::Number(7.0), Timestamp::from_seconds(10.0));
        let query2 = WriteQuery::new("test".into(), Primatives::Number(10.0), Timestamp::from_seconds(5.0));

        let _write_res = db.query_batch(vec![query1.into(), query2.into()]).unwrap();

        let read_query = GetLatestQuery{
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
}