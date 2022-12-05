use aoc_runner_derive::{aoc, aoc_generator};

/// Custom type to represent a move
/// (count, from, to)
type Move = (u32, u32, u32);

/// Extract the moves from lines of a string
pub fn get_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .map(|l| {
            let mut c_move = l
                .trim()
                .split(" ")
                .filter(|s| s.parse::<u32>().is_ok()) // Only keep the integers
                .map(|s| s.parse().unwrap());
            (
                c_move.next().unwrap(),
                c_move.next().unwrap(),
                c_move.next().unwrap(),
            )
        })
        .collect()
}

/// Extract the initial stacks from the string representation
pub fn get_stacks(stacks: &str) -> Vec<Vec<char>> {
    // TODO: Look to improve this peice of code
    //       Could be done by only expexting character at % 4 positions
    let mut iter = stacks.lines().rev();
    let column_line = iter.next().unwrap();
    let indices: Vec<usize> = column_line
        .char_indices()
        .filter(|(_i, c)| c.is_numeric())
        .map(|(i, _c)| i)
        .collect();
    let mut stack: Vec<Vec<char>> = vec![Vec::new(); indices.len()];

    let _: Vec<_> = iter
        .map(|l| {
            let _: Vec<_> = indices
                .iter()
                .enumerate()
                .filter(|(_i, &s_i)| l.chars().nth(s_i).unwrap().is_alphabetic())
                .map(|(v_i, &s_i)| {
                    stack[v_i].push(l.chars().nth(s_i).unwrap());
                })
                .collect();
        })
        .collect();

    stack
}

#[aoc_generator(day5)]
pub fn day5_gen(input: &str) -> (Vec<Move>, Vec<Vec<char>>) {
    let splitted: Vec<&str> = input.split("\n\n").collect();
    let moves = get_moves(splitted.get(1).unwrap());
    let stacks = get_stacks(splitted.get(0).unwrap());
    (moves, stacks)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Move>, Vec<Vec<char>>)) -> String {
    let moves = &input.0;
    let mut stacks = input.1.clone();

    let _: Vec<_> = moves
        .iter()
        .map(|&(c, f, t)| {
            for _ in 0..c {
                let top = stacks[f as usize - 1].pop().unwrap();
                stacks[t as usize - 1].push(top);
            }
        })
        .collect();

    stacks
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.last().unwrap())
        .collect()
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\n\
                     move 1 from 2 to 1\n\
                     move 3 from 1 to 3\n\
                     move 2 from 2 to 1\n\
                     move 1 from 1 to 2";
        let generated = day5_gen(input);
        assert_eq!(part1(&generated), "CMZ".to_owned())
    }
}
