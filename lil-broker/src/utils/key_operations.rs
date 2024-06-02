use std::collections::BTreeMap;

use crate::Primatives;

pub struct BrokerKey {
    pub string: String,
}

impl BrokerKey {
    pub fn new(string: String) -> Self {
        Self { string }
    }
    pub fn prefix(&mut self, prefix: &str) {
        self.string = format!("{}/{}", prefix, self.string);
    }

    pub fn append(&mut self, append: &str) {
        self.string = format!("{}/{}", self.string, append);
    }

    pub fn prefix_batch(
        batch: BTreeMap<String, Primatives>,
        prefix: &str,
    ) -> BTreeMap<String, Primatives> {
        let mut new_batch = BTreeMap::new();
        for (key, value) in batch {
            let new_key = format!("{}/{}", prefix, key);
            new_batch.insert(new_key, value);
        }
        new_batch
    }
}

impl From<&str> for BrokerKey {
    fn from(s: &str) -> Self {
        Self {
            string: s.to_string(),
        }
    }
}

impl From<String> for BrokerKey {
    fn from(s: String) -> Self {
        Self { string: s }
    }
}

impl From<BrokerKey> for String {
    fn from(bk: BrokerKey) -> String {
        bk.string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_broker_key_new() {
        let key = BrokerKey::new("test".to_string());
        assert_eq!(key.string, "test");
    }

    #[test]
    fn test_broker_key_prefix() {
        let mut key = BrokerKey::new("test".to_string());
        key.prefix("prefix");
        assert_eq!(key.string, "prefix/test");
    }

    #[test]
    fn test_broker_key_append() {
        let mut key = BrokerKey::new("test".to_string());
        key.append("append");
        assert_eq!(key.string, "test/append");
    }

    #[test]
    fn test_broker_key_prefix_batch() {
        let mut batch = BTreeMap::new();
        batch.insert("key1".to_string(), Primatives::Number(1.0));
        batch.insert("key2".to_string(), Primatives::Number(2.0));
        let new_batch = BrokerKey::prefix_batch(batch, "prefix");
        let mut expected = BTreeMap::new();
        expected.insert("prefix/key1".to_string(), Primatives::Number(1.0));
        expected.insert("prefix/key2".to_string(), Primatives::Number(2.0));
        assert_eq!(new_batch, expected);
    }

    #[test]
    fn test_broker_key_from_str() {
        let key = BrokerKey::from("test");
        assert_eq!(key.string, "test");
    }

    #[test]
    fn test_broker_key_from_string() {
        let key = BrokerKey::from("test".to_string());
        assert_eq!(key.string, "test");
    }

    #[test]
    fn test_broker_key_from_broker_key() {
        let key = BrokerKey::from("test".to_string());
        let string: String = key.into();
        assert_eq!(string, "test");
    }
}
