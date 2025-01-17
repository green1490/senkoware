use std::fs::DirEntry;

pub trait Filter {
    fn filter(&mut self, entries: Vec<DirEntry>) -> Vec<DirEntry>;
}