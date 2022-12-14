use std::fs;

use nom::{
    branch::alt,
    bytes::{self, streaming::tag},
    combinator::{all_consuming, map},
    sequence::preceded,
    Finish, IResult,
};

#[derive(Debug, Clone)]
struct Noop;
fn parse_noop(input: &str) -> IResult<&str, Noop> {
    map(tag("noop"), |_| Noop)(input)
}

#[derive(Debug, Clone)]
struct Addx(i32);
fn parse_add(input: &str) -> IResult<&str, Addx> {
    map(preceded(tag("addx "), nom::character::complete::i32), Addx)(input)
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}
impl From<Noop> for Instruction {
    fn from(_noop: Noop) -> Self {
        Instruction::Noop
    }
}
impl From<Addx> for Instruction {
    fn from(_add: Addx) -> Self {
        Instruction::Addx(_add.0)
    }
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((map(parse_noop, Into::into), map(parse_add, Into::into)))(input)
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: \n{}", part2(&input));
}

fn get_x_per_cycle(input: &str) -> Vec<i32> {
    let mut x_per_cycle: Vec<i32> = vec![];
    let mut x = 1;
    let instructions = input
        .trim()
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
    for ins in instructions {
        x_per_cycle.push(x);
        match ins {
            Instruction::Noop => {}
            Instruction::Addx(v) => {
                x += v;
                x_per_cycle.push(x);
            }
        }
    }
    x_per_cycle
}

fn part1(input: &str) -> i32 {
    let x_per_cycle = get_x_per_cycle(input);
    let cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut sig_str_sum = 0;
    for cycle in cycles {
        println!("Cycle {:?}, Str: {:?}", cycle, x_per_cycle[cycle - 2]);
        sig_str_sum += cycle as i32 * x_per_cycle[cycle - 2];
    }
    sig_str_sum
}

fn part2(input: &str) -> String {
    let mut output: String = "".to_string();
    let x_per_cycle = get_x_per_cycle(input);
    for line in 0..6 {
        for row in 0..40 {
            if line == 0 && row == 0 {
                output.push('#');
            } else {
                let idx = 40 * line + row;
                println!("idx: {:?}, x: {:?}", idx, x_per_cycle[idx]);

                if (row as i32 - x_per_cycle[idx - 1]).abs() <= 1 {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
        }
        if line < 5 {
            output.push('\n')
        }
    }
    output
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const EXPECTED_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), EXPECTED_OUTPUT.to_string());
    }
}
