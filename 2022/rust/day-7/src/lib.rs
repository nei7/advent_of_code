#![feature(iter_intersperse)]
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::{complete::is_a, streaming::tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    number::complete,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    use nom::character::complete;

    let (input, (size, name)) =
        separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;

    Ok((input, Files::File { size, name }))
}

fn dir(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), tag("/"), alpha1))(input)?;
    Ok((
        input,
        match dir {
            ".." => Operation::Cd(Cd::Up),
            "/" => Operation::Cd(Cd::Root),
            name => Operation::Cd(Cd::Down(name)),
        },
    ))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, dir)))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((cd, ls)))(input)?;
    Ok((input, cmds))
}

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

pub fn part_1(input: &str) -> u32 {
    let (_, cmds) = commands(input).unwrap();

    dbg!(&cmds);
    let mut dirs: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut ctx: Vec<&str> = vec![];

    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                ctx.push("/");
            }
            Operation::Cd(Cd::Up) => {
                ctx.pop();
            }
            Operation::Cd(Cd::Down(name)) => ctx.push(name),
            Operation::Ls(files) => {
                dirs.entry(ctx.iter().cloned().intersperse("/").collect::<String>())
                    .or_insert(vec![]);

                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            dirs.entry(ctx.iter().cloned().intersperse("/").collect::<String>())
                                .and_modify(|vec| vec.push(File { size: *size, name }));
                        }
                        Files::Dir(_) => {}
                    }
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in dirs.iter() {
        let directories = path.split("/").collect::<Vec<&str>>();

        let size = files.iter().map(|File { size, .. }| size).sum::<u32>();
        for i in 0..directories.len() {
            sizes
                .entry(
                    (&directories[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    sizes
        .iter()
        .map(|(_, size)| size)
        .filter(|&&size| size < 100000)
        .sum::<u32>()
}

pub fn part_2(input: &str) -> u32 {
    let (_, cmds) = commands(input).unwrap();

    dbg!(&cmds);
    let mut dirs: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut ctx: Vec<&str> = vec![];

    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                ctx.push("/");
            }
            Operation::Cd(Cd::Up) => {
                ctx.pop();
            }
            Operation::Cd(Cd::Down(name)) => ctx.push(name),
            Operation::Ls(files) => {
                dirs.entry(ctx.iter().cloned().intersperse("/").collect::<String>())
                    .or_insert(vec![]);

                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            dirs.entry(ctx.iter().cloned().intersperse("/").collect::<String>())
                                .and_modify(|vec| vec.push(File { size: *size, name }));
                        }
                        Files::Dir(_) => {}
                    }
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in dirs.iter() {
        let directories = path.split("/").collect::<Vec<&str>>();

        let size = files.iter().map(|File { size, .. }| size).sum::<u32>();
        for i in 0..directories.len() {
            sizes
                .entry(
                    (&directories[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }
    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get("").unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least = needed_space - current_free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    **valid_dirs.iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);

        assert_eq!(result, 24933642);
    }
}
