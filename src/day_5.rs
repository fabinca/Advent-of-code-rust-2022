use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  //read lines
  
  let mut cargo_plan = build_stacks("./src/input_day5_cargo.txt");
  let lines = read_lines("./src/input_day5.txt").expect("Problem reading file");
  for line in lines {
    if let Ok(string) = line {
      let (num, from, to) = read_move_info(string);
      let mut crane: Vec<char> = vec![];
      for _ in 0..num {
        let cargo = cargo_plan[from - 1].pop().expect("there should be a char");
        crane.push(cargo);
      }
      for _ in 0..num {
        let cargo = crane.pop().expect("there should be a char");
        cargo_plan[to - 1].push(cargo);
      }
      // for _ in 0..num {
      //   let cargo = cargo_plan[from - 1].pop().expect("there should be a char");
      //   cargo_plan[to - 1].push(cargo);
      // }
     }
  }
  for mut vec in cargo_plan{
    print!("{:?}", vec.pop().expect("expect a char"));
  }
}


fn read_move_info(string: String) -> ( usize,  usize,  usize){
  let split:  Vec<&str> = string.split_ascii_whitespace().collect();
  let num: usize = split[1].parse().expect("Expected a number");
  let from: usize = split[3].parse().expect("Expected a number");
  let to: usize = split[5].parse().expect("Expected a number");
  (num, from, to)
}


fn build_stacks(filename: &str) -> Vec<Vec<char>>{
  let lines = read_lines(filename).expect("Problem reading file");
  let mut plan = vec![vec![], vec![], vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
  for line in lines {
     if let Ok(string) = line {
       let char_pos = (1..string.len()).step_by(4);
       let plan_num = 0..9;
       for (i, n) in  char_pos.zip(plan_num){
         if string.chars().nth(i).unwrap() != ' ' {
           plan[n].insert(0, string.chars().nth(i).unwrap());
         }
       }
     }
  }
  plan
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}