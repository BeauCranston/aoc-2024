use std::env::current_dir;
use std::path::Path;
use std::{
    fs::File,
    io::{BufReader, Error, Read},
};

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
//parses the input.txt into a string
pub fn parse_file_contents(file_dir: &str, file_name: &str) -> Result<String, Error> {
    let path = Path::new(file_dir).join(file_name);
    let file: File = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!(
            "Error opening input-data.txt. Please make sure the file is in the aoc_day_one folder. {e}"
        ),
    };
    let mut reader = BufReader::new(file);

    let mut file_contents = String::new();

    let _ = reader.read_to_string(&mut file_contents);

    Ok(file_contents)
}
