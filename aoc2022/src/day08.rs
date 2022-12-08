use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn gen_day8(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(grid: &[Vec<usize>]) -> usize {
    grid.into_iter()
        .enumerate()
        .map(|(x, line)| {
            line.into_iter()
                .enumerate()
                .map(|(y, &tree)| {
                    if x == 0 || x == grid.len() - 1 {
                        return 1;
                    } else if y == 0 || y == line.len() - 1 {
                        return 1;
                    }
                    let mut result = true;
                    for i in 0..y {
                        if grid[x][i] >= tree {
                            result = false;
                            break;
                        }
                    }
                    if result {
                        return 1;
                    }
                    result = true;
                    for i in (y + 1)..line.len() {
                        if grid[x][i] >= tree {
                            result = false;
                            break;
                        }
                    }
                    if result {
                        return 1;
                    }
                    result = true;
                    for i in 0..x {
                        if grid[i][y] >= tree {
                            result = false;
                            break;
                        }
                    }
                    if result {
                        return 1;
                    }
                    result = true;
                    for i in (x + 1)..grid.len() {
                        if grid[i][y] >= tree {
                            result = false;
                            break;
                        }
                    }
                    if result {
                        return 1;
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum()
}
#[cfg(test)]
mod generator_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let excpected: Vec<Vec<usize>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let output = gen_day8(input);
        assert_eq!(output, excpected)
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(part1(&gen_day8(input)), 21)
    }
}
