use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  //read lines
  let lines = read_lines("./src/input_day4.txt").expect("Problem reading file");
  let cargo_plan: Vec<&str> = lines.iter().collect();
  
  
  println!("Total Score {:?}", cargo_plan);
}


fn build_stacks(input: Vec<&str>) -> Vec<Vec<&str>>{
  vec![input]
}




fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}