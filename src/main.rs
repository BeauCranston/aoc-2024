use aoc_day_one::run_solution_1;
use aoc_day_two::run_solution_2;
mod aoc_day_one;
mod aoc_day_two;
mod shared;
fn main() {
    println!("AOC 2024 in Rust. By Beau Cranston");
    print_solution_header(1);
    run_solution_1();
    print_break();
    print_solution_header(2);
    run_solution_2();
    print_break();
}

fn print_solution_header(num: u8) {
    println!("Runnng solution {num}");
    println!("==================================");
}
fn print_break() {
    println!("==================================");
    println!("\n\r");
}
