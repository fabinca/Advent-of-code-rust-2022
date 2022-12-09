use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn main() {
  let map = input_to_map("src/input_day8.txt");
  let visible_trees = find_visible_trees(&map);
  let best_scenic_score = find_best_scenic_score(&map);

  println!("{:?} {:?}", visible_trees, best_scenic_score);
  
}

fn find_best_scenic_score(map: &Vec<Vec<u32>>) -> u32 {
	let col_num = map[0].len();
	let mut best_scenic_score = 0;
	for row in 1..map.len() - 1{
		for col in 1..col_num - 1 {
			let scenic_score = get_scenic_score(&map, col, row);
			if scenic_score > best_scenic_score {
				best_scenic_score = scenic_score;
			}
		}
	}
	best_scenic_score
}

fn find_visible_trees(map: &Vec<Vec<u32>>) -> u32 {
	let col_num = map[0].len();
	let mut visible_trees = 0;
	for row in 1..map.len() - 1{
		for col in 1..col_num - 1 {
			if check_is_visible(&map, col, row) {
				visible_trees += 1;
			}
		}
	}
	visible_trees
}

fn get_scenic_score(map: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
	let mut score_left = 0;
	let mut score_right = 0;
	let mut score_up = 0;
	let mut score_down = 0;
	for col_var in (0..col).rev() {
		score_left += 1;
		if map[row][col_var] >= map[row][col] {
			break ;}}
	for col_var in col+1..map[0].len() {
		score_right += 1;
		if map[row][col_var] >= map[row][col] {
			break ;}}
	for row_var in (0..row).rev() {
		score_up += 1;
		if map[row_var][col] >= map[row][col] {
			break ;}}
	for row_var in row+1..map[0].len() {
		score_down += 1;
		if map[row_var][col] >= map[row][col] {
			break ;}}
	println!("[{row}, {col}]: {score_left} {score_right} {score_up} {score_down} ");
	score_left * score_right * score_down * score_up
}

fn check_is_visible(map: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
	let mut visible = true;
	for col_var in 0..col {
		if map[row][col_var] >= map[row][col] {
			visible = false;
			break ;
		}
	}
	if visible {return true}
	visible = true;
	for col_var in col+1..map[0].len() {
		if map[row][col_var] >= map[row][col] {

			visible = false;
			break ;
		}
	}
	if visible { 
		return true}
	visible = true;
	for row_var in 0..row {
		if map[row_var][col] >= map[row][col] {

			visible = false;
			break ;
		}
	}
	if visible {return true}
	visible = true;
	for row_var in row+1..map[0].len() {
		if map[row_var][col] >= map[row][col] {

			visible = false;
			break ;
		}
	}
	if visible { return true}

	
	false
}

fn input_to_map(filename: &str) -> Vec<Vec<u32>> {
	const RADIX: u32 = 10;
	let lines = read_lines(filename).expect("problem whie reading file");
	let mut map: Vec<Vec<u32>> = Vec::new();
	for line in lines {
		if let Ok(string) = line{
			map.push(string.chars().map(|c| c.to_digit(RADIX).unwrap()).collect_vec()); 
		}
	}
	map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}