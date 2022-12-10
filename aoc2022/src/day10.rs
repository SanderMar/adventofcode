use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn gen_day10(input: &str) -> Vec<isize> {
    let mut reg_val: Vec<isize> = vec![1];

    input.lines().for_each(|instr| {
        let splitted: Vec<&str> = instr.split(" ").collect();
        if splitted.len() == 2 {
            let cur_reg = *reg_val.last().unwrap();
            reg_val.push(cur_reg); // Simulate 2 tick duration
            let addx_val = splitted[1].parse::<isize>().unwrap();
            reg_val.push(cur_reg + addx_val); // Set reg size for next instruction
        } else {
            reg_val.push(*reg_val.last().unwrap());
        }
    });
    reg_val
}

#[aoc(day10, part1)]
pub fn part1(register_values: &[isize]) -> isize {
    register_values
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i + 1 >= 20 && ((i + 1) - 20) % 40 == 0)
        .map(|(i, reg_val)| (i + 1) as isize * reg_val)
        .sum()
}
