use super::AOCSolutions; 
use std::io::BufRead; 

struct Round<'a>(&'a Play, &'a Play);

#[derive (PartialEq, Eq, Clone)] // Clone is unnecessary, just for the looks
enum Play {
    Rock, 
    Paper, 
    Scissors
} 

impl <'a> Round<'a> {
    fn get_score(&self) -> i64 {
        let Round(my_play, other_play) = self; 
        my_play.outcome(other_play) + my_play.get_pref_score()
    }
}

impl Play {
    fn from_str(c: &str) -> Result<Play, ()> {
        match c {
            "A" | "X" => Ok(Play::Rock), 
            "B" | "Y" => Ok(Play::Paper), 
            "C" | "Z" => Ok(Play::Scissors), 
            _ => Err(()), 
        }
    }

    fn get_pref_score(&self) -> i64 {
        match self {
            Self::Rock => 1, 
            Self::Paper => 2, 
            Self::Scissors => 3, 
        }
    }

    fn outcome(&self, other: &Play) -> i64 {
        match self {
            Self::Rock => {
                match other {
                    Self::Paper => 0,  
                    Self::Rock => 3, 
                    Self::Scissors => 6, 
                }
            }, 
            Self::Paper => {
                match other {
                    Self::Scissors => 0, 
                    Self::Paper => 3, 
                    Self::Rock => 6, 
                }
            }, 
            Self::Scissors => {
                match other {
                    Self::Rock => 0, 
                    Self::Scissors => 3, 
                    Self::Paper => 6, 
                }
            }
        }
    }

    fn lose_over(&self) -> Play {
        match self {
            Play::Rock => Self::Scissors, 
            Play::Paper => Self::Rock,
            Play::Scissors => Self::Paper, 
        }
    }

    fn draw_over(&self) -> Play {
        return self.clone(); 
    }

    fn win_over(&self) -> Play {
        match self {
            Play::Rock => Self::Paper, 
            Play::Paper => Self::Scissors, 
            Play::Scissors => Self::Rock, 
        }
    }
}

pub struct Day2; 

impl AOCSolutions for Day2 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut input = input.as_bytes();
        let mut score: i64 = 0;
        let mut buf = String::new();  
        loop {
            let _ = input.read_line(&mut buf)
                .expect("[Day2::get_star_1] Error while reading line from input"); 
            
            match buf.as_str() {
                "" => return Ok(score), 
                _ => {
                    // There must be a better way...
                    let plays: Vec<Play> = buf.trim().split(' ')
                        .take(2)
                        .map(|s| { 
                            Play::from_str(s)
                                .expect(format!(
                                    "[Day2::get_star_2] Invalid substring in input: {}", s
                                ).as_str()) 
                            }
                        )
                        .collect();
                    let round = Round(&plays[1], &plays[0]); 
                    score += round.get_score(); 
                }
            }
            buf.clear(); 
        }
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let mut input = input.as_bytes();
        let mut score: i64 = 0;
        let mut buf = String::new();  
        loop {
            let _ = input.read_line(&mut buf)
                .expect("[Day2::get_star_1] Error while reading line from input"); 
            
            match buf.as_str() {
                "" => return Ok(score), 
                _ => {
                    let config: Vec<&str> = buf.trim().split(' ').take(2).collect();
                    let other_play = Play::from_str(config[0]).unwrap();  
                    let this_play = match config[1] {
                        "X" => Play::lose_over(&other_play), 
                        "Y" => Play::draw_over(&other_play), 
                        "Z" => Play::win_over(&other_play), 
                        a => panic!("[Day2::get_star_1] Invalid substring in input: {}", a)
                    }; 
                    let round = Round(&this_play, &other_play); 
                    score += round.get_score(); 
                }
            }
            buf.clear(); 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day2; 
    use super::AOCSolutions; 

    const SAMPLE_INPUT: &str = r"A Y
    B X
    C Z";


    #[test]
    fn test_star_1_against_sample_input() {
        assert_eq!(Day2::get_star_1(SAMPLE_INPUT).unwrap(), 15)
    }

    #[test]
    fn test_star_2_against_sample_input() {
        assert_eq!(Day2::get_star_2(SAMPLE_INPUT).unwrap(), 12)
    }
}