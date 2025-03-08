use std::path::PathBuf;
use crate::service::interface::filter::Filter;

struct GitignoreFilter {
    path: PathBuf
}

impl GitignoreFilter {
    pub fn new(path: PathBuf) -> Self{
        GitignoreFilter{path}
    }
}

impl Filter for GitignoreFilter {   
    fn filter<T, I>(&mut self, entries: T) -> Vec<std::fs::DirEntry> {
        todo!()
    }
}