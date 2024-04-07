use self::get_latest::GetLatestQuery;

mod tag_filter;
mod lookup;
mod lookup_range;
mod get_latest;
mod write;

pub use tag_filter::*;
pub use lookup::*;
pub use lookup_range::*
pub use get_latest::*;
pub use write::*;

pub enum QueryCommand{
    GetLatest(GetLatestQuery),
    Lookup(LookupQuery),
    LookupRange(LookupRangeQuery),
    Write(WriteQuery),
}
