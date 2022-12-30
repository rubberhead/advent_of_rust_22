use std::{path::Path, fs::File};
use std::io::{Error, Read}; 

pub mod get_solutions; 

pub fn parse_to_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let rf = File::open(path); 
    let mut buf = String::new(); 
    match rf {
        Ok(mut f) => {
            let rs = f.read_to_string(&mut buf);
            if let Ok(_) = rs { return Ok(buf) };
            return Err(rs.err().unwrap());  
        },  
        _ => return Err(rf.err().unwrap()), 
    }
}