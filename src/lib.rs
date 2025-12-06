use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

pub mod file_parser;
use crate::file_parser::FileParser;

pub fn solve_pt1(input_file: &str) -> u64 {
    0
}

pub fn solve_pt2(input_file: &str) -> u64 {
    0
}

pub fn solve(input_file: &str) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let my_file = FileParser::new("data/example.txt");
        let (part_1, part_2) = solve(my_file.get_str());
        assert_eq!(part_1, 0);
        assert_eq!(part_2, 0);
    }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 0);
        assert_eq!(solve_pt2(my_file.get_str()), 0);
    }

    #[test]
    fn actual() {
        let my_file = FileParser::new("data/input.txt");
        let (part_1, part_2) = solve(my_file.get_str());
        assert_eq!(part_1, 0);
        assert_eq!(part_2, 0);
    }

    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 0);
        assert_eq!(solve_pt2(my_file.get_str()), 0);
    }
}
