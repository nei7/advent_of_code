use day_1::part_1;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input_1.txt").unwrap();
    println!("{}", part_1(&input));
}
