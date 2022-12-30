use std::env; 

use advent_of_rust_22::get_solutions::AOCSolutions;
use advent_of_rust_22::get_solutions::day1::Day1;  
use advent_of_rust_22::get_solutions::day2::Day2; 
use advent_of_rust_22::get_solutions::day3::Day3; 

// TODO: Write a proper CLI. 
fn main() {
    let arguments: Vec<String> = env::args().collect(); 
    assert!(arguments.len() == 2); 
    
    let input = advent_of_rust_22::parse_to_string(& arguments[1]).unwrap(); 

    println!("{}", Day3::get_star_1(input.as_str()).unwrap());
    println!("{}", Day3::get_star_2(input.as_str()).unwrap()); 
}
