use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  //read lines
  let lines = read_lines("./src/input_day2.txt").expect("Problem reading file");
  let mut score = 0;
  for line in lines {
    if let Ok(string) = line
    {
      let split: Vec<&str> = string.trim().split(" ").collect();
      score += win_draw_lose_part_2(split[1]);
      score += get_symbol_part_2(split[0], split[1]);}
      // score += win_draw_lose_score(split[0], split[1]);
      // score += symbol_score(split[1]);}
  }
  println!("Total Score {}", score);
  
}

fn get_symbol_part_2(elf: &str, me: &str) -> i32 {
  let result = match elf {
    "A" => match me {
      "X" => 3,
      "Y" => 1,
      "Z" => 2,
      _ => panic!("can't be this value"),
    },
    "B" => match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("can't be this value"),
      },

    "C" => match me {
      "X" => 2,
      "Y" => 3,
      "Z" => 1,
      _ => panic!("can't be this value"),
    },
    _ => panic!("can't be this value"),
  };
  result
}

fn win_draw_lose_part_2(my_val: &str) -> i32 {
  let result = match my_val {
    "X" => 0,
    "Y" => 3,
    "Z" => 6,
    _ => panic!("can't be this value"),
  };
  result
}

fn symbol_score(my_val: &str) -> i32 {
  let result = match my_val {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
    _ => panic!("can't be this value"),
  };
  result
}

fn win_draw_lose_score(elf: &str, me: &str) -> i32 {
  let result = match elf {
    "A" => match me {
      "X" => 3,
      "Y" => 6,
      "Z" => 0,
      _ => panic!("can't be this value"),
    },
    "B" => match me {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("can't be this value"),
      },

    "C" => match me {
      "X" => 6,
      "Y" => 0,
      "Z" => 3,
      _ => panic!("can't be this value"),
    },
    _ => panic!("can't be this value"),
  };
  result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}