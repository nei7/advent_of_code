use day_2::part_2;

fn main() {
    let input = std::fs::read_to_string("input_2.txt").unwrap();

    println!("{}", part_2(&input));
}
