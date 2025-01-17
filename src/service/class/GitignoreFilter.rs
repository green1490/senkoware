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
    fn filter(&mut self, entries: Vec<std::fs::DirEntry>) -> Vec<std::fs::DirEntry> {
        todo!()
    }
}