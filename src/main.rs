use aoc_2025_xx::{file_parser::FileParser, solve, solve_pt1, solve_pt2};

fn main() {
    let my_file = FileParser::new("data/input.txt");
    println!("Part 1: {}", solve_pt1(my_file.get_str(), 1000));
    println!("Part 2: {}", solve_pt2(my_file.get_str()));
}
