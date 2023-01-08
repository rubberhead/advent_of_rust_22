use std::env; 

use advent_of_rust_22::get_solutions::AOCSolutions;
use advent_of_rust_22::get_solutions::*; 
use day10::Day10; 

// TODO: Write a proper CLI. 
fn main() {
    let arguments: Vec<String> = env::args().collect(); 
    assert!(arguments.len() == 2); 
    
    let input = advent_of_rust_22::parse_to_string(&arguments[1]).unwrap(); 

    println!("{}", Day10::get_star_1(input.as_str()).unwrap());
    println!("{}", Day10::get_star_2(input.as_str()).unwrap()); 
}
