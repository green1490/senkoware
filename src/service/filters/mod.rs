pub mod dummy_filter;
pub mod gitignore_filter;
pub mod extension_filter;

#[cfg(test)]
mod tests {
    use std::{path::Path, vec};

    use crate::service::interface::filter::Filter;

    use super::dummy_filter::DummyFilter;

    #[test]
    fn entry_filter() {
        let gitignore:Vec<&str> = vec!["node_modules", "filter.txt"];
        
        let entries:Vec<&str> = vec![
            "/home",
            "flag.txt",
            "/path/to/flag.txt",
            "node_moduels",
            "filter.txt"
        ];
        // checks if it has extension
        // and if its not in the gitignore file
        let filtered_entries:Vec<&&str> = entries.iter().filter(
            |entry| {
                if gitignore.contains(&entry) {
                    return false;
                }
                else if Path::new(entry).extension().is_none() {
                    return false;
                }
                else {
                    return true;
                }
            }
        ).collect();
        assert_eq!(2, filtered_entries.len());
    }

    #[test]
    fn dummy_filter() {
        let mut dummy: DummyFilter = DummyFilter{};
        let entries: Vec<&str> = vec![
            "test.txt",
            "flag.txt",
            ];
        let filtered_list = dummy.filter(entries);

        assert_eq!(filtered_list.len(), 0);
    }
}