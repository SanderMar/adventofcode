use aoc_runner_derive::{aoc, aoc_generator};

type Move = (Pos, isize);

#[non_exhaustive]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Pos(isize, isize);
impl Pos {
    pub const UP: Pos = Pos(-1, 0);
    pub const DOWN: Pos = Pos(1, 0);
    pub const LEFT: Pos = Pos(0, -1);
    pub const RIGHT: Pos = Pos(0, 1);
    pub fn get_dir(str_rep: &str) -> Pos {
        if str_rep == "U" {
            return Pos::UP;
        } else if str_rep == "D" {
            return Pos::DOWN;
        } else if str_rep == "L" {
            return Pos::LEFT;
        } else if str_rep == "R" {
            return Pos::RIGHT;
        }
        Pos(0, 0)
    }
    pub fn dist_to(self, other: &Self) -> isize {
        let in_sqrt = (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2);
        f64::sqrt(in_sqrt as f64) as isize
    }
}
impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

#[aoc_generator(day9)]
pub fn gen_day9(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            (Pos::get_dir(split[0]), split[1].parse::<isize>().unwrap())
        })
        .collect()
}
