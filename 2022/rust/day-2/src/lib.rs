use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

// Rock A or X
// Paper B or Y
// Scissors C or Z
impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Unknown move".to_owned()),
        }
    }
}

pub fn part_2(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();
            let oponent_move = moves[0].parse::<Move>().unwrap();

            match moves[1] {
                "X" => match oponent_move {
                    Move::Paper => 1,
                    Move::Rock => 3,
                    Move::Scissors => 2,
                },
                "Y" => 3 + oponent_move as u32,
                "Z" => {
                    6 + match oponent_move {
                        Move::Paper => 3,
                        Move::Rock => 2,
                        Move::Scissors => 1,
                    }
                }
                _ => panic!("unexpected response"),
            }
        })
        .sum();

    result
}

pub fn part_1(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();

            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                None => panic!("failed to compare"),
            }
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 15);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 12);
    }
}
