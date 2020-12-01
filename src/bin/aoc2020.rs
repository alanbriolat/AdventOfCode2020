use aoc2020::data_path;
use aoc2020::solutions;

fn main() {
    println!("day01part1: {:?}", solutions::day01::part1(data_path!("day01_input.txt")).unwrap());
    println!("day01part2: {:?}", solutions::day01::part2(data_path!("day01_input.txt")).unwrap());
}
