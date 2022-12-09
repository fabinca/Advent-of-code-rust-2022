enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct Move {
    dir: Dir,
    num: i32,
}

fn get_dir(input: &str) -> Dir {
    println!("{}", input);
    match input {
        "L" => Dir::Left,
        "R" => Dir::Right,
        "U" => Dir::Up,
        "D" => Dir::Down,
        _ => panic!(),
    }
}

fn get_move(line: &str) -> Move {
    let split: Vec<&str> = line.trim().split(" ").collect();
    Move {
        dir: get_dir(split[0]),
        num: split[1].parse::<i32>().unwrap(),
    }
}

fn move_head_and_tail(
    a_move: &Move,
    visited_fields: &mut Vec<(i32, i32)>,
    head_pos: &mut Vec<(i32, i32)>,
) {
    let move_num = a_move.num;
    for _ in 0..move_num {
        match a_move.dir {
            Dir::Up => head_pos[0].1 += 1,
            Dir::Down => head_pos[0].1 -= 1,
            Dir::Left => head_pos[0].0 -= 1,
            Dir::Right => head_pos[0].0 += 1,
        }
		for part in 1..head_pos.len() {
			println!("{}", part);

			if !touch(head_pos[part], head_pos[part - 1]) {
				let x_diff = head_pos[part - 1].0 - head_pos[part].0;
				let y_diff = head_pos[part - 1].1 - head_pos[part].1;
				if x_diff > 0 {
					head_pos[part].0 += 1;
				}
				if x_diff < 0 {
					head_pos[part].0 -= 1;
				}
				if y_diff > 0 {
					head_pos[part].1 += 1;
				}
				if y_diff < 0 {
					head_pos[part].1 -= 1;
				}
        }}
		visited_fields.push(head_pos.last().unwrap().clone());
    }
}

fn touch(tail: (i32, i32), head: (i32, i32)) -> bool {
    if (tail.0 - head.0).abs() <= 1 && (tail.1 - head.1).abs() <= 1 {
        true
    } else {
        false
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(|line| get_move(line)).collect();
    let mut visited_fields: Vec<(i32, i32)> = vec![(0, 0)];
    let mut head_pos: Vec<(i32, i32)> = vec![(0, 0),(0,0)];
    for a_move in moves {
        move_head_and_tail(&a_move, &mut visited_fields, &mut head_pos);
    }
    visited_fields.sort();
    visited_fields.dedup();
    println!("{:?}", visited_fields);
    visited_fields.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(|line| get_move(line)).collect();
    let mut visited_fields: Vec<(i32, i32)> = vec![(0, 0)];
    let mut body_pos: Vec<(i32, i32)> = vec![(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0)];
    for a_move in moves {
        move_head_and_tail(&a_move, &mut visited_fields, &mut body_pos);
    }
    visited_fields.sort();
    visited_fields.dedup();
    println!("{:?}", visited_fields);
    visited_fields.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
	U 4
	L 3
	D 1
	R 4
	D 1
	L 5
	R 2";

    #[test]
    fn part_1_works() {
        let result = solve_part_1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2_works() {
        let result = solve_part_2(INPUT);
        assert_eq!(result, 1);
    }

	#[test]
    fn part_2_really_works() {
		let input = "R 5
		U 8
		L 8
		D 3
		R 17
		D 10
		L 25
		U 20";
        let result = solve_part_2(input);
        assert_eq!(result, 36);
    }

    #[test]
    fn touch_works() {
        let head = (0, 0);
        let not_touching_tails = [(2, 2), (1, 2)];
        let touching_tails = [(1, 1), (-1, 1), (0, 0), (-1, 0)];
        for tail in touching_tails {
            let result = touch(head, tail);
            assert_eq!(result, true);
        }
        for tail in not_touching_tails {
            let result = touch(head, tail);
            assert_eq!(result, false);
        }
    }
}
