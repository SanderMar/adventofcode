use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::Integer;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    name: String,
    l: String,
    r: String,
}

type NodeMap = HashMap<String, Node>;

#[aoc_generator(day8)]
pub fn gen_day8(input: &str) -> (String, NodeMap) {
    let first: Vec<&str> = input.split("\n\n").collect();

    let mut node_map: NodeMap = HashMap::new();

    for n in first[1].split("\n") {
        let split_eq: Vec<&str> = n.split("=").map(|s| s.trim()).collect();
        let name = split_eq[0].to_owned();
        let dest: Vec<String> = split_eq[1]
            .split(",")
            .map(|dn| {
                dn.trim()
                    .chars()
                    .filter(|&c| c != '(' && c != ')')
                    .collect()
            })
            .collect();
        node_map.insert(
            name.to_owned(),
            Node {
                name,
                l: dest[0].to_owned(),
                r: dest[1].to_owned(),
            },
        );
    }

    (first[0].to_owned(), node_map)
}

fn steps_to_end(start: &str, complete: fn(&str) -> bool, dir: &str, map: &NodeMap) -> usize {
    let mut steps = 0;
    let mut current = start;

    while !complete(current) {
        current = match dir.as_bytes()[steps % dir.len()] as char {
            'L' => &map[current].l,
            'R' => &map[current].r,
            _ => "ZZZ",
        };
        steps += 1;
    }

    steps
}

#[aoc(day8, part1)]
pub fn part1(input: &(String, NodeMap)) -> usize {
    let (directions, map) = input;

    fn check(node: &str) -> bool {
        node == "ZZZ"
    }

    steps_to_end("AAA", check, directions, map)
}

#[aoc(day8, part2)]
pub fn part2(input: &(String, NodeMap)) -> usize {
    let (directions, map) = input;

    let starting: Vec<String> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_owned())
        .collect();

    fn check(node: &str) -> bool {
        node.ends_with('Z')
    }

    let steps: Vec<usize> = starting
        .par_iter()
        .map(|n| steps_to_end(n, check, directions, map))
        .collect();

    steps.iter().fold(1, |a, b| a.lcm(b))
}

#[cfg(test)]
mod gen_tests {
    use super::*;

    #[test]
    fn input_website_turncated() {
        let input = "RL\n\nAAA = (BBB, BBB)\nBBB = (ZZZ, AAA)\nZZZ = (ZZZ, ZZZ)";
        let expected_dir = "RL".to_owned();
        let expected_map: NodeMap = HashMap::from([
            (
                "AAA".to_owned(),
                Node {
                    name: "AAA".to_owned(),
                    l: "BBB".to_owned(),
                    r: "BBB".to_owned(),
                },
            ),
            (
                "BBB".to_owned(),
                Node {
                    name: "BBB".to_owned(),
                    l: "ZZZ".to_owned(),
                    r: "AAA".to_owned(),
                },
            ),
            (
                "ZZZ".to_owned(),
                Node {
                    name: "ZZZ".to_owned(),
                    l: "ZZZ".to_owned(),
                    r: "ZZZ".to_owned(),
                },
            ),
        ]);
        let (result_dir, result_map) = gen_day8(input);
        assert_eq!(result_dir, expected_dir);
        assert_eq!(result_map, expected_map);
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn input_website1() {
        let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&gen_day8(input)), 2)
    }

    #[test]
    fn input_website2() {
        let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&gen_day8(input)), 6)
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn input_website() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        assert_eq!(part2(&gen_day8(input)), 6)
    }
}
