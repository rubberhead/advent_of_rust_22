use super::AOCSolutions; 
use std::{collections::HashSet, cmp::{min, max}}; 

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    x: usize, 
    depth: usize, 
}

impl Position {
    pub fn from(str_rep: &str) -> Position {
        let mut str_rep_itr = str_rep.split(',').take(2); 
        let x: usize = str_rep_itr.next()
            .expect(&format!("[day14::Position::from] Malformed parse str: \"{}\"", str_rep))
            .parse()
            .expect(&format!("[day14::Position::from] Malformed parse str: \"{}\"", str_rep));
            
        let depth: usize = str_rep_itr.next()
            .expect(&format!("[day14::Position::from] Malformed parse str: \"{}\"", str_rep))
            .parse()
            .expect(&format!("[day14::Position::from] Malformed parse str: \"{}\"", str_rep));
        
        Position { x, depth }
    }

    pub fn generate_in_range_inclusive(from: &Position, to: &Position) -> Vec<Position> {
        if from == to { return vec![from.clone()]; } 
        let (min_pos, max_pos) = (min(from, to), max(from, to));

        if min_pos.x == max_pos.x {
            let mut pos_vec: Vec<Position> = Vec::with_capacity(max_pos.depth - min_pos.depth + 1); 
            let mut temp_pos: Position = Position { x: min_pos.x, depth: min_pos.depth }; 
            while temp_pos <= *max_pos {
                pos_vec.push(temp_pos.clone()); 
                temp_pos.depth += 1; 
            }
            return pos_vec; 
        } else if min_pos.depth == max_pos.depth {
            let mut pos_vec: Vec<Position> = Vec::with_capacity(max_pos.x - min_pos.x + 1); 
            let mut temp_pos: Position = Position { x: min_pos.x, depth: min_pos.depth }; 
            while temp_pos <= *max_pos {
                pos_vec.push(temp_pos.clone()); 
                temp_pos.x += 1; 
            }
            return pos_vec; 
        } else {
            panic!("[day14::Position::generate_in_range] Cannot parse diagonal segments"); 
        }
    }
}

const SEGMENT_SEP: &str = " -> "; 
const SAND_SOURCE: Position = Position { x: 500, depth: 0 }; 

/**
Recursively finds the deepest possible location attainable from a given source, returning that deepest 
attainable `Position` value or the first `Position` value that reaches `depth_bound`. 

**This function assumes that `curr_pos` is unblocked (i.e., not within `obstacle_set`).**
 */
fn find_bounded_sand_pos(curr_pos: &Position, obstacle_set: &HashSet<Position>, depth_bound: usize) -> (Position, bool) {
    if curr_pos.depth == depth_bound { 
        return (curr_pos.clone(), true); 
    }

    let mut next_pos = Position { x: curr_pos.x, depth: curr_pos.depth + 1 }; 
    if obstacle_set.contains(&next_pos) { 
        next_pos = Position { x: usize::saturating_sub(curr_pos.x, 1), depth: curr_pos.depth + 1 }; 
    }
    if obstacle_set.contains(&next_pos) { 
        next_pos = Position { x: curr_pos.x + 1, depth: curr_pos.depth + 1 }; 
    }
    if obstacle_set.contains(&next_pos) {
        return (curr_pos.clone(), false); 
    }
    return find_bounded_sand_pos(&next_pos, obstacle_set, depth_bound); 
}

/**
Parse input to `(blocked_set, abyss_bound)`
 */
fn parse_input(input: &str) -> (HashSet<Position>, usize) {
    let mut blocked_set: HashSet<Position> = HashSet::new(); 
    let mut min_depth = 0 as usize;
    for rock_formation in input.lines() {
        let ctrl_nodes: Vec<Position> = rock_formation.split(SEGMENT_SEP)
            .map(|s| Position::from(s))
            .collect(); // Split by separator

        for i in 0..ctrl_nodes.len() - 1 { // For each segment representation in line
            let (ref l_node, ref r_node) = (ctrl_nodes[i], ctrl_nodes[i + 1]); 
            for pos in Position::generate_in_range_inclusive(l_node, r_node) {
                if pos.depth > min_depth { min_depth = pos.depth; } // Find lowest position which denotes start of abyss
                blocked_set.insert(pos); 
            }
        }
    }
    return (blocked_set, min_depth + 1); 
}

pub struct Day14; 

impl AOCSolutions for Day14 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let (mut blocked_set, abyss_bound) = parse_input(input); 
        if blocked_set.contains(&SAND_SOURCE) { 
            eprintln!("[Day14::get_star_1] Blocked sand source in current configuration"); 
            return Err(()); 
        }

        let mut sand_unit_count: i64 = 0; 
        while let (sp, false) = find_bounded_sand_pos(&SAND_SOURCE, &blocked_set, abyss_bound) {
            sand_unit_count += 1;
            if sp == SAND_SOURCE {
                eprintln!("[Day14::get_star_1] Blocked sand source during iteration"); 
                return Err(()); 
            }
            blocked_set.insert(sp); 
        }
        return Ok(sand_unit_count); 
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let (mut blocked_set, floor_bound) = parse_input(input); 
        if blocked_set.contains(&SAND_SOURCE) { 
            eprintln!("[Day14::get_star_1] Blocked sand source in current configuration"); 
            return Ok(0); 
        }

        let mut sand_unit_count: i64 = 0; 
        loop {
            let (sp, _) = find_bounded_sand_pos(&SAND_SOURCE, &blocked_set, floor_bound); 
            sand_unit_count += 1;
            if sp == SAND_SOURCE { break; }
            blocked_set.insert(sp); 
        }
        return Ok(sand_unit_count); 
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day14::get_star_1(SAMPLE_INPUT).unwrap(), 24); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day14::get_star_2(SAMPLE_INPUT).unwrap(), 93); 
    }
}

