use super::AOCSolutions; 
use std::ops::RangeInclusive; 

pub struct Day4; 
type RangeInclDuo = (RangeInclusive<i64>, RangeInclusive<i64>); 

impl AOCSolutions for Day4 { // API
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let optional_ranges = Day4::parse_to_rangeduos(input); 
        if let Some(ranges) = optional_ranges {
            return Ok(ranges.iter().fold(0, |acc, (range_0, range_1)| {
                if range_0.contains(range_1.start()) && range_0.contains(range_1.end()) {
                    acc + 1
                } else if range_1.contains(range_0.start()) && range_1.contains(range_0.end()) {
                    acc + 1
                } else {
                    acc
                }
            })); 
        }

        eprintln!("[Day4::get_star_1] Malformed input"); 
        return Err(()); 
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let optional_ranges = Day4::parse_to_rangeduos(input); 
        if let Some(ranges) = optional_ranges {
            return Ok(ranges.iter().fold(0, |acc, (range_0, range_1)| {
                if range_0.start() <= range_1.end() && range_0.end() >= range_1.start() { // I'm getting dumb...
                    acc + 1
                } else {
                    acc
                }
            })); 
        }

        eprintln!("[Day4::get_star_2 Malformed input"); 
        return Err(()); 
    }
}

impl Day4 { // Helpers
    fn parse_to_rangeduos(input: &str) -> Option<Vec<RangeInclDuo>> {
        const NON_NUMERIC_MSG: &str = "[Day4::parse_to_rangeduos] Non-numeric input"; 

        let optional_ranges: Vec<Option<RangeInclDuo>> = input.lines()
            .map(|l| { // per line
                let line: Vec<&str> = l.trim().split(['-', ',']).take(4).collect(); 
                let nums: Option<(i64, i64, i64, i64)> = match line[..] {
                    [str_1, str_2, str_3, str_4] => Some(( // Correctly formed
                        str_1.parse().expect(NON_NUMERIC_MSG), 
                        str_2.parse().expect(NON_NUMERIC_MSG), 
                        str_3.parse().expect(NON_NUMERIC_MSG), 
                        str_4.parse().expect(NON_NUMERIC_MSG), 
                    )), 
                    _ => { // Otherwise malformed
                        eprintln!("[Day4::parse_to_rangeduos] Malformed line which contains {} < 4 parsible entries", line.len()); 
                        None
                    }, 
                }; 

                match nums {
                    None => None, 
                    Some((a, b, c, d)) => Some((a..=b, c..=d)), 
                }
            })
            .collect();
            
        if optional_ranges.iter().any(|rd| rd.is_none()) {
            return None;
        }

        return Some(optional_ranges.into_iter().map(Option::unwrap).collect());             
    }
}

#[cfg(test)]
mod tests {
    use super::AOCSolutions;
    use super::Day4; 

    const SAMPLE_INPUT: &str = r"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"; 
    // section IDs may compose of n digits, range inclusive on both sides

    const SAMPLE_2: &str = r"1-5,1-5
    2-16,49-55
    31-98,98-99"; // star_1 -- 1, star_2 -- 2

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day4::get_star_1(SAMPLE_INPUT).unwrap(), 2); 
        assert_eq!(Day4::get_star_1(SAMPLE_2).unwrap(), 1); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day4::get_star_2(SAMPLE_INPUT).unwrap(), 4); 
        assert_eq!(Day4::get_star_2(SAMPLE_2).unwrap(), 2); 
    }
}