use std::fs::DirEntry;

pub trait Filter {
    fn filter<T, I>(&mut self, collection: T ) -> Vec<DirEntry>
    where 
        T: IntoIterator<Item = I>;
}