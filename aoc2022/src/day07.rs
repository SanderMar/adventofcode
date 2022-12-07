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
