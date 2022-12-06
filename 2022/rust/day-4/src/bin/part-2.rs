use day_4::part_2;

fn main() {
    let input = std::fs::read_to_string("input_1.txt").unwrap();

    println!("{}", part_2(&input));
}
