use std::fs::{DirEntry, Metadata};
use std::io::Result;

// checks if you have the permissions to access the entry's content
pub fn permission_check(entry: DirEntry) {
    match entry.metadata() {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}