use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;  
use super::AOCSolutions; 

pub struct Day7; 

type FileSystem = HashMap<String, (usize, bool)>; // K: full path; V: (sum size, is_evaluated?)

impl AOCSolutions for Day7 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let fs = Day7::parse(&mut input.as_bytes());
        let sum = fs.iter()
            .filter_map(|(_, (size, _))| if size.to_owned() <= 100000 { Some(size.to_owned()) } else { None })
            .reduce(|acc, rhs| acc + rhs );
        if let None = sum {
            return Ok(0); 
        } else {
            return Ok(sum.unwrap().try_into().unwrap()); 
        }
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        const DISK_SIZE: usize = 70000000;
        const UPDATE_SIZE: usize = 30000000; 

        let fs = Day7::parse(&mut input.as_bytes()); 
        let total_usage = fs.get("/").unwrap().0; 
        if DISK_SIZE - total_usage >= UPDATE_SIZE { return Err(()); }
        let threshold = UPDATE_SIZE - (DISK_SIZE - total_usage); 
        let target_size = fs.iter()
            .filter_map(|(_, (size, _))| if *size >= threshold { Some(*size) } else { None } )
            .min();
        if let Some(size) = target_size {
            return Ok(size.try_into().unwrap()); 
        } else {
            return Err(()); 
        }
    }
}

impl Day7 {
    fn parse<R>(input: &mut R) -> FileSystem 
        where R: BufRead {
        let mut fs = FileSystem::new(); 
        let mut cwd = "".to_string(); 
        fs.insert(cwd.clone(), (0, false)); 

        loop {
            let mut line = String::new(); 
            if let Ok(read_amnt) = input.read_line(&mut line) {
                if read_amnt == 0 { 
                    fs.entry(cwd.clone()).and_modify(|(_, evaluated)| *evaluated = true ); 
                    return fs; 
                }
            } else {
                panic!("[Day7::parse] Cannot read from given `input`"); 
            }

            match line.find("$") {
                Some(0) => { // Command
                    fs.entry(cwd.clone()).and_modify(|(_, evaluated)| *evaluated = true ); 

                    let args: Vec<&str> = line.trim().split(' ').collect(); 
                    if args.len() < 2 { panic!("[Day7::parse] Illegal command: {}", line.trim()); }

                    match (args.get(1), args.get(2)) {
                        (Some(&"cd"), Some(&"..")) => { // cd w/ prev.lvl operator
                            if cwd.as_str() != "/" { 
                                cwd = cwd.rsplit_once('/').unwrap().0.to_string(); 
                            } 
                        }, 
                        (Some(&"cd"), Some(&"/")) => { // cd to root
                            cwd = "/".to_string(); 
                        }, 
                        (Some(&"cd"), Some(&dir_name)) => { // cd w/ dir_name
                            if &cwd != "/" { cwd.push('/'); } 
                            cwd.push_str(dir_name); 
                        }, 
                        (Some(&"ls"), None) => { // ls
                            fs.entry(cwd.clone()).or_insert((0, false)); 
                        }, 
                        _ => { // undefined cmd
                            panic!("[Day7::parse] Illegal command: {}", line.trim()); 
                        }
                    }
                }, 
                None => { // info dump
                    match fs.get(&cwd) {
                        Some((_, false)) => { 
                            if let Some(size) = line.trim().split(' ').next() { // first word in line: `dir` or size num
                                if let Ok(size) = size.parse::<usize>() { // file found -- size num
                                    Day7::update_all_sizes(size, &cwd, &mut fs); 
                                } else if size == "dir" { // sub-directory found -- `dir`
                                    ()
                                } else {
                                    panic!("[Day7::parse] Invalid `ls` output: {}", line.trim()); 
                                }
                            } else {
                                panic!("[Day7::parse] Invalid `ls` output: {}", line.trim()); 
                            }
                        }, 
                        Some((_, true)) => continue, 
                        None => panic!("[Day7::parse] `{}` not found in filesystem", cwd.as_str()),  
                    }; 
                }, 
                Some(_) => { // Err
                    panic!("[Day7::parse] Illegal command: {}", line.trim()); 
                }
            }
        }
    }

    fn update_all_sizes(delta: usize, cwd: &str, fs: &mut FileSystem) {
        println!("[Day7::update_all_sizes] Found file in `{}` of size `{}`", cwd, delta); 
        let mut cwd = String::from(cwd);
        if !cwd.ends_with('/') { cwd.push('/') }; 
        let mut pwd = String::new(); 
        
        while !cwd.is_empty() {
            let slash_idx = cwd.find('/').unwrap(); 
            let uri: String = cwd.drain(0..=slash_idx).collect();
            match pwd.as_str() {
                "" => pwd.push_str(&uri),   
                "/" => pwd.push_str(&uri[..uri.len() - 1]), 
                _ => {
                    pwd.push('/');
                    pwd.push_str(&uri[..uri.len() - 1]); 
                }, 
            }
            println!("[Day7::update_all_sizes] Modifying `{}`...", pwd.as_str());  
            fs.entry(pwd.clone()).and_modify(|(size, _)| *size += delta ); 
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Day7; 
    use super::AOCSolutions; 

    const SAMPLE_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";  
    
    #[test]
    fn test_get_star_1() {
        assert_eq!(Day7::get_star_1(SAMPLE_INPUT).unwrap(), 95437); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day7::get_star_2(SAMPLE_INPUT).unwrap(), 24933642); 
    }
}