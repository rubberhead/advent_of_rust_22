use std::collections::{VecDeque, HashSet, BTreeMap, HashMap};

use super::AOCSolutions; 

// Single-Source Shortest Path
type Graph = Vec<Vec<u8>>; 
type Node = u8; 
type Position = (usize, usize); // row-column

fn is_valid_move(here: &u8, there: &u8) -> bool {
    let (mut here, mut there) = (here.clone() ,there.clone()); 
    if here == 'S' as u8 { here = 'a' as u8; }
    if there == 'E' as u8 { there = 'z' as u8; }
    if there > here { there - here == 1 } else { true }
}

fn get_next_possible_moves(curr_pos: &Position, graph: &Vec<Vec<u8>>) -> Vec<Position> {
    let curr_row = curr_pos.0; 
    let curr_col = curr_pos.1; 
    let curr_pos_val: u8 = graph[curr_row][curr_col]; 
    let mut positions: Vec<Position> = Vec::with_capacity(4); 

    // DOWN
    if let (Some(next_row), next_col) = (usize::checked_sub(curr_row, 1), curr_col) {
        if next_col < graph[next_row].len() && is_valid_move(&curr_pos_val, &graph[next_row][next_col]) {
            positions.push((next_row, next_col)); 
        }
    }

    // LEFT
    if let (next_row, Some(next_col)) = (curr_row, usize::checked_sub(curr_col, 1)) {
        if is_valid_move(&curr_pos_val, &graph[next_row][next_col]) {
            positions.push((next_row, next_col)); 
        }
    }

    // UP
    let (next_row, next_col) = (curr_row + 1, curr_col); 
    if next_row < graph.len() && next_col < graph[next_row].len() {
        if is_valid_move(&curr_pos_val, &graph[next_row][next_col]) {
            positions.push((next_row, next_col)); 
        }
    }

    // RIGHT
    let (next_row, next_col) = (curr_row, curr_col + 1); 
    if next_col < graph[curr_row].len() {
        if is_valid_move(&curr_pos_val, &graph[next_row][next_col]) {
            positions.push((next_row, next_col)); 
        }
    }

    // Rank by heuristic? 
    positions
}

fn graph_search(graph: &Vec<Vec<u8>>, source: Position, goal: Position) -> Option<usize> {
    // Bound check
    if goal.0 > graph.len() || goal.1 > graph[goal.0].len() { return None; }

    // Store traversed nodes: no need to backtrack
    let mut traversed: HashMap<Position, usize> = HashMap::with_capacity(graph.len() * graph[0].len()); 
    traversed.insert(source, 0); 
    // Store queued nodes in frontier
    let mut frontier: BTreeMap<usize, Vec<Position>> = BTreeMap::from([(0 as usize, vec![source])]); 

    while !frontier.is_empty() {
        let curr_pos = frontier.first_entry().unwrap().get_mut().pop().unwrap();
        for next_pos in get_next_possible_moves(&curr_pos, graph) {
            let next_d_from_curr = traversed.get(&curr_pos).unwrap() + 1;
            match traversed.get(&next_pos) {
                None => {
                    traversed.insert(next_pos, next_d_from_curr); 
                    let next_d_vec = frontier.entry(next_d_from_curr).or_insert(vec![]); 
                    next_d_vec.push(next_pos); 
                }, 
                Some(curr_next_d) if *curr_next_d > next_d_from_curr => {
                    // Update frontier
                    frontier.entry(*curr_next_d).and_modify(|v| v.retain(|p| *p != next_pos )); 
                    let next_d_vec = frontier.entry(next_d_from_curr).or_insert(vec![]); 
                    next_d_vec.push(next_pos); 

                    // Update traversed
                    traversed.insert(next_pos, next_d_from_curr); 
                }, 
                _ => (), 
            }
        }

        if frontier.first_entry().unwrap().get().is_empty() { frontier.pop_first(); }
    }
    return traversed.get(&goal).copied(); 
}

pub struct Day12; 

impl AOCSolutions for Day12 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut graph: Graph = Vec::new(); 
        let (mut src_buf, mut tgt_buf): (Vec<Position>, Vec<Position>) = (Vec::with_capacity(1), Vec::with_capacity(1)); 
        // Get source and destination
        for (r_idx, line )in input.lines().enumerate() {
            graph.push(Vec::with_capacity(line.len())); 
            for (c_idx, chr )in line.as_bytes().iter().enumerate() {
                graph[r_idx].push(*chr); 
                if *chr == 'S' as u8 {
                    src_buf.push((r_idx, c_idx)); 
                } else if *chr == 'E' as u8 {
                    tgt_buf.push((r_idx, c_idx)); 
                }
            }
        }
        let (src, tgt) = (src_buf[0], tgt_buf[0]); 
        
        // Find result
        match graph_search(&graph, src, tgt) {
            Some(r) => return Ok(r.try_into().unwrap()), 
            None => {
                eprintln!("[Day12::get_star_1] Unreachable destination `{:?}` from source `{:?}`", tgt, src); 
                return Err(()); 
            }
        }
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let mut graph: Graph = Vec::new(); 
        let (mut src_buf, mut tgt_buf): (Vec<Position>, Vec<Position>) = (Vec::new(), Vec::with_capacity(1)); 
        // Get source(s) and destination
        for (r_idx, line )in input.lines().enumerate() {
            graph.push(Vec::with_capacity(line.len())); 
            for (c_idx, chr )in line.as_bytes().iter().enumerate() {
                graph[r_idx].push(*chr); 
                if *chr == 'S' as u8 || *chr == 'a' as u8 {
                    src_buf.push((r_idx, c_idx)); 
                } else if *chr == 'E' as u8 {
                    tgt_buf.push((r_idx, c_idx)); 
                }
            }
        }
        let tgt = tgt_buf[0]; 

        let mut min_step = i64::MAX;  
        for src in src_buf {
            match graph_search(&graph, src, tgt) {
                Some(r) => {
                    let r: i64 = r.try_into().unwrap(); 
                    if r < min_step { min_step = r; }
                }, 
                None => {
                    eprintln!("[Day12::get_star_1] Unreachable destination `{:?}` from source `{:?}`", tgt, src); 
                }
            }
        }
        return Ok(min_step); 
    }
}

#[cfg(test)]
mod tests {
    use super::AOCSolutions; 
    use super::Day12; 

    const SIMPLE_INPUT: &str = r"SbcdefghijklmnopqrstuvwxyE"; 

    const SAMPLE_INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day12::get_star_1(SIMPLE_INPUT).unwrap(), 25); 
        assert_eq!(Day12::get_star_1(SAMPLE_INPUT).unwrap(), 31); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day12::get_star_2(SAMPLE_INPUT).unwrap(), 29); 
    }
}