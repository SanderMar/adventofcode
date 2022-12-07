use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

type Dir = (Vec<String>, u32);

/// Construct directory name from all directories on stack
fn name_from_stack(stack: &Vec<&str>) -> String {
    stack
        .iter()
        .map(|s| s.to_owned().to_owned() + "/")
        .collect()
}

#[aoc_generator(day7)]
pub fn gen_day7(term_lines: &str) -> HashMap<String, Dir> {
    let mut dir_stack: Vec<&str> = Vec::new();
    let mut dir_map: HashMap<String, Dir> = HashMap::new();

    term_lines.split("\n").for_each(|line| {
        let l_output: Vec<&str> = line.split(" ").collect();
        if l_output[0] == "$" {
            // Handled if command
            // Handle 'cd' command, ls can be ignored
            if l_output[1] == "cd" && l_output[2] == ".." {
                dir_stack.pop();
            } else if l_output[1] == "cd" {
                dir_stack.push(l_output[2]);
                dir_map.insert(name_from_stack(&dir_stack), (Vec::new(), 0));
            }
        } else {
            if l_output[0] == "dir" {
                dir_map
                    .entry(name_from_stack(&dir_stack))
                    .and_modify(|(d, _)| d.push(name_from_stack(&dir_stack) + l_output[1] + "/"));
            } else {
                let size: u32 = l_output[0].parse::<u32>().unwrap();
                dir_map
                    .entry(name_from_stack(&dir_stack))
                    .and_modify(|(_, s)| *s += size);
            }
        }
    });

    dir_map
}

/// Recursively get the size of a directory
fn get_dir_size(map: &HashMap<String, Dir>, dir_name: &str) -> u32 {
    let dir = map.get(dir_name).unwrap();
    if dir.0.is_empty() {
        return dir.1;
    } else {
        let mut sub_dir_size: u32 = 0;
        for sdir in &dir.0 {
            sub_dir_size += get_dir_size(map, sdir);
        }
        return sub_dir_size + dir.1;
    }
}

#[aoc(day7, part1)]
pub fn part1(map: &HashMap<String, Dir>) -> u32 {
    map.keys()
        .map(|k| get_dir_size(map, k))
        .filter(|&size| size <= 100000u32)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(map: &HashMap<String, Dir>) -> u32 {
    let total_space = 70000000u32;
    let needed_free = 30000000u32;
    let total_used = get_dir_size(map, "//");
    let max_used = total_space - needed_free;
    let to_delete = total_used - max_used;

    map.keys()
        .map(|k| get_dir_size(map, k))
        .filter(|&size| size >= to_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod generator_test {
    use super::*;

    #[test]
    fn input_website() {
        let input = "$ cd /\n\
                     $ ls\n\
                     dir a\n\
                     14848514 b.txt\n\
                     8504156 c.dat\n\
                     dir d\n\
                     $ cd a\n\
                     $ ls\n\
                     dir e\n\
                     29116 f\n\
                     2557 g\n\
                     62596 h.lst\n\
                     $ cd e\n\
                     $ ls\n\
                     584 i\n\
                     $ cd ..\n\
                     $ cd ..\n\
                     $ cd d\n\
                     $ ls\n\
                     4060174 j\n\
                     8033020 d.log\n\
                     5626152 d.ext\n\
                     7214296 k";
        let mut expected: HashMap<String, Dir> = HashMap::new();
        expected.insert(
            "//".to_owned(),
            (vec!["//a/".to_owned(), "//d/".to_owned()], 23352670),
        );
        expected.insert("//a/".to_owned(), (vec!["//a/e/".to_owned()], 94269));
        expected.insert("//a/e/".to_owned(), (Vec::new(), 584));
        expected.insert("//d/".to_owned(), (Vec::new(), 24933642));
        assert_eq!(gen_day7(input), expected);
    }
}

#[cfg(test)]
mod part1_test {
    use super::*;

    #[test]
    fn input_website() {
        let input = "$ cd /\n\
                     $ ls\n\
                     dir a\n\
                     14848514 b.txt\n\
                     8504156 c.dat\n\
                     dir d\n\
                     $ cd a\n\
                     $ ls\n\
                     dir e\n\
                     29116 f\n\
                     2557 g\n\
                     62596 h.lst\n\
                     $ cd e\n\
                     $ ls\n\
                     584 i\n\
                     $ cd ..\n\
                     $ cd ..\n\
                     $ cd d\n\
                     $ ls\n\
                     4060174 j\n\
                     8033020 d.log\n\
                     5626152 d.ext\n\
                     7214296 k";
        assert_eq!(part1(&gen_day7(input)), 95437)
    }
}

#[cfg(test)]
mod part2_test {
    use super::*;

    #[test]
    fn input_website() {
        let input = "$ cd /\n\
                     $ ls\n\
                     dir a\n\
                     14848514 b.txt\n\
                     8504156 c.dat\n\
                     dir d\n\
                     $ cd a\n\
                     $ ls\n\
                     dir e\n\
                     29116 f\n\
                     2557 g\n\
                     62596 h.lst\n\
                     $ cd e\n\
                     $ ls\n\
                     584 i\n\
                     $ cd ..\n\
                     $ cd ..\n\
                     $ cd d\n\
                     $ ls\n\
                     4060174 j\n\
                     8033020 d.log\n\
                     5626152 d.ext\n\
                     7214296 k";
        assert_eq!(part2(&gen_day7(input)), 24933642)
    }
}
