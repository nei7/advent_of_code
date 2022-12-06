pub fn part_1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u32 {
    let mut res = input
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    res.sort_by(|a, b| b.cmp(a));

    res.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        println!("{}", part_1(INPUT));
        assert_eq!(part_1(INPUT), 24000);
    }

    #[test]
    fn test_part_2() {
        const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(part_2(INPUT), 45000);
    }
}
