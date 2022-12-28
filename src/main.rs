use advent_of_rust_22::get_solutions::day1; 
use std::env; 
use std::fs::File;
use std::io::Read;

// TODO: Write a proper CLI. 
fn main() {
    let arguments: Vec<String> = env::args().collect(); 
    assert!(arguments.len() == 2); 
    
    let mut buf = String::new(); 
    let mut inputf = File::open(&arguments[1])
        .expect("[main] Invalid path");
    inputf.read_to_string(&mut buf).unwrap();  

    println!("{}", day1::get_star_1(buf.as_str()));
    println!("{}", day1::get_star_2(buf.as_str()).unwrap()); 
}
