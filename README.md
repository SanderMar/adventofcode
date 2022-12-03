# Advent of Code

My [Advent of Code](https://adventofcode.com/) puzzle solutions.

The solutions are made in rust.
As of 2022 day 3, nightly is used for array_chunk suppport.

I am far from an experienced rust programmer.
The puzzles are made for fun and to further familiarize myself with rust and test my knowledge.
I will try to solve as many puzzles as possible when I have time in between my studies.

## Running the code

I use [cargo-aoc](https://lib.rs/crates/cargo-aoc) to run my code and download my inputs.
My inputs are not included in the repository.  
To run the code, you will have to install cargo-aoc: `cargo install cargo-aoc`.
Then set up your session token using `cargo aoc credentials -s {token}`.
Finally `cd` into the desired year, run `cargo aoc input -y <year> -d <day> to fetch the inputs`
and run the solution with `cargo aoc -d <day>`.

