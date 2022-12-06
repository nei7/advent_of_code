use std::ops::RangeInclusive;

use nom::IResult;

use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, character::complete};

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u32(input)?;
    Ok((input, start..=end))
}

fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, (start, end)))
}

fn section_assigment(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, ranges) = separated_list1(newline, line)(input)?;

    Ok((input, ranges))
}

pub fn part_1(input: &str) -> usize {
    let (_, assigments) = section_assigment(input).unwrap();

    assigments
        .iter()
        .filter(|(range_x, range_y)| {
            range_x.clone().into_iter().all(|n| range_y.contains(&n))
                || range_y.clone().into_iter().all(|n| range_x.contains(&n))
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    let (_, assigments) = section_assigment(input).unwrap();

    assigments
        .iter()
        .filter(|(range_x, range_y)| {
            range_x.clone().into_iter().any(|n| range_y.contains(&n))
                || range_y.clone().into_iter().any(|n| range_x.contains(&n))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 4);
    }
}
