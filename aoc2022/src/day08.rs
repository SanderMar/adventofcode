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
