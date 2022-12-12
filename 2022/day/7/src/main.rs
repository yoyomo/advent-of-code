use std::{fs, rc::Rc};

use regex::Regex;

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            files: vec![],
            directories: vec![],
        }
    }

    fn add_dir(&mut self, dir_name: String) {
        let new_directory = Directory::new(dir_name);
        self.directories.push(new_directory);
    }
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let lines: Vec<&str> = contents.split('\n').collect();

    // map out directories and files
    let mut tree: Directory = Directory::new("/".to_string());
    let root = Rc::new(&mut tree);
    let mut parent_directory = Rc::clone(&root);
    let mut current_directory = Rc::clone(&root);
    for line in lines {
        println!("{line}");

        if line.starts_with('$') {
            if line == "$ ls" {
                continue;
            }

            let re = Regex::new(r"\$ cd ([\w./]+)").unwrap();
            let groups = re.captures(line).unwrap();

            let directory_name = &groups[1];
            if directory_name == ".." {
                current_directory = Rc::clone(&parent_directory);
            } else if directory_name == "/" {
                current_directory = Rc::clone(&root);
            } else {
                parent_directory = Rc::clone(&current_directory);
                // current_directory = Rc::new(
                //     &mut current_directory
                //         .directories
                //         .iter()
                //         .find(|&dir| dir.name == directory_name)
                //         .unwrap(),
                // );
            }
        } else if line.starts_with("dir") {
            // is directory or list
            let re = Regex::new(r"dir ([\w./]+)").unwrap();
            let groups = re.captures(line).unwrap();

            let directory_name = &groups[1];
            match current_directory
                .directories
                .iter()
                .find(|&dir| dir.name == directory_name)
            {
                Some(_) => {}
                None => {
                    current_directory.add_dir(directory_name.to_string());
                }
            }
        } else {
            // if first is number and second is list
        }
    }

    println!("{:?}", tree);
}
