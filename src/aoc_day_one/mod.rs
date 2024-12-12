use std::path::Path;
use std::usize;
use std::{
    fs::File,
    io::{BufReader, Error, Read},
};

use crate::shared::get_current_dir;

//main function
pub fn run_solution_1() {
    //get the current directory to read the data file
    let dir_str = match get_current_dir("one") {
        Some(t) => t,
        None => panic!("Failed to get dir"),
    };
    //parse the contents of the input data file
    let contents = match parse_file_contents(&dir_str, "input-data.txt") {
        Ok(contents) => contents,
        Err(e) => {
            panic!("Error reading the file: {}", e);
        }
    };
    //init variables for each list
    let mut first_column: Vec<u32> = vec![];
    let mut second_column: Vec<u32> = vec![];

    //split the lines by "  " and insert appropriate values into their respective lists
    for line in contents.lines() {
        let spl: Vec<&str> = line.split("  ").collect();
        if spl.len() < 2 {
            continue;
        }
        //if the parse fails just ignore it
        match spl[0].trim().parse::<u32>() {
            Ok(t) => first_column.push(t),
            Err(_e) => {}
        };

        //if the parse fails just ignore it
        match spl[1].trim().parse::<u32>() {
            Ok(t) => second_column.push(t),
            Err(_e) => {}
        }
    }
    let first_column_count = &first_column.iter().count();
    let second_column_count = &second_column.iter().count();

    //perform quick sort on the lists
    quick_sort(&mut first_column, 0, first_column_count - 1);
    quick_sort(&mut second_column, 0, second_column_count - 1);

    let mut total_distance: u32 = 0;

    for n in 0..first_column.len() {
        let temp_distance: u32 = first_column[n].abs_diff(second_column[n]);

        total_distance += temp_distance;
    }

    println!("{}", total_distance);
}

//quick sort algorith
fn quick_sort(column: &mut Vec<u32>, low: usize, high: usize) {
    //ensure to only process the list if the low value is less than the high value
    if low < high {
        //acuire the pivot point
        let part: usize = partition(column, low, high);
        //use the pivot to sort the items less than the pivot
        quick_sort(column, low, part - 1);
        //use the pivot to sort the items greater than the pivot
        quick_sort(column, part + 1, high);
    }
}

//partition algorithm that just chooses the last value as the pivot point
fn partition(arr: &mut Vec<u32>, low: usize, high: usize) -> usize {
    //select last value
    let pivot = arr[high];

    //initialize i must be a minimum of 0
    let mut i: usize = if low as i32 - 1 < 0 { 0 } else { low - 1 };

    for n in low..high {
        if arr[n] < pivot {
            i += 1;
            swap(arr, i as usize, n);
        }
    }

    swap(arr, i as usize + 1, high);

    return i as usize + 1;
}

fn swap(arr: &mut Vec<u32>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn parse_file_contents(file_dir: &str, file_name: &str) -> Result<String, Error> {
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
