use super::AOCSolutions; 
use std::collections::HashSet; 

pub struct Day9; 

type Position = (i64, i64);

trait RopeConfig {
    /**
    Checks if `self` is a valid rope configuration (as defined by AOC). 
     */
    fn is_valid_configuration(&self) -> bool; 

    /**
    Mutates `self` to the next **valid** state in given direction (as `incr_variant` -- axis of 
    incrementation). 
     */
    fn increment_in_dir(&mut self, incr_variant: MoveVariant); 
}

#[derive(Clone, Copy)]
struct BasicRopeConfig {
    head_pos: Position, 
    tail_pos: Position, 
}

impl BasicRopeConfig {
    pub fn new(head_pos: Position, tail_pos: Position) -> BasicRopeConfig {
        BasicRopeConfig { head_pos, tail_pos }
    }
}

impl RopeConfig for BasicRopeConfig {
    fn is_valid_configuration(&self) -> bool {
        let del_x = i64::abs_diff(self.head_pos.0, self.tail_pos.0); 
        let del_y = i64::abs_diff(self.head_pos.1, self.tail_pos.1); 
        del_x <= 1 && del_y <= 1
    }

    fn increment_in_dir(&mut self, incr_variant: MoveVariant) {
        let old_head_pos = self.head_pos;
        match incr_variant { // only moves head
            MoveVariant::XAdd => self.head_pos.0 += 1, // x++
            MoveVariant::XSub => self.head_pos.0 -= 1, // x--
            MoveVariant::YAdd => self.head_pos.1 += 1, // y++
            MoveVariant::YSub => self.head_pos.1 -= 1, // y--
        }
        if !self.is_valid_configuration() {
            self.tail_pos = old_head_pos; 
        }
    }
}

struct AdvancedRopeConfig {
    ctrl_nodes: Vec<Position>, 
}

impl AdvancedRopeConfig {
    pub fn new(resolution: usize, init_position: Position) -> AdvancedRopeConfig {
        let mut ctrl_nodes: Vec<Position> = Vec::with_capacity(resolution); 
        for _ in 0..resolution {
            ctrl_nodes.push(init_position.clone()); 
        }
        AdvancedRopeConfig { ctrl_nodes }
    }
}

impl RopeConfig for AdvancedRopeConfig {
    fn increment_in_dir(&mut self, incr_variant: MoveVariant) {
        // Chain increment: 
        // for segment (a, b), if w/ head-only move we obtain valid (a, b), then we are finished, 
        // else alter (a, b) to (a*, b*) as usual and move onto , say, (b*, c) 
        todo!()
    }

    fn is_valid_configuration(&self) -> bool {
        // Unnecessary, but why not
        todo!()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MoveVariant {
    XAdd, XSub, YAdd, YSub 
}

struct Move {
    move_amnt: u64, 
    variant: MoveVariant, 
}

impl Move {
    pub fn from_line(line: &str) -> Move {
        let line = line.trim();
        let mut line_itr = line.split_whitespace(); 
        if let Some(direction) = line_itr.next() {
            if let Some(move_amnt_str) = line_itr.next() {
                if let Ok(move_amnt) = move_amnt_str.parse::<u64>() {
                    match direction {
                        "R" => return Move{ move_amnt, variant: MoveVariant::XAdd }, 
                        "L" => return Move{ move_amnt, variant: MoveVariant::XSub }, 
                        "U" => return Move{ move_amnt, variant: MoveVariant::YAdd }, 
                        "D" => return Move{ move_amnt, variant: MoveVariant::YSub }, 
                        _   => panic!("[day9::Move::from_line] Invalid operation code in `line` \"{}\"", line), 
                    }
                }
            }
        }
        panic!("[day9::Move::from_line] Invalid `line` fromat: \"{}\"", line)
    }

    pub fn perform_once(&mut self, rope_config: &mut impl RopeConfig) {
        if !self.is_noop() {
            rope_config.increment_in_dir(self.variant); 
            self.move_amnt -= 1;
        }
    }

    pub fn is_noop(&self) -> bool {
        self.move_amnt == 0
    }
}

impl AOCSolutions for Day9 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut unique_tail_positions: HashSet<Position> = HashSet::from([(0, 0)]); 
        let mut rope_config = BasicRopeConfig::new((0, 0), (0, 0)); 

        for line in input.lines() {
            let mut mvmt = Move::from_line(line); 
            while !mvmt.is_noop() {
                mvmt.perform_once(&mut rope_config); 
                unique_tail_positions.insert(rope_config.tail_pos); 
            }
        }

        return Ok(unique_tail_positions.len().try_into().unwrap()); 
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let mut unique_tail_positions: HashSet<Position> = HashSet::from([(0, 0)]); 


        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::AOCSolutions;
    use super::Day9; 

    const SAMPLE_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day9::get_star_1(SAMPLE_INPUT).unwrap(), 13); 
    }
}

