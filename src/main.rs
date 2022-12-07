use std::fs::File;
use itertools::Itertools;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
  let mut data = read_file("src/input_day6.txt");
  let first = data[0];
  let mut deq = VecDeque::from([first, first, first, first]);
  let mut count = 0;
  loop {
    deq.pop_front();
    let next = data.pop().expect("should be a char");
    if data.into_iter().unique() {
      deq.push_back(next);
      count += 1;
    }
    else {
      deq.push_back(next);
      break ;
    }
  }
  println!("{:?}, {}", deq, count);
}

fn read_file(filename: &str) -> Vec<char> {
  let mut file = File::open(filename).expect("error while reading file");
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).expect("error while reading file");
  buf.iter().map(|b| *b as char).collect::<Vec<_>>()
}