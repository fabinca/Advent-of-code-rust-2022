use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  //read lines
  let lines = read_lines("./src/input_day3.txt").expect("Problem reading file");
  let mut score = 0;
  for line in lines {
    if let Ok(string) = line
    {
      let split: Vec<&str> = string.trim().split(" ").collect();
      
  }
  println!("Total Score {}", score);
  
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}