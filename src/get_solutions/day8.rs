use super::AOCSolutions; 
use std::{iter::{Iterator, DoubleEndedIterator, Zip}, str::Lines}; 
use core::ops::Index; 

pub struct Day8;

impl Day8 {
    /** 
    Bare minimum for checking if value @ idx in given `iterator` is "visible" in the given axis, as 
    represented by the `iterator`, assumed to be a plain old `Iterator` -- no length or anything.

    This function **consumes** `iterator`. 

    Obviously this is a dumb solution to the given problem -- we should be able to find all visible 
    elements in a view in one single iteration. This is more of a proof-of-concept, I guess?
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
        let mut flag = false; 
        self.line_itrs.iter_mut()
            .for_each(|i| {
                if let Some(bc) = i.next() { 
                    buf.push(bc); 
                    flag = true; 
                }
            }); 
        self.is_consumed = flag; 
        return Some(buf); 
    }
}

#[cfg(test)]
mod tests {
    use super::Day8; 

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
}