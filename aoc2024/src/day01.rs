use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::zip;

#[aoc_generator(day1)]
pub fn gen_day1(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut frst: Vec<i32> = vec![];
    let mut scnd: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        frst.push(splitted[0].parse::<i32>().unwrap());
        scnd.push(splitted[1].parse::<i32>().unwrap());
    });

    frst.sort();
    scnd.sort();

    (frst, scnd)
}

#[aoc(day1, part1)]
pub fn part1((x, y): &(Vec<i32>, Vec<i32>)) -> usize {
    zip(x, y).fold(0, |acc, (x, y)| acc + (x - y).abs() as usize)
}

#[aoc(day1, part2)]
pub fn part2((x, y): &(Vec<i32>, Vec<i32>)) -> usize {
    let mut count_map: HashMap<i32, usize> = HashMap::new();

    y.iter().for_each(|num| {
        let count = count_map.entry(*num).or_insert(0);
        *count += 1;
    });

    x.iter().fold(0, |acc, &num| {
        acc + num as usize * count_map.get(&num).unwrap_or(&0)
    })
}

#[cfg(test)]
mod gen_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        let frst = vec![1, 2, 3, 3, 3, 4];
        let scnd = vec![3, 3, 3, 4, 5, 9];
        let result_vec = gen_day1(input);
        assert_eq!(result_vec, (frst, scnd));
    }

    #[test]
    fn input_large_nums() {
        let input = "300   400\n500   1000\n200   700";
        let frst = vec![200, 300, 500];
        let scnd = vec![400, 700, 1000];
        let result_vec = gen_day1(input);
        assert_eq!(result_vec, (frst, scnd));
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website() {
        let frst = vec![1, 2, 3, 3, 3, 4];
        let scnd = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(part1(&(frst, scnd)), 11)
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn input_website() {
        let frst = vec![1, 2, 3, 3, 3, 4];
        let scnd = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(part2(&(frst, scnd)), 31)
    }
}
