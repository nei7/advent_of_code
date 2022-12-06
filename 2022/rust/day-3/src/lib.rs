#![feature(iter_array_chunks)]

use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let letters = ('a'..='z')
        .into_iter()
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment_a = &line[0..sack_length];
            let compartment_b = &line[sack_length..];

            let common_char = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();

            letters.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result
}

pub fn part_2(input: &str) -> usize {
    let letters = ('a'..='z')
        .into_iter()
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

   let sum = input.lines().array_chunks::<3>().map(|[a, b, c]| {
       let c_char = a.chars().find(|ch| b.contains(*ch) && c.contains(*ch)).unwrap();

       letters.get(&c_char).unwrap()
    }).sum::<usize>();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 70);
    }
}
