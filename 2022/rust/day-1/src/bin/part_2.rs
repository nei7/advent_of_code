use day_1::part_2;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input_2.txt").unwrap();
    println!("{}", part_2(&input));
}
