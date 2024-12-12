use std::usize;

use crate::shared::{get_current_dir, parse_file_contents};

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
    let first_column_count: i32 = first_column.len() as i32;
    let second_column_count: i32 = second_column.len() as i32;

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
fn quick_sort(column: &mut Vec<u32>, low: i32, high: i32) {
    //ensure to only process the list if the low value is less than the high value
    if low < high {
        //acuire the pivot point
        let part: i32 = partition(column, low, high);
        //use the pivot to sort the items less than the pivot
        quick_sort(column, low, part - 1);
        //use the pivot to sort the items greater than the pivot
        quick_sort(column, part + 1, high);
    }
}

//partition algorithm that just chooses the last value as the pivot point
fn partition(arr: &mut Vec<u32>, low: i32, high: i32) -> i32 {
    //select last value
    let pivot = arr[high as usize];

    //initialize i minimum of - 1 but we increment before
    let mut i: i32 = low as i32 - 1;
    // loop low to high
    for n in low..high {
        //if the number is less than the pivot than increment i and swap i and n
        if arr[n as usize] < pivot {
            i += 1;
            swap(arr, i, n);
        }
    }

    //finally swap the value after i with the pivot (in our case the last element)
    swap(arr, i + 1, high);

    return i + 1;
}

//algorithm to swap 2 elements
fn swap(arr: &mut Vec<u32>, i: i32, j: i32) {
    let temp = arr[i as usize];
    arr[i as usize] = arr[j as usize];
    arr[j as usize] = temp;
}
