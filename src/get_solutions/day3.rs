use std::hash::Hash;
use std::io::BufRead;
use std::collections::HashSet; 

use super::AOCSolutions; 

pub struct Day3; 

impl AOCSolutions for Day3 {
    // Can be parallelized. Maybe work on it later? 
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut input = input.as_bytes(); 
        let mut line = String::new(); 
        let mut sum: i64 = 0; 
        loop {
            let _ = input.read_line(&mut line)
                .expect("[Day3::get_star_1] Error while reading line from `input`");
            let line_u8 = line.trim().as_bytes(); 
            let divisor = line_u8.len() / 2; // Assumes len % 2 == 0
            if divisor == 0 {
                return Ok(sum); 
            }
            
            let (compartment_1, compartment_2): (HashSet<&u8>, HashSet<&u8>) = (
                HashSet::from_iter(line_u8[..divisor].iter()), 
                HashSet::from_iter(line_u8[divisor..].iter())
            );
            
            let intersection = compartment_1.intersection(&compartment_2)
                .filter(|u| u.is_ascii()); 

            sum += intersection
                .map(|u| Day3::priority(u).unwrap())
                .sum::<i64>(); 

            line.clear(); 
        }
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        return Err(()); 
    }
}

impl Day3 {
    fn priority(ascii_u8: &u8) -> Result<i64, ()> {
        const UPPER_A_U8: u8 = 0x41u8; 
        const LOWER_A_U8: u8 = 0x61u8;

        if ascii_u8.is_ascii() {
            match ascii_u8.is_ascii_uppercase() {
                true => return Ok((ascii_u8 - UPPER_A_U8 + 27).into()), 
                false => return Ok((ascii_u8 - LOWER_A_U8 + 1).into()),  
            }
        }
        return Err(()); 
    }
}

#[cfg(test)]
mod tests {
    use crate::get_solutions::AOCSolutions;

    use super::Day3;

    const SAMPLE_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day3::get_star_1(SAMPLE_INPUT).unwrap(), 157)
    }
}