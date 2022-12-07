use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

type Dir = (Vec<String>, u32);

#[aoc_generator(day7)]
pub fn gen_day7(term_lines: &str) -> HashMap<&str, Dir> {
    let mut dir_stack: Vec<&str> = Vec::new();
    let mut dir_map: HashMap<&str, Dir> = HashMap::new();

    term_lines.split("\n").for_each(|line| {
        let l_output: Vec<&str> = line.split(" ").collect();
        if l_output[0] == "$" {
            // Handled if command
            // Handle 'cd' command, ls can be ignored
            if l_output[1] == "cd" && l_output[2] == ".." {
                dir_stack.pop();
            } else if l_output[1] == "cd" {
                dir_stack.push(l_output[2]);
                dir_map.insert(l_output[2], (Vec::new(), 0));
            }
        } else {
            if l_output[0] == "dir" {
                dir_map
                    .entry(dir_stack.last().unwrap())
                    .and_modify(|(d, _)| d.push(l_output[1].to_owned()));
            } else {
                let size: u32 = l_output[0].parse::<u32>().unwrap();
                dir_map
                    .entry(dir_stack.last().unwrap())
                    .and_modify(|(_, s)| *s += size);
            }
        }
    });

    dir_map
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
        let mut expected: HashMap<&str, Dir> = HashMap::new();
        expected.insert("/", (vec!["a".to_owned(), "d".to_owned()], 23352670));
        expected.insert("a", (vec!["e".to_owned()], 94269));
        expected.insert("e", (Vec::new(), 584));
        expected.insert("d", (Vec::new(), 24933642));
        assert_eq!(gen_day7(input), expected);
    }
}
