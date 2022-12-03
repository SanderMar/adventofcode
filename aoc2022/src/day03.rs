use aoc_runner_derive::aoc;

use std::collections::HashSet;

// #[aoc_generator(day3)]
// pub fn day3_gen(input: &str) -> Vec<&str> {
//     input.split("\n").collect()
// }

#[aoc(day3, part1)]
pub fn part1(rucksacks: &str) -> u32 {
    rucksacks
        .split("\n")
        .map(|r| {
            let mut first_comp: HashSet<char> = HashSet::new();
            let (comp1, comp2) = r.split_at(r.len() / 2);
            let _: Vec<_> = comp1.chars().map(|c| first_comp.insert(c)).collect();
            let mut cur_char: char = 'a';
            for c in comp2.chars() {
                if first_comp.contains(&c) {
                    cur_char = c;
                    break;
                }
            }

            let char_as_u32: u32 = cur_char as u32;

            if char_as_u32 < 97 {
                char_as_u32 - 64 + 26
            } else {
                char_as_u32 - 96
            }
        })
        .sum::<u32>()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .array_chunks::<3>()
        .map(|g| {
            let r1: HashSet<char> = g.get(0).unwrap().chars().collect();
            let r2: HashSet<char> = g.get(1).unwrap().chars().collect();
            let r3: HashSet<char> = g.get(2).unwrap().chars().collect();
            let r1_r2: HashSet<_> = r1.intersection(&r2).cloned().collect();
            let r1_r2_r3: HashSet<_> = r1_r2.intersection(&r3).cloned().collect();
            r1_r2_r3.into_iter().map(|c| {
                let char_as_u32: u32 = c as u32;

                if char_as_u32 < 97 {
                    char_as_u32 - 64 + 26
                } else {
                    char_as_u32 - 96
                }
            }).sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part1(input), 157)
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part2(input), 70)
    }
}
