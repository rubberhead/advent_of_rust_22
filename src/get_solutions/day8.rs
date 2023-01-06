use super::AOCSolutions; 
use std::{iter::{Iterator, DoubleEndedIterator, Zip}, str::Lines, os::linux::raw}; 
use core::ops::Index; 

pub struct Day8;

impl AOCSolutions for Day8 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        Ok(Day8::parse(input))
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let res: Result<i64, _> = Day8::parse2(input).try_into(); 
        match res {
            Ok(v) => Ok(v), 
            Err(_) => Err(()), 
        }
    }
}

impl Day8 {
    /** 
    Bare minimum for checking if value @ idx in given `iterator` is "visible" in the given axis, as 
    represented by the `iterator`, assumed to be a plain old `Iterator` -- no length or anything.

    This function **consumes** `iterator`. 

    Obviously this is a dumb solution to the given problem -- we should be able to find all visible 
    elements in a view in two iterations (one from front, one from back) -- O(2n) time instead of O(n^2) 
    time for each of the n elements in view. This is more of a proof-of-concept, I guess?
     */
    fn is_visible_in_view<I>(idx: usize, iterator: I) -> bool
        where I: Iterator, 
              <I as Iterator>::Item: PartialOrd 
    {
        let mut size: usize = 0; 
        let mut max_val: Option<<I as Iterator>::Item> = None;
        for (jdx, val) in iterator.enumerate() {
            size = jdx + 1; 
            if jdx < idx { // "top" or "left" 
                if max_val.is_none() || max_val.as_ref().unwrap() < &val { max_val = Some(val); } 
                // Get maximum prior to (exclusive of) `@idx`
            } else if jdx == idx { // found @idx as `val`
                if max_val.is_none() || max_val.unwrap() < val { return true; }   
                max_val = Some(val); 
            } else { // "bottom" or "right"
                if max_val.as_ref().unwrap() <= &val { return false; }
            }
        }

        if size <= idx { 
            panic!(
                "[Day8::is_visible_in_view] Out of range: `iterator` has length {} but `{}` is given as `idx`", 
                size, 
                idx, 
            ); 
        }

        return true; 
    }

    fn scenic_score_of_idx_along_axis<T: PartialOrd>(idx: usize, axis: &[T]) -> usize {
        if let Some(ref_v) = axis.get(idx) {
            let (prev_slice, next_slice) = axis.split_at(idx); 
            let prev_itr = prev_slice.iter().rev();
            let prev_result = Day8::find_view_distance(ref_v, prev_itr); 

            let next_itr = next_slice[1..].iter();
            let next_result = Day8::find_view_distance(ref_v, next_itr); 
            return prev_result * next_result; 
        } else {
            panic!("[Day8::scenic_score_of_idx_along_axis] Out of bounds: `slice` has length {} but {} given as `idx`", axis.len(), idx); 
        }
    }

    fn find_eqincreasing_subsequence<T: PartialOrd>(ref_v: T, mut itr: impl Iterator<Item = T>) -> Vec<T> {
        let mut result: Vec<T> = Vec::new(); 
        while let Some(next) = itr.next() {
            match result.last() {
                None => result.push(next), // first tree encountered 
                Some(last) => {
                    if next > *last { // taller tree
                        result.push(next) 
                    } else if next == *last && *last < ref_v { // equally-sized tree that is shorter than viewpoint
                        result.push(next)
                    }
                }, 
            }
        }
        return result;
    }

    fn find_view_distance<T: PartialOrd>(ref_v: T, mut itr: impl Iterator<Item = T>) -> usize {
        let mut result: usize = 0;
        while let Some(next) = itr.next() {
            result += 1;
            if next >= ref_v { break; }
        }
        return result; 
    }

    fn parse2(input: &str) -> usize {
        let mut results: Vec<Vec<usize>> = Vec::new();  
        let row_itr = input.lines().map(str::as_bytes);
        let col_itr = ColumnIterator::from(input.lines()); 

        row_itr.for_each(|r| {
            let row_view_scenic_scores: Vec<usize> = r.iter().enumerate()
                .map(|(c_idx, _)| Day8::scenic_score_of_idx_along_axis(c_idx, r) )
                .collect(); 
            results.push(row_view_scenic_scores); 
        }); 

        let mut max_scenic_score: usize = 0;
        col_itr.enumerate().for_each(|(c_idx, c)| {
            c.iter().enumerate().for_each(|(r_idx, _)| {
                results[r_idx][c_idx] *= Day8::scenic_score_of_idx_along_axis(r_idx, &c); 
                if results[r_idx][c_idx] > max_scenic_score {
                    max_scenic_score = results[r_idx][c_idx]; 
                }
            })
        });
        println!("Done finding max scenic score."); 
        return max_scenic_score; 
    }

    fn parse(input: &str) -> i64 {
        let mut result_mat: Vec<Vec<bool>> = Vec::new(); 
        let row_itr = input.lines().map(|s| s.as_bytes() ); 
        let col_itr = ColumnIterator::from(input.lines()); 

        row_itr.for_each(|r| {
            let row_view_vis_result: Vec<bool> = r.iter().enumerate() 
                .map(|(c_idx, _)| Day8::is_visible_in_view(c_idx, r.iter())) 
                .collect(); 
            result_mat.push(row_view_vis_result); 
        });  
        
        col_itr.enumerate().for_each(|(c_idx, c)| {
            // println!("{}", c_idx); 
            c.iter().enumerate().for_each(|(r_idx, _)| {
                let col_vis_r_c = Day8::is_visible_in_view(r_idx, c.iter()); 
                result_mat[r_idx][c_idx] |= col_vis_r_c; 
            })
        }); 
        
        return result_mat.iter()
            .map(|r| {
                r.iter().fold(0 as i64, |acc, r| if *r { acc + 1 } else { acc })
            })
            .sum();  
    }
}

/**
Simple iterator for "transposing" the given `&str` input to a column-row configuration. Applies to 
both square and non-square `&str` formations, as the iterator takes whatever remains in its owned 
list of line iterators. 
 */
struct ColumnIterator<'a> {
    line_itrs: Vec<std::slice::Iter<'a, u8>>,
    is_consumed: bool, 
}

impl<'a> ColumnIterator<'a> {
    pub fn from(lines: Lines) -> ColumnIterator {
        let line_itrs: Vec<std::slice::Iter<u8>> = lines.map(|s| s.as_bytes().iter() ).collect(); 
        ColumnIterator { line_itrs, is_consumed: false }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = Vec<&'a u8>; // Variable length

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_consumed { return None; }

        let mut buf = Vec::new();
        let mut consumed_flag = true; 
        self.line_itrs.iter_mut()
            .for_each(|i| {
                if let Some(bc) = i.next() {
                    match bc {
                        10 => (),  // \n
                        _ => {
                            buf.push(bc); 
                            consumed_flag = false;
                        },  
                    }
                }
            }); 
        self.is_consumed = consumed_flag; 
        if self.is_consumed { return None; }
        return Some(buf); 
    }
}

#[cfg(test)]
mod tests {
    use crate::get_solutions::AOCSolutions;

    use super::{Day8, ColumnIterator}; 

    const SAMPLE_INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn test_is_visible_in_view_1() {
        let axis_1 = vec![3,5,1,2,6,7,3,1,2,5]; // visible idx: 0, 1, 4, 5, 9
        let answer_1 = vec![true, true, false, false, true, true, false, false, false, true]; 
        let result_1: Vec<bool> = axis_1.iter().enumerate()
            .map(|(idx, _)| {
                let iter_loop = axis_1.iter(); 
                Day8::is_visible_in_view(idx, iter_loop)
            })
            .collect(); 
        assert_eq!(answer_1, result_1); 
    }

    #[test]
    fn test_scenic_score(){
        let axis = "33549";
        let axis2 = "35353";
        assert_eq!(Day8::scenic_score_of_idx_along_axis(2, axis.as_bytes()), 4); 
        assert_eq!(Day8::scenic_score_of_idx_along_axis(3, axis2.as_bytes()), 2); 
    }

    #[test]
    fn test_scenic_score_2(){
        let result = Day8::parse2(SAMPLE_INPUT);
        println!("Done! {}", result); 
    }

    #[test]
    fn test_is_visible_in_view_2(){
        let axis_2 = "abccba"; // visible idx: all
        let answer_2 = vec![true, true, true, true, true, true]; 
        let result_2: Vec<bool> = axis_2.as_bytes().iter().enumerate()
            .map(|(idx, _)| {
                let iter_loop = axis_2.as_bytes().iter(); 
                Day8::is_visible_in_view(idx, iter_loop)
            })
            .collect(); 
        assert_eq!(answer_2, result_2); 
    }

    #[test]
    fn test_col_iter() {
        let test_str = "abc\ndef\nghi"; 
        let mut col_iter = ColumnIterator::from(test_str.lines()); 
        let sub_u8s = col_iter.next().unwrap(); 
        // assert!(col_iter.next().is_some()); 
        assert!(col_iter.next().is_some());
        assert!(col_iter.next().is_some());
        assert!(col_iter.next().is_none()); 
    }

    #[test]
    fn test_parse() {
        assert_eq!(Day8::parse(SAMPLE_INPUT), 21); 
    }
}