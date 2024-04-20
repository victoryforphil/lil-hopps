use crate::{ Bucket, DataPoint, Database, Primatives, QueryCommand, QueryResponse, Tag, Timestamp};


use tracing::info;
#[derive(Debug, Clone)]
pub struct WriteQuery{
    pub topic: String,
    pub data: Primatives,
    pub timestamp: Timestamp,
    pub tags: Vec<Tag>,
}

impl WriteQuery{
    pub fn new(topic: String, data: Primatives, timestamp: Timestamp) -> WriteQuery{
        WriteQuery{
            topic,
            data,
            timestamp,
            tags: Vec::new(),
        }
    }

}
// Conversion from WriteQuery to QueryCommands::Write(WriteQuery)
impl From<WriteQuery> for QueryCommand{
    fn from(query: WriteQuery) -> QueryCommand{
        QueryCommand::Write(query)
    }
}

impl From<QueryCommand> for WriteQuery{
    fn from(query: QueryCommand) -> WriteQuery{
        match query{
            QueryCommand::Write(query) => query,
            _ => panic!("Not a WriteQuery"),
        }
    }
}

impl Database{
    pub fn query_write(&mut self, query: WriteQuery) -> Result<QueryResponse, String>{
        let mut response = QueryResponse::default();

        // If no bucket exists create one 
        if !self.buckets.contains_key(&query.topic){
            self.buckets.insert(query.topic.clone(), Bucket::new(&query.topic));
            info!("Created new bucket for topic: {}", query.topic);
        }

        if let Some(bucket) = self.buckets.get_mut(&query.topic){
            let datapoint = DataPoint::new(query.timestamp, query.data);
            bucket.add_data_point(datapoint.clone());
            if response.data.contains_key(&query.topic){
                response.data.get_mut(&query.topic).unwrap().push(datapoint);
            }else{ 
                response.data.insert(query.topic.clone(), vec![datapoint]);
            }
         
            response.metadata.n_results += 1;
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_write_query_basic(){
        let mut db = Database::new();
        let query = WriteQuery::new("test".into(), Primatives::Number(7.0), Timestamp::new(0));

        let response = db.query(query.into()).unwrap();

        assert_eq!(response.metadata.n_results, 1);
        assert_eq!(response.data.len(), 1);
      
        let bucket = db.buckets.get("test").unwrap();
        let data = bucket.get_latest();
        assert_eq!(data.unwrap().data, Primatives::Number(7.0));
    }
}