use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut max_calories_vec = vec![0,0,0];
        let mut elf_calories: i32 = 0;
        for line in lines {
            if let Ok(calorie) = line {
              if calorie.trim().is_empty() {
                let calorie_to_beat: i32 = *max_calories_vec.iter().min().unwrap_or(&0);
                if elf_calories > calorie_to_beat {
                  let index = max_calories_vec.iter().position(|value| *value ==         calorie_to_beat).unwrap_or(4);
                  if index == 4 {println!("something went wrong")}
                  max_calories_vec[index] = elf_calories;
                }
                elf_calories = 0;
              }
              else {
                elf_calories += calorie.trim().parse::<i32>().unwrap();
              }
            }
        }
      let calorie_to_beat: i32 = *max_calories_vec.iter().min().unwrap_or(&0);
                if elf_calories > calorie_to_beat {
                  let index = max_calories_vec.iter().position(|value| *value ==         calorie_to_beat).unwrap_or(4);
                  if index == 4 {println!("something went wrong")}
                  max_calories_vec[index] = elf_calories;}
      println!("{:?}", max_calories_vec);
      println!("{}", max_calories_vec[0] + max_calories_vec[1] + max_calories_vec[2] )
    }
  else {
    println!("File not found")
  }
   println!("Done")
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}