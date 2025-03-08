mod service;

use home::home_dir;
use std::fs;

fn main() -> std::io::Result<()> {
    let home_dir = fs::read_dir(home_dir().ok_or(std::io::Error::last_os_error())?)?;
    // iterates trough the directories
    // path_traversal(home_dir);
    Ok(())
}
