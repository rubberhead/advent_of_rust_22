pub mod day1; 
pub mod day2;
pub mod day3;
pub mod day4; 
pub mod day5; 

/**
Interface for iterating through problems as `Box<dyn AOCSolutions>` in *main.rs*. 
 */
pub trait AOCSolutions {
    fn get_star_1(input: &str) -> Result<i64, ()>; 
    fn get_star_2(input: &str) -> Result<i64, ()>; 
}