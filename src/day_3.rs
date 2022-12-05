use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  //read lines
  let lines = read_lines("./src/input_day3.txt").expect("Problem reading file");
  let mut sum = 0;
  let mut option = 1;
  let mut backpacks = vec![String::from(""),String::from("")];
  for line in lines {
    if let Ok(string) = line {
      match option {
        1 => backpacks[0] = string,
        2 => backpacks[1] = string,
        _ => {option = 0; sum += get_priority_of_badge(&backpacks[0], &backpacks[1], &string);},
      }
      option += 1;
      // sum += get_priority_of_misplaced_item(&string);
    }
  }
  println!("Total Score {}", sum);
  
}

fn get_badge(bp1: &str, bp2: &str, bp3: &str) -> char {
  for char in bp1.chars() {
    if bp2.contains(char) && bp3.contains(char) {
      println!("Badge char: {}", char);
      return char; 
    }
  }
   panic!("No badge occurs in all three");
}

fn get_priority_of_char(char: char) -> usize {
  let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let prio = chars.find(char).unwrap() + 1;
  println!("Prio: {}", prio);
  prio
}

fn get_priority_of_badge(bp1: &str, bp2: &str, bp3: &str) -> usize {
  let badge = get_badge(bp1, bp2, bp3);
  get_priority_of_char(badge)
}

fn get_priority_of_misplaced_item(string: &str) -> usize {
  let misplaced_char = get_misplaced_char(string);
  get_priority_of_char(misplaced_char)
}

fn get_misplaced_char(string: &str) -> char {
  let mid = string.len() / 2;
  let (first, sec) = string.trim().split_at(mid);
  for char in first.chars() {
    if sec.contains(char) { 
      println!("Misplaced char: {}", char);
      return char; }
  }
  panic!("No item misplaced");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}