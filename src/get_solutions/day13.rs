use super::AOCSolutions; 
use std::cmp::Ordering; 

const LIST_BGN: u8 = '[' as u8; 
const LIST_END: u8 = ']' as u8; 
const ATOMIC_SEP: u8 = ',' as u8; 

const fn is_digit(u: u8) -> bool {
    '0' as u8 <= u && u <= '9' as u8
}

type ByteCursor<'a> = (&'a [u8], usize, usize); 

fn cursor_at_end(bc: &ByteCursor) -> bool {
    bc.1 == bc.0.len() - 1
}

fn increment_cursor(bc: &mut ByteCursor) {
    if bc.1 < bc.0.len() - 1 {
        bc.1 += 1; 
    }
}

fn increment_depth(bc: &mut ByteCursor) {
    bc.2 += 1;
}

fn decrement_depth(bc: &mut ByteCursor) {
    if bc.2 > 0 { bc.2 -= 1; } 
}

fn modify_cursor_to(bc: &mut ByteCursor, idx: usize) {
    if idx >= bc.0.len() {
        bc.1 = bc.0.len() - 1; 
    } else {
        bc.1 = idx; 
    }
}

fn compare_expr(left: &mut ByteCursor, right: &mut ByteCursor) -> Ordering {
    match (left.0[left.1], right.0[right.1]) {
        // Cons / Atom
        (LIST_BGN, LIST_BGN) => { // L = cons, R = cons
            // Move up cursor position
            increment_cursor(left);
            increment_cursor(right);

            // Deepen value depth for hierarchical ordering
            increment_depth(left); 
            increment_depth(right); 

            return compare_expr(left, right);
        }, 
        (LIST_BGN, u) if is_digit(u) => { // L = cons, R = atom
            increment_cursor(left); 
            increment_depth(left);
            let ord = compare_expr(left, right); 
            if ord != Ordering::Equal { 
                return ord;
            } else {
                increment_cursor(left);
                decrement_depth(left);

                increment_cursor(right);
                return compare_expr(left, right);
            }
        }, 
        (u, LIST_BGN) if is_digit(u) => { // L = atom, R = cons
            increment_cursor(right); 
            increment_depth(right); 
            // return compare_expr(left, right);
            let ord = compare_expr(left, right); 
            if ord != Ordering::Equal { 
                return ord;
            } else {
                increment_cursor(left);

                increment_cursor(right);
                decrement_depth(right);

                return compare_expr(left, right);
            }
        }, 
        (LIST_END, LIST_END) => { 
            // L/R cons both ends
            if cursor_at_end(&left) && cursor_at_end(&right) { 
                return Ordering::Equal; 
            } else {
                // New elem at same lvl available, continue
                increment_cursor(left);
                decrement_depth(left);

                increment_cursor(right);
                decrement_depth(right); 

                return compare_expr(left, right); 
            }
        }, 
        (LIST_END, u) if is_digit(u) || u == ATOMIC_SEP || u == LIST_BGN => {
            if left.2 >= right.2 { 
                // L cons ended but R has more elements (atomic or cons list)
                return Ordering::Less; 
            } else {
                increment_cursor(left); 
                decrement_depth(left); 
                return compare_expr(left, right); 
            }
        }, 
        (u, LIST_END) if is_digit(u) || u == ATOMIC_SEP || u == LIST_BGN => {
            if right.2 >= left.2 { 
                // R cons ended but L has more elements (atomic or cons list)
                return Ordering::Greater; 
            } else {
                increment_cursor(right);
                decrement_depth(right); 
                return compare_expr(left, right); 
            }  
        }, 
        
        // Comma
        (ATOMIC_SEP, ATOMIC_SEP) => { 
            // Next atomic elem (which may be list or num)
            increment_cursor(left);
            increment_cursor(right);
            return compare_expr(left, right);
        }, 
        (u_l, u_r) if is_digit(u_l) && is_digit(u_r) => {
            // Num encountered
            let l_border = left.1 + left.0[left.1..].iter().enumerate()
                .filter(|(_, u)| **u == LIST_END || **u == ATOMIC_SEP )
                .next().unwrap().0; // First ']' or ',' on left
            let r_border = right.1 + right.0[right.1..].iter().enumerate()
                .filter(|(_, u)| **u == LIST_END || **u == ATOMIC_SEP )
                .next().unwrap().0; // First ']' or ',' on right

            let left_val = String::from_utf8_lossy(&left.0[left.1..l_border]).parse::<usize>(); 
            let right_val = String::from_utf8_lossy(&right.0[right.1..r_border]).parse::<usize>(); 

            if left_val.is_err() || right_val.is_err() { 
                panic!(
                    "[day13::compare_expr] Cannot parse numeric atomic values `{}` or `{}`", 
                    &String::from_utf8_lossy(&left.0[left.1..l_border]), 
                    &String::from_utf8_lossy(&right.0[right.1..r_border]), 
                ); 
            }

            let left_val = left_val.unwrap(); 
            let right_val = right_val.unwrap(); 
            let ord = left_val.cmp(&right_val); 
            if ord != Ordering::Equal {
                return ord; 
            } else {
                // Modify lowest hierarchy (or both) 
                if left.2 == right.2 {
                    // L, R at same depth 
                    modify_cursor_to(left, l_border); 
                    modify_cursor_to(right, r_border);
                } else if left.2 < right.2 {
                    // L extrapolated to R's depth
                    // This happens when, say, L = [3, 7] and R = [[3, 1]] (what if R = [[3]]?)
                    // In this case 3 need to compare against the entirety of [3, 1] (Less is the result)
                    modify_cursor_to(right, r_border);
                } else {
                    // R extrapolated to L's depth
                    modify_cursor_to(left, l_border); 
                }

                return compare_expr(left, right);
            }
        }, 
        (u, ATOMIC_SEP) if is_digit(u) => {
            if left.2 <= right.2 { 
                return Ordering::Less; 
            } else {
                // [?] else? 
                unreachable!(); 
            }
        }, 
        (ATOMIC_SEP, u) if is_digit(u) => {
            if right.2 <= left.2 { 
                return Ordering::Greater; 
            } else {
                // [?] else? 
                unreachable!(); 
            }  
        }, 

        // Other: unreachable? 
        _ => unreachable!(), 
    }; 
}

pub struct Day13; 

impl AOCSolutions for Day13 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut count: i64 = 0; 
        let mut idx: i64 = 1; 
        for pair in input.split("\n\n").map(|chunk| chunk.lines().take(2).collect::<Vec<&str>>() ) {
            if pair.len() < 2 { 
                eprintln!("[Day13::get_star_1] Malformed input: Expect L/R pair but received \n\"{:?}\"", pair); 
                continue;
            }
            let mut left: ByteCursor = (pair[0].as_bytes(), 0, 0); 
            let mut right: ByteCursor = (pair[1].as_bytes(), 0, 0); 

            let ord = compare_expr(&mut left, &mut right); 
            if ord != Ordering::Greater { count += idx; }
            idx += 1;
        }
        return Ok(count); 
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let mut packets: Vec<&str> = input.split_whitespace().collect(); 
        let (div_1, div_2): (&str, &str) = ("[[2]]", "[[6]]");
        packets.append(&mut vec![div_1, div_2]); 

        packets.sort_by(|pack_1, pack_2| {
            let mut left: ByteCursor = (pack_1.as_bytes(), 0, 0); 
            let mut right: ByteCursor = (pack_2.as_bytes(), 0, 0); 
            return compare_expr(&mut left, &mut right); 
        }); 

        let idx_1 = packets.iter().enumerate().find(|(_, pk)| **pk == div_1).unwrap().0 + 1; 
        let idx_2 = packets.iter().enumerate().find(|(_, pk)| **pk == div_2).unwrap().0 + 1; 

        return Ok((idx_1 * idx_2).try_into().unwrap()); 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r"[[[[[[]]]]]]
[]"; 

    #[test]
    fn test_example_1() {
        let mut cursors = EXAMPLE_1.lines()
            .map(|s| (s.as_bytes(), 0 as usize, 0 as usize)); 

        let mut l_cursor = cursors.next().unwrap(); 
        let mut r_cursor = cursors.next().unwrap(); 

        assert_eq!(compare_expr(&mut l_cursor, &mut r_cursor), Ordering::Greater); 
    }

    const EXAMPLE_2: &str = r"[[[[[[3,7]]]]]]
[12]"; 

    #[test]
    fn test_example_2() {
        let mut cursors = EXAMPLE_2.lines()
            .map(|s| (s.as_bytes(), 0 as usize, 0 as usize)); 

        let mut l_cursor = cursors.next().unwrap(); 
        let mut r_cursor = cursors.next().unwrap(); 

        assert_eq!(compare_expr(&mut l_cursor, &mut r_cursor), Ordering::Less); 
    }

    const EXAMPLE_3: &str = r"[225870]
[225870,101293]";

    #[test]
    fn test_example_3() {
        let mut cursors = EXAMPLE_3.lines()
            .map(|s| (s.as_bytes(), 0 as usize, 0 as usize)); 

        let mut l_cursor = cursors.next().unwrap(); 
        let mut r_cursor = cursors.next().unwrap(); 

        assert_eq!(compare_expr(&mut l_cursor, &mut r_cursor), Ordering::Less); 
    }

    const EXAMPLE_4: &str = r"[[1],[2,3,4]]
[[1],4]";

    #[test]
    fn test_example_4() {
        let mut cursors = EXAMPLE_4.lines()
            .map(|s| (s.as_bytes(), 0 as usize, 0 as usize)); 

        let mut l_cursor = cursors.next().unwrap(); 
        let mut r_cursor = cursors.next().unwrap(); 

        assert_eq!(compare_expr(&mut l_cursor, &mut r_cursor), Ordering::Less); 
    }

    const EXAMPLE_5: &str = r"[[1],4]
[1,1,2]"; 

    #[test]
    fn test_example_freeform() {
        let mut cursors = EXAMPLE_5.lines()
            .map(|s| (s.as_bytes(), 0 as usize, 0 as usize)); 

        let mut l_cursor = cursors.next().unwrap(); 
        let mut r_cursor = cursors.next().unwrap(); 

        assert_ne!(compare_expr(&mut l_cursor, &mut r_cursor), Ordering::Greater); 
    }

    const SAMPLE_INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day13::get_star_1(SAMPLE_INPUT).unwrap(), 13); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day13::get_star_2(SAMPLE_INPUT).unwrap(), 140); 
    }
}