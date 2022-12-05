use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  //read lines
  let lines = read_lines("./src/input_day4.txt").expect("Problem reading file");
  let mut sum = 0;
  for line in lines {
    if let Ok(string) = line {
      let (first, sec) = config(&string);
      if contains_fully(&first, &sec) || contains_fully(&sec, &first) || overlap(&first, &sec) || overlap(&first, &sec) {
        sum += 1;
      }
  }
      
  }
  println!("Total Score {}", sum);
}

struct Range {
  lowest: i32,
  highest: i32
}

impl Range {
 fn new(input: Vec<&str>) -> Range{
  let first: i32 = input[0].trim().parse().expect("not a number");
  let sec: i32 = input[1].trim().parse().expect("not a number");
  Range{lowest: first, highest: sec}
 }
}

fn overlap(first: &Range, sec: &Range) -> bool {
  if first.lowest <= sec.lowest && first.highest >= sec.lowest {
    true
  }
  else if first.lowest <= sec.highest && first.highest >= sec.highest {
    true
  }
  else {false}
}

fn contains_fully(first: &Range, sec: &Range) -> bool {
  if first.lowest <= sec.lowest && first.highest >= sec.highest {
    true
  }
  else {false}
}

fn config(string: &str) -> (Range, Range)  {
  let split: Vec<&str> = string.trim().split(",").collect();
  let first: Vec<&str> = split[0].split("-").collect();
  let sec: Vec<&str> = split[1].split("-").collect();
  //let first_int
  (Range::new(first), Range::new(sec))
  
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}