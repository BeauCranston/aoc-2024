use crate::shared::{get_current_dir, parse_file_contents};
use std::usize;

pub fn run_solution_2() {
    //get the current directory to read the data file
    let dir_str = match get_current_dir("two") {
        Some(t) => t,
        None => panic!("Failed to get dir"),
    };
    //parse the contents of the input data file
    let contents = match parse_file_contents(&dir_str, "input.txt") {
        Ok(contents) => contents,
        Err(e) => {
            panic!("Error reading the file: {}", e);
        }
    };

    for line in contents.lines() {
        println!("line")
    }
}
