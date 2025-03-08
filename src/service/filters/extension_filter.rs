use std::{collections::HashMap, hash::Hash};

use crate::service::interface::filter::Filter;

pub struct ExtensionFilter {
    
}

impl Default for ExtensionFilter {
    fn default() -> Self {
        Self {  }
    }
}

impl Filter for ExtensionFilter {
    fn filter<T, I>(&mut self, collection: T ) -> Vec<std::fs::DirEntry>
    where 
        T: IntoIterator<Item = I> {
        todo!()
    }
}