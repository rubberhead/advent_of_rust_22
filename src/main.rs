use std::env; 

use advent_of_rust_22::get_solutions::AOCSolutions;
use advent_of_rust_22::get_solutions::*; 
use day13::Day13; 

// TODO: Write a proper CLI. 
fn main() {
    
    //let arguments: Vec<String> = env::args().collect(); 
    //assert!(arguments.len() == 2); 
    //let path = &arguments[1];
    let path = "./inputs/day13/input"; 
    
    let input = advent_of_rust_22::parse_to_string(path).unwrap(); 

    println!("{}", Day13::get_star_1(input.as_str()).unwrap());
    println!("{}", Day13::get_star_2(input.as_str()).unwrap()); 
}
