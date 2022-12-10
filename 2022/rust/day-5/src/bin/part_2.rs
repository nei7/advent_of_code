use day_5::part_2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input_1.txt").unwrap();
    let res = part_2(&input);

    println!("{}", res);
}
