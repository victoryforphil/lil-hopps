use std::hash::Hash;

#[derive(Debug, Clone, PartialOrd)]
pub struct Tag {
    pub name: String,
    pub value: Option<String>,
}
impl Hash for Tag {
    /// We only hash for the name
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Tag {}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

/// Conversion function from &str to Tag
impl From<&str> for Tag {
    fn from(name: &str) -> Self {
        Tag::new(name)
    }
}

/// Conversion function from String to Tag
impl From<String> for Tag {
    fn from(name: String) -> Self {
        Tag::new(&name)
    }
}

/// Conversion function from (String, String) to Tag
impl From<(String, String)> for Tag {
    fn from((name, value): (String, String)) -> Self {
        Tag::new_with_value(&name, &value)
    }
}

/// Conversion function from (&str, &str) to Tag
impl From<(&str, &str)> for Tag {
    fn from((name, value): (&str, &str)) -> Self {
        Tag::new_with_value(name, value)
    }
}

impl Tag {
    /// Create a new tag with no value set
    /// - `name`: The name of the tag
    pub fn new(name: &str) -> Tag {
        Tag {
            name: name.to_string(),
            value: None,
        }
    }
    /// Create a new tag with a value set
    /// - `name`: The name of the tag
    /// - `value`: The value of the tag
    pub fn new_with_value(name: &str, value: &str) -> Tag {
        Tag {
            name: name.to_string(),
            value: Some(value.to_string()),
        }
    }

    /// Builder function to add a value to the tag
    /// - `value`: The value of the tag
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_new() {
        let tag = Tag::new("test");
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, None);
    }

    #[test]
    fn test_tag_new_with_value() {
        let tag = Tag::new_with_value("test", "value");
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, Some("value".to_string()));
    }

    #[test]
    fn test_tag_value() {
        let tag = Tag::new("test").value("value");
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, Some("value".to_string()));
    }

    #[test]
    fn test_tag_cmp() {
        let tag1 = Tag::new("test1");
        let tag2 = Tag::new("test2");
        assert_eq!(tag1.cmp(&tag2), std::cmp::Ordering::Less);
        assert_eq!(tag2.cmp(&tag1), std::cmp::Ordering::Greater);
        assert_eq!(tag1.cmp(&tag1), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_tag_eq() {
        let tag1 = Tag::new("test1");
        let tag2 = Tag::new("test1").value("value"); // Value should not matter
        assert_eq!(tag1, tag2);
    }

    #[test]
    fn test_tag_string_conversion() {
        let tag: Tag = "test".into();
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, None);

        let tag: Tag = String::from("test").into();
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, None);

        let tag: Tag = ("test", "value").into();
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, Some("value".to_string()));

        let tag: Tag = ("test", "value").into();
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value, Some("value".to_string()));
    }
}
