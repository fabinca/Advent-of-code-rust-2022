use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
  let mut data = VecDeque::from(read_file("src/input_day6.txt"));
  
}

fn read_file(filename: &str) -> Vec<char> {
  let mut file = File::open(filename).expect("error while reading file");
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).expect("error while reading file");
  buf.iter().map(|b| *b as char).collect::<Vec<_>>()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}