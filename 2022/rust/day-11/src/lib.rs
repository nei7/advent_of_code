use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{multispace1, newline},
    multi::separated_list1,
    number::complete,
    sequence::{delimited, preceded},
    IResult, *,
};

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recipient: u64,
    false_recipient: u64,
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    items: VecDeque<u64>,
    test: Test,
    touch_count: u64,
}

impl Monkey {
    fn inspect(&mut self, relief_lowers_worry_level: bool, magic_trick: u64) -> u64 {
        self.touch_count += 1;

        let item = self.items.pop_front().unwrap();

        let worry_level = match &self.operation {
            Operation::Mul((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let result = num_a * num_b;

                result % magic_trick
            }
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let result = num_a + num_b;

                result % magic_trick
            }
        };

        if relief_lowers_worry_level {
            worry_level / 3
        } else {
            worry_level
        }
    }

    fn test(&self, item: u64) -> u64 {
        if item % self.test.divisible == 0 {
            self.test.true_recipient
        } else {
            self.test.false_recipient
        }
    }
}

pub fn part_1(input: &str) -> u64 {
    let mut monkeys = input
        .split("\n\n")
        .into_iter()
        .map(|s| monkey(s).unwrap().1)
        .collect::<Vec<Monkey>>();

    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let item = monkey.inspect(true, magic_trick);
                let monkey_to_send_to = monkey.test(item);

                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.touch_count);

    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.touch_count)
        .product::<u64>()
}

pub fn part_2(input: &str) -> u64 {
    // let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    let mut monkeys = input
        .split("\n\n")
        .into_iter()
        .map(|s| monkey(s).unwrap().1)
        .collect::<Vec<Monkey>>();

    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();

    for _ in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            // println!("Monkey {monkey_index}:");
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();

                let item = monkey.inspect(false, magic_trick);

                let monkey_to_send_to = monkey.test(item);
                // println!("    Item with worry level {item} is thrown to monkey {monkey_to_send_to}.");
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;

    let (input, _) = multispace1(input)?;

    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;

    let (input, operation) = operation(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = test(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            test,
            operation,
            touch_count: 0,
        },
    ))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;

    let (input, val1) = value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("+"), tag("*"))), multispace1)(input)?;
    let (input, val2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mul((val1, val2)),
        "+" => Operation::Add((val1, val2)),
        _ => panic!("invalid operator"),
    };

    Ok((input, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) =
        preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)?;

    let (input, _) = multispace1(input)?;

    let (input, true_recipient) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;

    let (input, _) = multispace1(input)?;

    let (input, false_recipient) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;

    Ok((
        input,
        Test {
            divisible,
            true_recipient,
            false_recipient,
        },
    ))
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(|v| Value::Num(v)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part_2() {
        let res = part_2(INPUT);
        assert_eq!(res, 2713310158)
    }
}
