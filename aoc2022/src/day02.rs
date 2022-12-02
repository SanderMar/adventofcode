use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|l| {
            let own_play: u8 = l.chars().nth(2).unwrap() as u8 - 'X' as u8;
            let opponent: u8 = l.chars().nth(0).unwrap() as u8 - 'A' as u8;
            let play_diff: i32 = (own_play as i32 - opponent as i32).rem_euclid(3);
            let score: u32 = own_play as u32 + 1;
            if play_diff == 1 {
                score + 6
            } else if play_diff == 0 {
                score + 3
            } else {
                score
            }
        })
        .sum()
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(part1(&input), 15)
    }
}
#[cfg(test)]
