use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

type Dir = (Vec<String>, Vec<u32>);

#[aoc_generator(day7)]
pub fn gen_day7(term_lines: &str) -> u32 {
    let mut dir_stack: Vec<&str> = Vec::new();
    let mut dir_map: HashMap<&str, Dir> = HashMap::new();

    term_lines
        .split("\n")
        .for_each(|line| {
            let l_output: Vec<&str> = line.split(" ").collect();
            if l_output[0] == "$" {  // Handled if command
                // Handle 'cd' command, ls can be ignored
                if l_output[1] == "cd" && l_output[2] == ".." {
                    dir_stack.pop();
                } else if l_output[1] == "cd" {
                    dir_stack.push(l_output[2]);
                    dir_map.insert(l_output[2], (Vec::new(), Vec::new()));
                }
            } else {
                if l_output[0] == "dir" {
                    dir_map[dir_stack.last().unwrap()];
                }
            }
        });


    0
}
