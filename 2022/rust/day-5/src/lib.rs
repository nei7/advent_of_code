use std::vec;

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{self, alpha1, anychar, digit1, line_ending, multispace1, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[derive(Debug)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

fn cell(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    Ok((
        input,
        match c {
            "   " => None,
            v => Some(v),
        },
    ))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), cell)(input)?;

    Ok((input, result))
}

fn get_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;

    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            number: number as usize,
            from: from as usize - 1,
            to: to as usize - 1,
        },
    ))
}

fn cells(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, cells_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _nums) = many1(preceded(multispace1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, moves) = separated_list1(newline, get_move)(input)?;

    let mut cells_vertical: Vec<Vec<Option<&str>>> = vec![Vec::new(); cells_horizontal.len() + 1];
    for vec in cells_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            cells_vertical[i].push(c.clone());
        }
    }
    dbg!(input, &moves, &cells_vertical);

    Ok((
        input,
        (
            cells_vertical
                .iter()
                .map(|v| v.iter().filter_map(|v| *v).collect())
                .collect(),
            moves,
        ),
    ))
}

pub fn part_1(input: &str) -> String {
    let (_, (mut cells, moves)) = cells(input).unwrap();

    for Move { to, from, number } in moves.iter() {
        let len = cells[*from as usize].len();
        for c in cells[*from as usize]
            .drain((len - *number as usize)..)
            .rev()
            .collect::<Vec<&str>>()
            .iter()
        {
            cells[*to as usize].push(c)
        }
    }

    cells
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect()
}

pub fn part_2(input: &str) -> String {
    let (_, (mut cells, moves)) = cells(input).unwrap();

    for Move { to, from, number } in moves.iter() {
        let len = cells[*from as usize].len();
        for c in cells[*from as usize]
            .drain((len - *number as usize)..)
            .collect::<Vec<&str>>()
            .iter()
        {
            cells[*to as usize].push(c)
        }
    }

    cells
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, "MCD");
    }
}
