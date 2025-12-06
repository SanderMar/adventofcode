use std::iter::zip;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn gen_day1(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let safe = zip(report, report.iter().skip(1)).try_fold(0, |acc, (x, y)| {
        let diff = y - x;
        let diff_abs = diff.abs();
        if (diff_abs < 1) || (diff_abs > 3) {
            // Check 1 <= y - x <= 3
            return Err(());
        } else if diff * acc < 0 {
            return Err(());
        }
        Ok(diff)
    });
    safe.is_ok()
}

#[aoc(day2, part1)]
pub fn part1(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

//#[aoc(day2, part1)]
//pub fn part1(reports: &Vec<Vec<i32>>) -> usize {
//    reports.iter().filter(|report| is_safe(report)).count()
//}

#[cfg(test)]
mod gen_tests {
    use super::*;

    #[test]
    fn input_small() {
        let input = "1 2 3 4\n5 6 7 8";
        let expected = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
        assert_eq!(gen_day1(input), expected);
    }
}

#[cfg(test)]
mod safe_tests {
    use super::*;

    #[test]
    fn all_incr() {
        let input = vec![1, 2, 3, 4];
        assert!(is_safe(&input))
    }
    #[test]
    fn all_dcr() {
        let input = vec![4, 3, 2, 1];
        assert!(is_safe(&input))
    }
    #[test]
    fn all_incr_big() {
        let input = vec![1, 2, 4, 5];
        assert!(is_safe(&input))
    }
    #[test]
    fn all_dcr_big() {
        let input = vec![5, 4, 2, 1];
        assert!(is_safe(&input))
    }

    #[test]
    fn incr_change() {
        let input = vec![1, 2, 3, 2];
        assert!(!is_safe(&input))
    }
    #[test]
    fn decr_change() {
        let input = vec![4, 3, 2, 3];
        assert!(!is_safe(&input))
    }

    #[test]
    fn incr_level() {
        let input = vec![1, 2, 2, 3];
        assert!(!is_safe(&input))
    }
    #[test]
    fn incr_level_begin() {
        let input = vec![1, 1, 2, 3];
        assert!(!is_safe(&input))
    }
    #[test]
    fn incr_level_end() {
        let input = vec![1, 2, 3, 3];
        assert!(!is_safe(&input))
    }

    #[test]
    fn decr_level() {
        let input = vec![3, 2, 2, 1];
        assert!(!is_safe(&input))
    }
    #[test]
    fn decr_level_begin() {
        let input = vec![3, 3, 2, 1];
        assert!(!is_safe(&input))
    }
    #[test]
    fn decr_level_end() {
        let input = vec![3, 2, 1, 1];
        assert!(!is_safe(&input))
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn all_incr() {
        let input = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
        assert_eq!(part1(&input), 2)
    }

    #[test]
    fn all_dcr() {
        let input = vec![vec![4, 3, 2, 1], vec![8, 7, 6, 5]];
        assert_eq!(part1(&input), 2)
    }

    #[test]
    fn incr_decr_mix() {
        let input = vec![vec![1, 2, 3, 4], vec![8, 7, 6, 5]];
        assert_eq!(part1(&input), 2)
    }

    #[test]
    fn incr_small_step() {
        let input = vec![vec![1, 2, 3, 4], vec![1, 2, 2, 4]];
        assert_eq!(part1(&input), 1)
    }

    #[test]
    fn incr_big_step() {
        let input = vec![vec![1, 2, 8, 9], vec![1, 2, 3, 4]];
        assert_eq!(part1(&input), 1)
    }

    #[test]
    fn decr_small_step() {
        let input = vec![vec![4, 2, 2, 1], vec![1, 2, 3, 4]];
        assert_eq!(part1(&input), 1)
    }

    #[test]
    fn decr_big_step() {
        let input = vec![vec![9, 8, 2, 1], vec![1, 2, 3, 4]];
        assert_eq!(part1(&input), 1)
    }
}

//#[cfg(test)]
//mod part2_tests {
//    use super::*;
//
//    #[test]
//    fn input_website() {
//        let frst = vec![1, 2, 3, 3, 3, 4];
//        let scnd = vec![3, 3, 3, 4, 5, 9];
//        assert_eq!(part2(&(frst, scnd)), 31)
//    }
//}
