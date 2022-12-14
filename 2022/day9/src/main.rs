use std::{collections::HashSet, fs};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{all_consuming, map, value},
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
}

#[derive(Clone, Debug)]
struct Move {
    dir: Direction,
    delta: u32,
}

impl Move {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Direction::parse,
                preceded(tag(" "), nom::character::complete::u32),
            )),
            |(dir, delta)| Self { dir, delta },
        )(input)
    }
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let moves = input
        .trim()
        .lines()
        .map(|l| all_consuming(Move::parse)(l).finish().unwrap().1);
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut head: Coord = Coord { x: 0, y: 0 };
    let mut tail: Coord = Coord { x: 0, y: 0 };
    for _move in moves {
        for _ in 0.._move.delta {
            // Move head
            match _move.dir {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }
            let diff = Coord {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            // Move tail
            let move_tail = (tail.x - head.x).abs() > 1 || (tail.y - head.y).abs() > 1;
            if move_tail {
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
            }
            // update map
            seen.insert(tail);
        }
    }
    seen.len() as u64
}

fn part2(input: &str) -> u64 {
    let moves = input
        .trim()
        .lines()
        .map(|l| all_consuming(Move::parse)(l).finish().unwrap().1);
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut rope: Vec<Coord> = vec![Coord { x: 0, y: 0 }; 10];
    for _move in moves {
        for _ in 0.._move.delta {
            // Move head
            match _move.dir {
                Direction::Up => rope[0].y += 1,
                Direction::Down => rope[0].y -= 1,
                Direction::Left => rope[0].x -= 1,
                Direction::Right => rope[0].x += 1,
            }
            // Move rest of rope
            for idx in 1..10 {
                let diff = Coord {
                    x: rope[idx - 1].x - rope[idx].x,
                    y: rope[idx - 1].y - rope[idx].y,
                };
                let move_node = (rope[idx].x - rope[idx - 1].x).abs() > 1
                    || (rope[idx].y - rope[idx - 1].y).abs() > 1;
                if move_node {
                    rope[idx].x += diff.x.signum();
                    rope[idx].y += diff.y.signum();
                }
            }
            // Update tail
            seen.insert(rope[9]);
        }
    }
    seen.len() as u64
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1);
        assert_eq!(part2(INPUT2), 36);
    }
}
