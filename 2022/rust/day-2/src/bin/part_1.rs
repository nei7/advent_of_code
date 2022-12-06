use day_2::part_1;

fn main() {
    let input = std::fs::read_to_string("input_1.txt").unwrap();

    println!("{}", part_1(&input));
}
