use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn main() {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read file");

    let root_dir = Rc::new(RefCell::new(Directory::new("/", None)));

    let mut current_dir = &root_dir;
    let mut dirs = vec![Rc::clone(&root_dir)];

    for line in input_data.lines() {
        if line.starts_with("$") {
            let input: Vec<&str> = line.split_terminator(" ").collect();
            
            println!("{:?}", current_dir.borrow().name);
            match input[1] {
                "cd" => match input[2] {
                    ".." => {current_dir = current_dir.borrow().parent.as_ref().unwrap();},
                    _ => {current_dir = &Rc::new(RefCell::new(Directory::new(input[2], Some(Rc::clone(&current_dir)))));}
                },
                _ => continue,
            }
        } else {
            let (first, second) = line.split_once(" ").unwrap();
            match first {
                "dir" => {
                    continue
                }
                _ => {
                    current_dir.borrow_mut().files.push(File {
                        name: second.to_string(),
                        size: first.parse().unwrap(),
                    });
                }
            }
        }
    }
    println!("{:?}", current_dir);
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}
#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl Directory {
    fn new(name: &str, parent: Option<Rc<RefCell<Directory>>>) -> Directory {
        Directory {
            name: name.to_string(),
            parent: match parent{
                Some(parent) => Some(parent),
                None => None,
            },
            files: Vec::new(),
        }
    }
}

// impl File {
//     fn new(name: String, size: u32) -> File {
//         File { name, size }
//     }
// }

// impl Directory {
//     fn new(name: String) -> Rc<Directory> {
//         Rc::new(Directory {
//             name,
//             parent: None,
//             sub_directories: Vec::new(),
//             files: Vec::new(),
//         })
//     }
//     fn new_from_parent(name: String, parent: Rc<Directory>) -> Rc<Directory> {
//         Rc::new(Directory {
//             name,
//             parent,
//             sub_directories: Vec::new(),
//             files: Vec::new(),
//         })
//     }

//     fn add_sub_directory(&mut self, sub_directory: Rc<Directory>) {
//         self.sub_directories.push(sub_directory);
//     }

//     fn add_file(&mut self, file: File) {
//         self.files.push(file);
//     }

//     fn get_size(&self) -> u32 {
//         let mut size = 0;
//         for file in &self.files {
//             size += file.size;
//         }
//         for sub_directory in &self.sub_directories {
//             size += sub_directory.get_size();
//         }
//         size
//     }

//     fn get_sub_directory(&mut self, name: &str) -> Option<&mut Rc<Directory>> {
//         for sub_directory in &mut self.sub_directories {
//             if sub_directory.name == name {
//                 return Some(sub_directory);
//             }
//         }
//         None
//     }
// }
