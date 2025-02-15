use std::{collections::VecDeque, fs::{DirEntry, ReadDir}};

use crate::service::interface::filter::Filter;

// uses dfs algorithm for traveling in the directory system
pub fn path_traversal<T: Filter>(home: ReadDir, filter: T) -> std::io::Result<()> {
    let mut stack:VecDeque<std::io::Result<DirEntry>> = home.collect();
    while stack.len() != 0 {
        let current_node = stack.pop_front().ok_or(std::io::Error::last_os_error())??;
        if current_node.metadata()?.is_dir() {
            let path = current_node.path();
            let child_node: VecDeque<std::io::Result<DirEntry>> = std::fs::read_dir(path)?.collect();
            stack.extend(child_node);
        }
        println!("{:?}", current_node);
    }
    Ok(())
}