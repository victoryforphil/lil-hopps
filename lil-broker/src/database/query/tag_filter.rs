use crate::{DataPoint, Tag};
#[derive(Debug,Clone)]
pub enum TagFilterMode{
    Include,
    Exclude,
    ExcludeWithValue,
    IncludeWithValue
}
#[derive(Debug, Clone)]
pub struct TagFilter{
    pub tag: Tag,
    pub mode: TagFilterMode
}

impl TagFilter{
    pub fn new(tag: Tag) -> TagFilter{
        TagFilter{
            tag,
            mode: TagFilterMode::Include
        }
    }

    pub fn exclude(mut self) -> Self{
        self.mode = TagFilterMode::Exclude;
        self
    }

    pub fn exclude_with_value(mut self) -> Self{
        self.mode = TagFilterMode::ExcludeWithValue;
        self
    }

    pub fn include(mut self) -> Self{
        self.mode = TagFilterMode::Include;
        self
    }

    pub fn include_with_value(mut self) -> Self{
        self.mode = TagFilterMode::IncludeWithValue;
        self
    }

    pub fn is_valid(&self, datapoint: &DataPoint) -> bool{
        match self.mode{
            TagFilterMode::Include => datapoint.tags.contains(&self.tag),
            TagFilterMode::Exclude => !datapoint.tags.contains(&self.tag),
            // TODO: Implement value checking [vfp//2021-09-29]
            TagFilterMode::IncludeWithValue => datapoint.tags.contains(&self.tag),
            TagFilterMode::ExcludeWithValue => !datapoint.tags.contains(&self.tag),
        }
    }
}