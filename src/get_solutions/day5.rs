use super::AOCSolutions; 
use std::io::{BufRead, Seek, SeekFrom};

pub struct Day5; 
type CargoLoad = Vec<Vec<char>>; // FILO, literal `stacks` of crates

impl Day5 {
    /**
    Obtains number of rows of the given `CargoLoad` environment from given `reader`, 
    which is assumed to be at top of buffer. 

    Returns `None` if EOF is at start of `reader`. Otherwise returns `Some`-wrapped number of rows 
    and rewinds `reader` to beginning-of-file.
     */
    fn get_row_count<R: BufRead + Seek>(reader: &mut R) -> Option<usize> {
        let mut line = String::new();
        let rows: usize; 
        if let Ok(read_amnt) = reader.read_line(&mut line) {
            match read_amnt {
                0 => {
                    eprintln!("[Day5::generate_init_warehouse_state] Malformed input: EOF at start of `reader`"); 
                    return None; 
                }, 
                _ => {
                    rows = read_amnt / 4; // inclusive of '\n'
                    reader.rewind()
                        .expect("[Day5::generate_init_warehouse_state] Cannot rewind given `reader` to beginning");  
                }, 
            }
        } else {
            panic!("[Day5::generate_init_warehouse_state] Cannot read from given input"); 
        }

        return Some(rows); 
    }

    /**
    ...
     */
    fn construct_init_cargo_load<R: BufRead>(reader: &mut R) -> Option<CargoLoad> {
        // loop until numerics in buffer, then read past `\n` in buffer
        // in the meantime, use [&u8] and split by each 4 `u8`s, trim whitespace and "[]", then add to each row if not ""
        return None; 
    }


}