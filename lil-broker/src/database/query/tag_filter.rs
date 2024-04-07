use crate::Tag;
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
}