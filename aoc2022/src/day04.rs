use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn day4_gen(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .split("\n")
        .map(|l| {
            let p: Vec<(i32, i32)> = l
                .split(",")
                .map(|p| {
                    let r: Vec<i32> = p.split("-").map(|v| v.parse::<i32>().unwrap()).collect();
                    (r.get(0).cloned().unwrap(), r.get(1).cloned().unwrap())
                })
                .collect();
            (p.get(0).cloned().unwrap(), p.get(1).cloned().unwrap())
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(range_pair: &[((i32, i32), (i32, i32))]) -> u32 {
    range_pair
        .into_iter()
        .map(|(r1, r2)| {
            let f = r1.0 - r2.0;
            let s = r1.1 - r2.1;
            if f * s <= 0 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod generator_tests {
    use super::*;

    #[test]
    fn single_item() {
        let expected = vec![((1, 2), (3, 4))];
        assert_eq!(day4_gen("1-2,3-4"), expected)
    }

    #[test]
    fn input_website() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let expected = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        assert_eq!(day4_gen(input), expected)
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn single_item() {
        let input = vec![((1, 4), (3, 4))];
        assert_eq!(part1(&input), 1)
    }

    #[test]
    fn input_website() {
        let input = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        assert_eq!(part1(&input), 2)
    }
}
