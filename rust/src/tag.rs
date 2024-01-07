mod get_range;
mod tag;

pub use get_range::get_range;

#[derive(Clone, Debug, PartialEq)]
pub struct Tag(Vec<usize>);
