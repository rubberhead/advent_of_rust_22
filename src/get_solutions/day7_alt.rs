use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::rc::*; 
use std::cell::RefCell; 

/**
Working on linked-list-like implementations in Rust... 
It seems the most "safe" way is to use `Rc/Weak` and `RefCell` smart pointers to enable multiple 
ownership and interior mutability, otherwise lifetime would be hard (impossible?) to deal with.
 */

type Link<T> = Rc<RefCell<T>>;
type WeakLink<T> = Weak<RefCell<T>>;  

#[derive(Debug)]
struct Dir {
    parent: WeakLink<Dir>, 
    children: Box<HashMap<String, WeakLink<Dir>>>, 
    size: usize, 
    name: String, 
}

impl Dir {
    pub fn new_root() -> Self {
        Self { 
            parent: WeakLink::new(), 
            children: Box::new(HashMap::new()), 
            size: 0, 
            name: String::from("/"), 
        }
    }

    pub fn to_full_name(&self) -> String {
        let mut full_name = self.name.clone(); 
        while let Some(parent) = Weak::upgrade(&self.parent) {
            full_name.push_str(&parent.borrow().to_full_name()); 
        }


        return full_name; 
    }
}

struct FileSystem {
    dir_map: HashMap<String, Link<Dir>>, 
    work_dir: Link<Dir>,
}

impl FileSystem {
    pub fn new() -> Self {
        let root_dir = Rc::new(RefCell::new(Dir::new_root()));
        let dir_map = HashMap::from([(String::from("/"), Rc::clone(&root_dir))]);  
        Self { dir_map, work_dir: root_dir }
    }

    pub fn mkdir(&mut self, dir_name: &str) {
        let nd = self.dir_map.entry(String::from(dir_name))
            .or_insert_with(|| {
                let new_dir = Rc::new(RefCell::new(Dir::new_root())); 
                RefCell::borrow_mut(&new_dir).name = dir_name.to_string(); // Change name of new_dir
                return new_dir; 
            }); 
        RefCell::borrow_mut(nd).parent = Rc::downgrade(&self.work_dir); // Change parent of new_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkdir() {
        let mut fs = FileSystem::new(); 
        fs.mkdir("fasd"); 
        assert_eq!(fs.dir_map.len(), 2); 

        let root_dir = fs.dir_map.remove("/").unwrap();
        let new_dir = fs.dir_map.remove("fasd").unwrap();
        let parent_dir = new_dir.borrow().parent.upgrade().unwrap(); 
        assert_eq!(root_dir.as_ptr(), parent_dir.as_ptr()); 
        // println!("done!"); 
    }
}