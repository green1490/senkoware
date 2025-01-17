use std::path::PathBuf;
use crate::service::interface::filter::Filter;

pub struct NoFilter {
    path: PathBuf
}

impl Filter for NoFilter {
    fn filter(&mut self, entries: Vec<std::fs::DirEntry>) -> Vec<std::fs::DirEntry> {
        todo!()
    }
}