use std::fs;

use camino::Utf8PathBuf;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug, Default, PartialEq)]
struct Node {
    idx: usize,
    size: u64,
    name: Utf8PathBuf,
    children: Vec<usize>,
    parent: Option<usize>,
}

fn parse_path(input: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(input)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(input: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(input)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(input: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(input)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(_cd: Cd) -> Self {
        Command::Cd(_cd.0)
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(input)
}

#[derive(Debug)]
enum Entry {
    File(u64, Utf8PathBuf),
    Dir(Utf8PathBuf),
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(input)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(input)
}

fn get_dir_idx(tree: &Vec<Node>, current_parent: usize, name: &str) -> Option<usize> {
    if let Some(parent) = tree.get(current_parent) {
        for child_idx in &parent.children {
            if let Some(child) = tree.get(*child_idx) {
                if child.name == name {
                    return Some(*child_idx);
                }
            } else {
                panic!("Trying to access a children that was never created");
            }
        }
    }
    return None;
}

fn find_node_size(tree: &Vec<Node>, node: &Node) -> u64 {
    let children_size: u64 = node
        .children
        .iter()
        .map(|n| find_node_size(tree, tree.get(*n).expect("")))
        .sum();
    node.size + children_size
}

fn get_folder_size(tree: &Vec<Node>) -> Vec<(String, u64)> {
    let mut size_vec = vec![];
    for node in tree {
        // We assume that only directories will have children
        if !node.children.is_empty() {
            size_vec.push((node.name.to_string(), find_node_size(tree, node)));
        }
    }
    size_vec
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    // let input = fs::read_to_string("sample_input.txt").expect("Impossible to read file");
    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
    let mut tree: Vec<Node> = vec![Node {
        idx: 0 as usize,
        name: Utf8PathBuf::from_str("/").unwrap(),
        size: 0, // TODO: this is not ok... need an option maybe????
        parent: None,
        children: vec![],
    }];
    let mut current_parent = 0;

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(dir) => {
                    if dir.to_string() == "/" {
                        current_parent = 0 as usize;
                    } else if dir.to_string() == ".." {
                        current_parent = tree
                            .get(current_parent)
                            .expect("Trying to access unexistant parent node")
                            .parent
                            .unwrap();
                    } else {
                        if let Some(idx) = get_dir_idx(&tree, current_parent, &dir.to_string()) {
                            current_parent = idx;
                        }
                    }
                }
            },
            Line::Entry(entry) => {
                let mut _size: u64 = 0;
                let mut _name: Utf8PathBuf;
                match entry {
                    Entry::File(size, name) => {
                        _size = size;
                        _name = name;
                    }
                    Entry::Dir(name) => {
                        _name = name;
                    }
                }
                let idx = tree.len();
                tree.get_mut(current_parent)
                    .expect("Trying to access non-existant parent")
                    .children
                    .push(idx);
                tree.push(Node {
                    idx,
                    name: _name,
                    children: vec![],
                    size: _size,
                    parent: Some(current_parent as usize),
                });
            }
        }
    }
    let sizes = get_folder_size(&tree);
    println!("Part 1: {}", part1(&sizes));
    println!("Part 2: {}", part2(&sizes));
}

fn part1(input: &Vec<(String, u64)>) -> u64 {
    input
        .into_iter()
        .filter(|x| x.1 <= 100000)
        .map(|x| x.1)
        .sum()
}

fn part2(input: &Vec<(String, u64)>) -> u64 {
    const TOTAL_SIZE: u64 = 70000000;
    const NEEDED_SIZE: u64 = 30000000;
    let space_to_free = NEEDED_SIZE - (TOTAL_SIZE - input[0].1);
    input
        .into_iter()
        .filter(|x| x.1 >= space_to_free)
        .map(|x| x.1)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, str::FromStr};

    use crate::*;
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
    fn test_part1() {
        assert_eq!(part1(INPUT), 95437);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part1(INPUT), 95437);
    }

    #[test]
    fn test_path() {
        assert_eq!(
            parse_path(".abcde").unwrap(),
            ("", Utf8PathBuf::from_str(".abcde").expect(""))
        );
    }
}
