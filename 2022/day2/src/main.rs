use std::char::ParseCharError;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Round {
    elf: u32,
    player: u32,
}

impl FromStr for Round {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(" ");
        Ok(Round {
            elf: iter.next().unwrap().chars().next().unwrap() as u32 - 'A' as u32,
            player: iter.next().unwrap().chars().next().unwrap() as u32 - 'X' as u32,
        })
    }
}

fn parse_input(input: &str) -> Vec<Round> {
    let mut v = Vec::new();
    let rounds = input.trim().split("\n");
    for round in rounds {
        v.push(Round::from_str(round.into()).unwrap())
    }
    v
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    let rounds = parse_input(&input);
    println!("Part 1: {}", part1(&rounds));
    println!("Part 2: {}", part2(&rounds));
}

fn part1(rounds: &Vec<Round>) -> u32 {
    // let scores: Vec<u32> = rounds
    rounds
        .iter()
        .map(|r| {
            if (r.elf == 2 && r.player == 0) || r.player == r.elf + 1 {
                6 + r.player + 1
            } else if r.elf == r.player {
                3 + r.player + 1
            } else {
                r.player + 1
            }
        } as u32)
        .sum()
}

fn part2(rounds: &Vec<Round>) -> u32 {
    rounds
        .iter()
        .map(|r| match r.player {
            0 => {
                if r.elf == 0 {
                    3
                } else {
                    r.elf
                }
            }
            1 => 3 + r.elf + 1,
            2 => {
                if r.elf == 2 {
                    6 + 1
                } else {
                    6 + r.elf + 2
                }
            }
            _ => panic!("We should not get here"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_round() {
        assert_eq!(Round::from_str("A Y").unwrap(), Round { elf: 0, player: 1 })
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![Round { elf: 0, player: 0 }]), 4);
        assert_eq!(part1(&vec![Round { elf: 1, player: 0 }]), 1);
        assert_eq!(part1(&vec![Round { elf: 2, player: 0 }]), 7);
        assert_eq!(part1(&vec![Round { elf: 0, player: 1 }]), 8);
        assert_eq!(part1(&vec![Round { elf: 1, player: 1 }]), 5);
        assert_eq!(part1(&vec![Round { elf: 2, player: 1 }]), 2);
        assert_eq!(part1(&vec![Round { elf: 0, player: 2 }]), 3);
        assert_eq!(part1(&vec![Round { elf: 1, player: 2 }]), 9);
        assert_eq!(part1(&vec![Round { elf: 2, player: 2 }]), 6);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![Round { elf: 0, player: 1 }]), 4);
        assert_eq!(part2(&vec![Round { elf: 1, player: 1 }]), 5);
        assert_eq!(part2(&vec![Round { elf: 2, player: 1 }]), 6);
        assert_eq!(part2(&vec![Round { elf: 0, player: 0 }]), 3);
        assert_eq!(part2(&vec![Round { elf: 1, player: 0 }]), 1);
        assert_eq!(part2(&vec![Round { elf: 2, player: 0 }]), 2);
        assert_eq!(part2(&vec![Round { elf: 0, player: 2 }]), 8);
        assert_eq!(part2(&vec![Round { elf: 1, player: 2 }]), 9);
        assert_eq!(part2(&vec![Round { elf: 2, player: 2 }]), 7);
    }
}
