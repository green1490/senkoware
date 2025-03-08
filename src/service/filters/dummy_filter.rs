use crate::service::interface::filter::Filter;

pub struct DummyFilter;

impl Filter for DummyFilter {
    fn filter<T, I>(&mut self, _entries: T) -> Vec<std::fs::DirEntry> {
        vec![]
    }
}