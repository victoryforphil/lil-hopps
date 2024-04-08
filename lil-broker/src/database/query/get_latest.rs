use crate::{Database, QueryCommand, QueryResponse};

use super::tag_filter::TagFilter;
use tracing::{info, debug};
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

        let all_bucket_keys = self.get_keys().into_iter();
        let matching_keys = all_bucket_keys.filter(|key| {
            for topic in &query.topics{
                if key.starts_with(topic){
                    return true;
                }
            }
            false
        });
        for bucket_key in matching_keys{
            if let Some(bucket) = self.buckets.get_mut(&bucket_key){
                let data = bucket.get_latest_mut();
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

                // Check to see if we should ack the data
                if query.ack_topics.contains(&bucket_key){
                    debug!("Acking data for topic: {}", &bucket_key);
                    data.add_tag("sys/acknowledged".into());
                }

                if passed_filters{
                    response.data.insert(bucket_key.clone(), data.clone());
                    response.metadata.n_results += 1;
                }
            }
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests{
    use crate::{Primatives, Tag, Timestamp, WriteQuery};

    use super::*;

    #[test]
    fn test_write_query_basic(){
        let mut db = Database::new();
        let query1 = WriteQuery::new("test".into(), 7.0.into(), Timestamp::from_seconds(10.0));
        let query2 = WriteQuery::new("test".into(), 10.0.into(), Timestamp::from_seconds(5.0));

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

    #[test]
    fn test_write_query_wildcard(){
        let mut db = Database::new();
        let query1 = WriteQuery::new("test/a/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));
        let query2 = WriteQuery::new("test/a/2".into(), 2.0.into(), Timestamp::from_seconds(1.0));
        let query3 = WriteQuery::new("test/a/3".into(), 3.0.into(), Timestamp::from_seconds(1.0));
        let query4 = WriteQuery::new("test/b/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));
        

        let _write_res = db.query_batch(vec![query1.into(), query2.into(), query3.into(), query4.into()]).unwrap();

        let read_query = GetLatestQuery{
            topics: vec!["test/a/".into()],
            ack_topics: Vec::new(),
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();

        assert_eq!(read_res.metadata.n_results, 3);
        assert_eq!(read_res.data.len(), 3);

        assert_eq!(read_res.data.keys().into_iter().collect::<Vec<&String>>(), vec!["test/a/1", "test/a/2", "test/a/3"]);
    }

    #[test]
    fn test_write_query_ack(){
        env_logger::init();
        let mut db = Database::new();
        let query1 = WriteQuery::new("test/a/1".into(), 1.0.into(), Timestamp::from_seconds(1.0));
        let _write_res = db.query_batch(vec![query1.into() ]).unwrap();

        let read_query = GetLatestQuery{
            topics: vec!["test/a/".into()],
            ack_topics: vec!["test/a/1".into()],
            tag_filters: Vec::new(),
        };
        let read_res = db.query(read_query.into()).unwrap();
        debug!("Read Response 1: {:#?}", read_res);
        assert_eq!(read_res.metadata.n_results, 1);
        assert_eq!(read_res.data.len(), 1);

        let data = read_res.data.get("test/a/1").unwrap();
        assert_eq!(data.tags.contains(&"sys/acknowledged".into()), true);

        let read_query = GetLatestQuery{
            topics: vec!["test/a/".into()],
            ack_topics: Vec::new(),
            tag_filters: vec![TagFilter::new("sys/acknowledged".into()).exclude()],
        };
    
        let read_res = db.query(read_query.into()).unwrap();
        debug!("Read Res after ack: {:#?}", read_res);
        assert_eq!(read_res.metadata.n_results, 0);
    }
}