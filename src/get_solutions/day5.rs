use super::AOCSolutions; 
use std::io::{BufRead, Seek, Cursor};

pub struct Day5; 
type CargoLoad = Vec<Vec<u8>>; // FILO, literal `stacks` of crates

impl AOCSolutions for Day5 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let mut csr = Cursor::new(input); 
        let row_count = Day5::get_row_count(&mut csr); 
        if row_count.is_none() {
            return Err(()); 
        }

        let cargo_load = Day5::construct_init_cargo_load(&mut csr, row_count.unwrap()); 
        if cargo_load.is_none() {
            return Err(()); 
        }

        let result = Day5::follow_instructions(&mut csr, &mut cargo_load.unwrap(), 9000); 
        if result.is_none() {
            return Err(()); 
        }

        println!("STAR 1: {}", result.unwrap()); 
        return Ok(1); 
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let mut csr = Cursor::new(input); 
        let row_count = Day5::get_row_count(&mut csr); 
        if row_count.is_none() {
            return Err(()); 
        }

        let cargo_load = Day5::construct_init_cargo_load(&mut csr, row_count.unwrap()); 
        if cargo_load.is_none() {
            return Err(()); 
        }

        let result = Day5::follow_instructions(&mut csr, &mut cargo_load.unwrap(), 9001); 
        if result.is_none() {
            return Err(()); 
        }

        println!("STAR 2: {}", result.unwrap()); 
        return Ok(1); 
    }
}


impl Day5 {
    /**
    Obtains number of rows of the given `CargoLoad` environment from given `reader`, 
    which is assumed to be at top of buffer. 

    Returns `None` if EOF is at start of `reader`. Otherwise returns `Some`-wrapped number of rows 
    and rewinds `reader` to beginning-of-file.
     */
    fn get_row_count<R>(reader: &mut R) -> Option<usize> 
        where R: BufRead + Seek {
        let mut line = String::new();
        let rows: usize; 
        if let Ok(read_amnt) = reader.read_line(&mut line) {
            match read_amnt {
                0 => {
                    eprintln!("[Day5::get_row_count] Malformed input: EOF at start of `reader`"); 
                    return None; 
                }, 
                _ => {
                    rows = read_amnt / 4; // inclusive of '\n'
                    reader.rewind()
                        .expect("[Day5::get_row_count] Cannot rewind given `reader` to beginning");  
                }, 
            }
        } else {
            panic!("[Day5::get_row_count] Cannot read from given `reader`"); 
        }

        return Some(rows); 
    }

    /**
    Constructs `CargoLoad` environment from given `reader` (assumed at top of buffer). 
    
    Returns `Ok`-wrapped constructed `CargoLoad` if input is well-formed, while aligning given 
    `reader` to the expected start of instructions. Otherwise returns `None` if input is malformed. 
     */
    fn construct_init_cargo_load<R>(reader: &mut R, expected_row_count: usize) -> Option<CargoLoad> 
        where R: BufRead {
        // loop until numerics in buffer, then read past `\n` in buffer (expected to be exactly 2 lines)
        // in the meantime, use [&u8] and split by each 4 `u8`s, trim whitespace and "[]", then add to each row if not ""
        let mut cargo_load = vec![Vec::<u8>::new(); expected_row_count]; 
        let expected_line_len = expected_row_count * 4; 

        loop {
            let mut line = String::new(); 
            if let Ok(read_amnt) = reader.read_line(&mut line) {
                match read_amnt {
                    0 => { // EOF
                        eprintln!("[Day5::construct_init_cargo_load] Malformed input: EOF at start of `reader`"); 
                        return None; 
                    }, 
                    read_amnt if read_amnt == expected_line_len => { // Crate or row count line
                        if line.as_bytes().iter().any(u8::is_ascii_digit) { // row count line
                            continue; 
                        }

                        for (idx, chunk) in line.as_bytes().chunks(4).enumerate() {
                            let mut chunk_iter = chunk.iter().filter(|u| u.is_ascii_alphabetic() ); 
                            if let Some(item) = chunk_iter.next() { // Has element, assumed to be 1 element
                                cargo_load[idx].push(item.clone()); 
                            } // else no element, do nothing
                        }
                    }, 
                    _ => { // Assumed "\n", maybe invalid but whatever -- expected behavior
                        cargo_load.iter_mut().for_each(|v| v.reverse()); // Should have used `Deque`
                        // Already aligned to instructions if well-formed
                        return Some(cargo_load); 
                    }, 
                }
            } else {
                panic!("[Day5::construct_init_cargo_load] Cannot read from given `reader`"); 
            }
        }
    }

    /**
    Alters given `cargo_load` in accordance to instructions provided in `reader`. 

    Returns `Some`-wrapped `String` containing top crates of each row (i.e., numbered column) after 
    performing instructions. Otherwise returns `None` if given instructions cannot be followed. 

    > **This operation is destructive** -- returned results are popped from `cargo_load` sub-vectors.  
     */
    fn follow_instructions<R>(reader: &mut R, cargo_load: &mut CargoLoad, model: usize) -> Option<String> 
        where R: BufRead {
        loop {
            let mut line = String::new(); 
            if let Ok(read_amnt) = reader.read_line(&mut line) {
                match read_amnt {
                    0 => { // EOF
                        let top_crates: Vec<u8> = cargo_load.iter_mut()
                            .map_while(|r| r.pop())
                            .collect(); 
                        if let Ok(s) = String::from_utf8(top_crates) {
                            return Some(s); 
                        } else {
                            panic!("[Day5::follow_instructions] Invalid `cargo_load` elements -- non-UTF8 encountered"); 
                        }
                    }, 
                    _ => { // Instruction
                        // Parse into config
                        let config: Vec<usize> = line.trim().split(' ')
                            .filter_map(|sp| 
                                if let Ok(n) = sp.parse::<usize>() { Some(n) } else { None } 
                            )
                            .collect(); 
                        if config.len() != 3 {
                            panic!("[Day5::follow_instructions] Undefined or malformed instruction \"{}\"", line); 
                        }

                        // Check existence of rows, obtain mut ref for from row (necessary?)
                        let crate_count = config[0]; 
                        let from = config[1] - 1;
                        let to = config[2] - 1; 
                        if from == to { continue; }
                        if to > cargo_load.len() {
                            eprintln!("[Day5::follow_instructions] Cannot follow instruction \"{}\": row {} does not exist", line.trim(), to);
                            return None;  
                        }
                        let from_row: &mut Vec<u8>; 
                        match cargo_load.get_mut(from) { 
                            Some(r) => from_row = r, 
                            None => {
                                eprintln!("[Day5::follow_instructions] Cannot follow instruction \"{}\": row {} does not exist", line.trim(), from); 
                                return None; 
                            }, 
                        }
                        // Check from row has enough crates
                        if from_row.len() < crate_count {
                            eprintln!(
                                "[Day5::follow_instructions] Cannot follow instruction \"{}\": row {} has {} < {} crates", 
                                line.trim(), from, from_row.len(), crate_count
                            );
                            return None; 
                        }

                        // Clone-and-truncate-and-append (maybe better solution in API?)
                        let mut to_be_moved: Vec<u8> = Vec::with_capacity(crate_count); 
                        from_row[from_row.len() - crate_count..].clone_into(&mut to_be_moved); 
                        match model {
                            9000 => to_be_moved.reverse(),  // "[moved] one at a time"
                            9001 => (), // "move multiple crates at once"
                            _ => panic!("[Day5::follow_instructions] Undefined model number: CrateMover {}", model), 
                        }
                        from_row.truncate(from_row.len() - crate_count); 
                        cargo_load[to].append(&mut to_be_moved); 
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day5; 
    use super::AOCSolutions; 
    use std::io::Cursor; 

    const SAMPLE_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_get_result_9000() {
        let mut br = Cursor::new(SAMPLE_INPUT);

        // get number of columns in representation
        let row_count = Day5::get_row_count(&mut br).unwrap(); 
        assert_eq!(row_count, 3); 

        // construct init cargo load (hold)
        let mut cargo_load = Day5::construct_init_cargo_load(&mut br, row_count).unwrap(); 
        assert_eq!(cargo_load[0], "ZN".as_bytes()); 
        assert_eq!(cargo_load[1], "MCD".as_bytes()); 
        assert_eq!(cargo_load[2], "P".as_bytes()); 
        
        // follow instructions
        let result = Day5::follow_instructions(&mut br, &mut cargo_load, 9000).unwrap(); 
        assert_eq!(result.as_str(), "CMZ");
        // Since destructive... 
        assert_eq!(cargo_load[0], "".as_bytes()); 
        assert_eq!(cargo_load[1], "".as_bytes()); 
        assert_eq!(cargo_load[2], "PDN".as_bytes()); 
    }

    #[test]
    fn test_get_result_9001() {
        let mut br = Cursor::new(SAMPLE_INPUT);

        // get number of columns in representation
        let row_count = Day5::get_row_count(&mut br).unwrap(); 
        assert_eq!(row_count, 3); 

        // construct init cargo load (hold)
        let mut cargo_load = Day5::construct_init_cargo_load(&mut br, row_count).unwrap(); 
        assert_eq!(cargo_load[0], "ZN".as_bytes()); 
        assert_eq!(cargo_load[1], "MCD".as_bytes()); 
        assert_eq!(cargo_load[2], "P".as_bytes()); 
        
        // follow instructions
        let result = Day5::follow_instructions(&mut br, &mut cargo_load, 9001).unwrap(); 
        assert_eq!(result.as_str(), "MCD");
        // Since destructive... 
        assert_eq!(cargo_load[0], "".as_bytes()); 
        assert_eq!(cargo_load[1], "".as_bytes()); 
        assert_eq!(cargo_load[2], "PZN".as_bytes()); 
    }

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day5::get_star_1(SAMPLE_INPUT).unwrap(), 1); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day5::get_star_2(SAMPLE_INPUT).unwrap(), 1); 
    }
}