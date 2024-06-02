use crate::{Bucket, DataPoint, Timestamp};

use tracing::debug;
impl Bucket {
    ///Get the latest DataPoint in the bucket
    pub fn get_latest(&self) -> Option<DataPoint> {
        let data_point = self.values.values().last();

        let mut data_point = match data_point {
            Some(dp) => dp.clone(),
            None => {
                return None;
            }
        };

        self.apply_global_tags(&mut data_point);
        Some(data_point)
    }

    pub fn get_latest_mut(&mut self) -> Option<&mut DataPoint> {
        let data_point = self.values.values_mut().last();

        let data_point = match data_point {
            Some(dp) => dp,
            None => {
                return None;
            }
        };

        let global_tags = self.bucket_tags.clone();
        for tag in global_tags.iter() {
            debug!("Adding tag: {:#?} to data point", tag);
            data_point.add_tag(tag.clone());
        }
        Some(data_point)
    }

    pub fn get_earliest(&self) -> Option<DataPoint> {
        let data_point = self.values.values().nth(0);

        let mut data_point = match data_point {
            Some(dp) => dp.clone(),
            None => {
                return None;
            }
        };

        self.apply_global_tags(&mut data_point);
        Some(data_point)
    }

    /// Get the datapoint right before/after the timestamp
    pub fn lookup(&self, timestamp: Timestamp, before: bool) -> Option<DataPoint> {
        let data_point = match before {
            true => self.values.range(..timestamp).next_back(),
            false => self.values.range(timestamp..).next(),
        };

        let mut data_point = match data_point {
            Some(dp) => dp.1.clone(),
            None => {
                return None;
            }
        };

        self.apply_global_tags(&mut data_point);
        Some(data_point)
    }

    /// Get all the datapoints between two timestamps
    pub fn range(&self, start: Timestamp, end: Timestamp) -> Vec<DataPoint> {
        let data_points = self
            .values
            .range(start..end)
            .map(|(_, dp)| dp.clone())
            .collect();
        data_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{types::Tag, Primatives, Timestamp};

    /// Querying tests
    #[test]
    fn test_bucket_get_latest() {
        let mut bucket = Bucket::new("test_topic");
        let mut timestamp = Timestamp::zero();

        // Verify latest is none at start
        let latest_value = bucket.get_latest();
        assert_eq!(latest_value, None);

        for i in 0..11 {
            timestamp = Timestamp::from_seconds(i as f32);
            bucket.add_primative(timestamp, (i as f64).into());
        }

        let latest_value = bucket.get_latest();
        // Check that latest value is now populated
        assert_eq!(latest_value.is_some(), true);
        let latest_value = latest_value.unwrap();

        // Check that the timestamp is correct
        assert_eq!(latest_value.timestamp, Timestamp::from_seconds(10.0));
        assert_eq!(latest_value.data, Primatives::Number(10.0));
    }

    #[test]
    fn test_bucket_get_earliest() {
        let mut bucket = Bucket::new("test_topic");
        let mut timestamp = Timestamp::zero();

        // Verify latest is none at start
        let latest_value = bucket.get_earliest();
        assert_eq!(latest_value, None);

        for i in 0..11 {
            timestamp = Timestamp::from_seconds(i as f32);
            bucket.add_primative(timestamp, (i as f64).into());
        }

        let latest_value = bucket.get_earliest();
        // Check that latest value is now populated
        assert_eq!(latest_value.is_some(), true);
        let latest_value = latest_value.unwrap();

        // Check that the timestamp is correct
        assert_eq!(latest_value.timestamp, Timestamp::from_seconds(0.0));
        assert_eq!(latest_value.data, Primatives::Number(0.0));
    }

    #[test]
    fn test_bucket_lookup() {
        let mut bucket = Bucket::new("test_topic");
        let mut timestamp = Timestamp::zero();

        // Verify latest is none at start
        let latest_value = bucket.lookup(Timestamp::from_seconds(0.0), false);
        assert_eq!(latest_value, None);

        for i in 0..11 {
            timestamp = Timestamp::from_seconds(i as f32);
            bucket.add_primative(timestamp, (i as f64).into());
        }

        let latest_value = bucket.lookup(Timestamp::from_seconds(5.0), false);
        // Check that latest value is now populated
        assert_eq!(latest_value.is_some(), true);
        let latest_value = latest_value.unwrap();

        // Check that the timestamp is correct
        assert_eq!(latest_value.timestamp, Timestamp::from_seconds(5.0));
        assert_eq!(latest_value.data, Primatives::Number(5.0));

        let latest_value = bucket.lookup(Timestamp::from_seconds(5.0), true);
        // Check that latest value is now populated
        assert_eq!(latest_value.is_some(), true);
        let latest_value = latest_value.unwrap();

        // Check that the timestamp is correct
        assert_eq!(latest_value.timestamp, Timestamp::from_seconds(4.0));
        assert_eq!(latest_value.data, Primatives::Number(4.0));
    }

    #[test]
    fn test_bucket_range() {
        let mut bucket = Bucket::new("test_topic");
        let mut timestamp = Timestamp::zero();

        // Verify latest is none at start
        let latest_value =
            bucket.range(Timestamp::from_seconds(0.0), Timestamp::from_seconds(10.0));
        assert_eq!(latest_value.len(), 0);

        for i in 0..11 {
            timestamp = Timestamp::from_seconds(i as f32);
            bucket.add_primative(timestamp, (i as f64).into());
        }

        let latest_value =
            bucket.range(Timestamp::from_seconds(0.0), Timestamp::from_seconds(10.0));
        // Check that latest value is now populated
        assert_eq!(latest_value.len(), 10);
        for (i, dp) in latest_value.iter().enumerate() {
            assert_eq!(dp.timestamp, Timestamp::from_seconds(i as f32));
            assert_eq!(dp.data, Primatives::Number(i as f64));
        }
    }
}
