use std::path::Path;

// filters by extension
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
fn filter_function() {
    // filtering: regex, .gitignore, wordlist, permission
    // common: path(filter child nodes) -> vector
}