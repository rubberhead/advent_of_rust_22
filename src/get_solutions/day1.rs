use std::io::BufRead; 

pub fn get_star_1(input: &str) -> i64 {
    let mut input = input.as_bytes(); 
    let mut max: i64 = 0; 
    let mut curr: i64 = 0; 
    loop {
        let mut buf = String::new(); 
        let result = input.read_line(&mut buf)
            .expect("[day1::get_star_1] Error while reading line from input"); 

        match buf.as_str() {
            "\n" | "" => {
                max = if curr >= max { curr } else { max };
                curr = 0;  
            }, 
            _ => {
                curr += buf.trim().parse::<i64>()
                    .expect(format!("[day1::get_star_1] Input contains non-integer lines: {}", buf).as_str()); 
            }
        }

        if result == 0 {
            break;
        }
    }

    return max; 
}

pub fn get_star_2(input: &str) -> Result<i64, ()> {
    let mut input = input.as_bytes(); 
    let mut calories: Vec<i64> = Vec::new();
    let mut curr = 0;  
    loop {
        let mut buf = String::new(); 
        let result = input.read_line(&mut buf)
            .expect("[day1::get_star_1] Error while reading line from input"); 

        match buf.as_str() {
            "\n" | "" => {
                calories.push(curr);  
                curr = 0;  
            }, 
            _ => {
                curr += buf.trim().parse::<i64>()
                    .expect(format!("[day1::get_star_1] Input contains non-integer lines: {}", buf).as_str()); 
            }
        }

        if result == 0 {
            break;
        }
    }

    calories.sort();
    calories.reverse();  
    if let (Some(a), Some(b), Some(c)) = (calories.get(0), calories.get(1), calories.get(2)) {
        return Ok(a + b + c);
    }
    return Err(()); 
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn test_star_1_against_sample_input() {
        assert_eq!(super::get_star_1(SAMPLE_INPUT), 24000); 
    }

    #[test]
    fn test_star_2_against_sample_input() {
        assert_eq!(super::get_star_2(SAMPLE_INPUT).unwrap(), 45000); 
    }
}

