use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, ops::AddAssign};

type Move = (Pos, isize);

#[non_exhaustive]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Pos(isize, isize);
impl Pos {
    pub const UP: Pos = Pos(-1, 0);
    pub const DOWN: Pos = Pos(1, 0);
    pub const LEFT: Pos = Pos(0, -1);
    pub const RIGHT: Pos = Pos(0, 1);
    pub fn get_dir(str_rep: &str) -> Pos {
        if str_rep == "U" {
            return Pos::UP;
        } else if str_rep == "D" {
            return Pos::DOWN;
        } else if str_rep == "L" {
            return Pos::LEFT;
        } else if str_rep == "R" {
            return Pos::RIGHT;
        }
        Pos(0, 0)
    }
    pub fn dist_to(self, other: &Self) -> isize {
        let in_sqrt = (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2);
        f64::sqrt(in_sqrt as f64) as isize
    }
}
impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

#[aoc_generator(day9)]
pub fn gen_day9(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            (Pos::get_dir(split[0]), split[1].parse::<isize>().unwrap())
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(moves: &[Move]) -> usize {
    // 1 == head, 2 == tail
    let mut head_pos: Pos = Pos(0, 0);
    let mut tail_pos: Pos = Pos(0, 0);
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(tail_pos);

    moves.into_iter().for_each(|&(dir, c)| {
        for _ in 0..c {
            head_pos += dir;
            if tail_pos.dist_to(&head_pos) > 1 {
                tail_pos += dir;
                let diag: bool = tail_pos.0 - head_pos.0 != 0 && tail_pos.1 - head_pos.1 != 0;
                if diag && dir.0 != 0 {
                    tail_pos.1 += head_pos.1 - tail_pos.1;
                } else if diag && dir.1 != 0 {
                    tail_pos.0 += head_pos.0 - tail_pos.0;
                }
                visited.insert(tail_pos);
            }
        }
    });

    visited.into_iter().count()
}

fn perform_move(rope: &mut [Pos], dir: Pos, i: usize) {
    if dir == Pos(0, 0) || i == rope.len() {
        return
    }
    if i == 0 {
        rope[i] += dir;
        return perform_move(rope, dir, i+1)
    }

    let pred_pos = rope[i - 1];
    let mut final_dir = Pos(0, 0);
    if rope[i].dist_to(&pred_pos) > 1 {
        let mut tmp_pos = rope[i];
        tmp_pos += dir;
        final_dir = dir;

        let check: bool = dir.0.abs() > 0 && dir.1.abs() > 0;
        let diag: bool = tmp_pos.0 - pred_pos.0 != 0 && tmp_pos.1 - pred_pos.1 != 0 && !check;
        if diag && dir.0 != 0 {
            final_dir.1 += pred_pos.1 - tmp_pos.1;
        } else if diag && dir.1 != 0 {
            final_dir.0 += pred_pos.0 - tmp_pos.0;
        }
    }
    rope[i] += final_dir;
    return perform_move(rope, final_dir, i+1)
}

#[aoc(day9, part2)]
pub fn part2(moves: &[Move]) -> usize {
    // 1 == head, 2 == tail
    let mut rope: Vec<Pos> = vec![Pos(0, 0); 10];
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(Pos(0, 0));

    moves.into_iter().for_each(|&(dir, c)| {
        for _ in 0..c {
            perform_move(&mut rope, dir, 0);
            visited.insert(*rope.last().unwrap());
            // show_rope(&rope);
        }
        println!("{:?}", rope);
    });

    visited.into_iter().count()
}

pub fn show_rope(rope: &[Pos]) {
    let max_x: usize = rope.iter().map(|&x| x.0.abs()).max().unwrap() as usize;
    let max_y: usize = rope.iter().map(|&y| y.1.abs()).max().unwrap() as usize;
    let mut grid = vec![vec!["."; max_y + 1]; max_x + 1];
    rope.iter().for_each(|pos| grid[pos.0.abs() as usize][pos.1.abs() as usize] = "o");
    grid.iter().rev().for_each(|row| println!("{:?}", row));
}

#[cfg(test)]
mod generator_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let expected: Vec<Move> = vec![
            (Pos(0, 1), 4),
            (Pos(-1, 0), 4),
            (Pos(0, -1), 3),
            (Pos(1, 0), 1),
            (Pos(0, 1), 4),
            (Pos(1, 0), 1),
            (Pos(0, -1), 5),
            (Pos(0, 1), 2),
        ];
        assert_eq!(gen_day9(input), expected)
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(part1(&gen_day9(input)), 13)
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(part2(&gen_day9(input)), 1)
    }

    #[test]
    fn large_input() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part2(&gen_day9(input)), 36)
    }
}
