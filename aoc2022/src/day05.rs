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
