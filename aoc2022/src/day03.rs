use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

// #[aoc_generator(day3)]
// pub fn day3_gen(input: &str) -> Vec<&str> {
//     input.split("\n").collect()
// }

#[aoc(day3, part1)]
pub fn part1(rucksacks: &str) -> u32 {
    rucksacks.split("\n")
        .map(|r| {
            let mut first_comp: HashSet<char> = HashSet::new();
            let (comp1, comp2) = r.split_at(r.len()/2);
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
        }).sum::<u32>()
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
