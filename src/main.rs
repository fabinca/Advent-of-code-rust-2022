use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file_structure = parse_input("src/input_day7.txt");
    println!("filestructure: {:?}", file_structure);
    //Find all of the directories with a total size of at most 100000.
    // What is the sum of the total sizes of those directories?
    let result = get_sum_of_dirs_smaller_than_100000(&file_structure);

    println!("{:?}", result);
}

fn get_sum_of_dirs_smaller_than_100000(file_structure: &HashMap<String, u32>) -> u32 {
    let mut result = 0;
    for (_dir, size) in file_structure {
        if size <= &100000 {
            result += size;
        }
    }

    result
}

fn parse_input(filename: &str) -> HashMap<String, u32> {
    let mut new = true;
    let mut hash_map: HashMap<String, u32> = HashMap::new();
    hash_map.insert("/".to_string(), 0);
    let mut open_dirs: Vec<String> = vec!["/".to_string()];
    let lines = read_lines(filename).expect("Problem reading file");
    for line in lines {
        if let Ok(string) = line {
            let split: Vec<&str> = string.trim().split(" ").collect();
            if split[1] == "cd" {
                if split[2] == ".." {
                    open_dirs.pop();
                } else if split[2] == "/" {
                    open_dirs.clear();
                    open_dirs.push("/".to_string());
                } else {
                    open_dirs.push(split[2].to_string());
                }
            } else if split[1] == "ls" {
                println!("reading current directory{:?}", open_dirs.last());
                new = match hash_map[open_dirs.last().unwrap()] {
                    0 => true,
                    _ => false,
                }
            } else if split[0] == "dir" {
                hash_map.entry(split[1].to_string()).or_insert(0);
            } else {
                // add files for current directory if not already
                println!("{}", string);
                let filesize: u32 = split[0].parse().expect("should be a filesize");
                for dir in open_dirs.clone() {
                    *hash_map.entry(dir).or_insert(0) += filesize;
                }
            }
        }
    }
    hash_map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
