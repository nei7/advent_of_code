use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    find_unique_sequence(input, 4)
}

pub fn part_2(input: &str) -> usize {
    find_unique_sequence(input, 14)
}

fn find_unique_sequence(input: &str, size: usize) -> usize {
    let seq = input
        .as_bytes()
        .windows(size)
        .position(|chars| chars.iter().all_unique())
        .map(|idx| idx + size)
        .unwrap();
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
