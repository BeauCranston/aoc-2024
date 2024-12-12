use std::env::current_dir;

pub fn get_current_dir(day_text: &str) -> Option<String> {
    let immediate_parent_dir = format!("aoc_day_{}", day_text);
    match current_dir() {
        Ok(path_buf) => path_buf
            .join("src")
            .join(immediate_parent_dir)
            .to_str()
            .map(String::from),
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            None
        }
    }
}
