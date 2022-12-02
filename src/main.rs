use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut max_calories = 0;
        let mut elf_calories = 0;
        for line in lines {
            if let Ok(calorie) = line {
              if calorie.trim().is_empty() {
                if elf_calories > max_calories {
                  max_calories = elf_calories;
                }
                elf_calories = 0;
              }
              else {
                elf_calories += calorie.trim().parse::<i32>().unwrap();
              }
            }
        }
      println!("{}", max_calories);
    }
  else {
    println!("File not found")
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}