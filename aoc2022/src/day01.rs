use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<u32> {
    let elves: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|cal| cal.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut elves_cals: Vec<u32> = elves.iter().map(|e| e.iter().sum()).collect();
    elves_cals.sort_by_key(|&c| std::cmp::Reverse(c));
    elves_cals
}

#[aoc(day1, part1)]
pub fn part1(elve_cals: &[u32]) -> u32 {
    match elve_cals.iter().max() {
        Some(max) => return *max,
        None => return 0,
    }
}

#[cfg(test)]
mod generator_tests {
    use super::*;

    #[test]
    fn single_item() {
        let expected = vec![300];
        assert_eq!(day1_gen("100\n200"), expected)
    }

    #[test]
    fn input_website() {
        let input = "1000\n\
                     2000\n\
                     3000\n\n\
                     4000\n\n\
                     5000\n\
                     6000\n\n\
                     7000\n\
                     8000\n\
                     9000\n\n\
                     10000";
        let expected = vec![24000, 11000, 10000, 6000, 4000];
        assert_eq!(day1_gen(input), expected)
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn single_item() {
        let input = vec![300];
        assert_eq!(part1(&input), 300)
    }

    #[test]
    fn input_website() {
        let input = vec![24000, 11000, 10000, 5000, 4000];
        assert_eq!(part1(&input), 24000)
    }
}
