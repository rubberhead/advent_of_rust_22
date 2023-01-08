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

    fn is_valid_segment(head: &Position, tail: &Position) -> bool {
        let del_x = i64::abs_diff(head.0, tail.0); 
        let del_y = i64::abs_diff(head.1, tail.1); 
        del_x <= 1 && del_y <= 1
    }
}

impl RopeConfig for AdvancedRopeConfig {
    fn increment_in_dir(&mut self, incr_variant: MoveVariant) {
        // Chain validation: 
        // Move head once, then...
        // if (*a, b) valid then we are finished, 
        // else alter b to a => (*a, a) and compare (a, c) until valid seg found
        let mut old_seg_head = self.ctrl_nodes[0].clone(); 
        match incr_variant {
            MoveVariant::XAdd => self.ctrl_nodes[0].0 += 1, 
            MoveVariant::XSub => self.ctrl_nodes[0].0 -= 1, 
            MoveVariant::YAdd => self.ctrl_nodes[0].1 += 1, 
            MoveVariant::YSub => self.ctrl_nodes[0].1 -= 1, 
        };

        for i in 1..self.ctrl_nodes.len() {
            if AdvancedRopeConfig::is_valid_segment(&self.ctrl_nodes[i - 1], &self.ctrl_nodes[i]) {
                break; 
            } else {
                // Alter ctrl_nodes[i] according to specification
                let x_dist = self.ctrl_nodes[i].0 - self.ctrl_nodes[i - 1].0;
                let y_dist = self.ctrl_nodes[i].1 - self.ctrl_nodes[i - 1].1; 
                match (i64::abs(x_dist), i64::abs(y_dist)) {
                    (0, 2) => self.ctrl_nodes[i].1 -= y_dist / 2, 
                    (2, 0) => self.ctrl_nodes[i].0 -= x_dist / 2, 
                    (1, 2) => {
                        self.ctrl_nodes[i].0 -= x_dist;
                        self.ctrl_nodes[i].1 -= y_dist / 2;
                    }, 
                    (2, 1) => {
                        self.ctrl_nodes[i].0 -= x_dist / 2; 
                        self.ctrl_nodes[i].1 -= y_dist; 
                    }, 
                    (2, 2) => {
                        self.ctrl_nodes[i].0 -= x_dist / 2; 
                        self.ctrl_nodes[i].1 -= y_dist / 2;
                    }
                    _ => panic!("[day9::AdvancedRopeConfig::increment_in_dir] Invalid rope configuration: This should not happen"), 
                }
            }
        }
    }

    fn is_valid_configuration(&self) -> bool {
        // Unnecessary, but why not
        for (seg_head, seg_tail) in self.ctrl_nodes.iter().zip(self.ctrl_nodes[1..].iter()) {
            if !AdvancedRopeConfig::is_valid_segment(seg_head, seg_tail) {
                return false; 
            }
        }
        return true; 
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
        let mut rope_config = AdvancedRopeConfig::new(10, (0, 0)); 

        for line in input.lines() {
            let mut mvmt = Move::from_line(line); 
            while !mvmt.is_noop() {
                mvmt.perform_once(&mut rope_config); 
                unique_tail_positions.insert(rope_config.ctrl_nodes.last().unwrap().clone()); 
            }
        }

        return Ok(unique_tail_positions.len().try_into().unwrap()); 
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

    const SAMPLE_INPUT_2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day9::get_star_1(SAMPLE_INPUT).unwrap(), 13); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day9::get_star_2(SAMPLE_INPUT).unwrap(), 1); 
        assert_eq!(Day9::get_star_2(SAMPLE_INPUT_2).unwrap(), 36); 
    }
}

