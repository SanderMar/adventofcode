use aoc_runner_derive::aoc;

pub fn get_first_unique(input_stream: &[char], window_size: usize) -> usize {
    for (i, window) in input_stream.windows(window_size).enumerate() {
        let mut ordered = window.to_owned();
        ordered.sort();
        ordered.dedup();
        if ordered.len() == window_size {
            return i + window_size;
        }
    }
    0
}

#[aoc(day6, part1)]
pub fn part1(input_stream: &str) -> usize {
    let char_stream: Vec<char> = input_stream.chars().collect();
    let window_size = 4;

    get_first_unique(&char_stream, window_size)
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part1(input), 7)
    }
    #[test]
    fn input_website2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(input), 5)
    }
    #[test]
    fn input_website3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(input), 6)
    }
    #[test]
    fn input_website4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(input), 10)
    }
    #[test]
    fn input_website5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(input), 11)
    }
}
