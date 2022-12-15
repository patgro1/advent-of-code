use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fs,
};

#[derive(Debug, Clone)]
enum Op {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<Vec<u64>>,
    op: Op,
    test: u64,
    test_pass_monkey: usize,
    test_fail_monkey: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_def in input.trim().split("\n\n") {
        let mut lines = monkey_def.split("\n").skip(1);
        let items: Vec<_> = lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut operation_str = lines
            .next()
            .unwrap()
            .split(": new = old ")
            .skip(1)
            .next()
            .unwrap()
            .split(" ");
        let operation = match operation_str.next().unwrap() {
            "+" => Op::Add(operation_str.next().unwrap().parse().unwrap()),
            "*" => {
                let operand = operation_str.next().unwrap();
                if operand == "old" {
                    Op::Square
                } else {
                    Op::Mult(operand.parse().unwrap())
                }
            }
            _ => panic!("Unhandle operand"),
        };
        let test: u64 = lines
            .next()
            .unwrap()
            .split("by ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let test_pass_monkey = lines
            .next()
            .unwrap()
            .split("true: throw to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let test_fail_monkey = lines
            .next()
            .unwrap()
            .split("false: throw to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            items: RefCell::new(items),
            op: operation,
            test,
            test_pass_monkey,
            test_fail_monkey,
        })
    }

    monkeys
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    let mut monkeys = parse_input(&input);
    println!("Part 1: {}", solve(&mut monkeys.clone(), |x| x / 3, 20));
    let modulo: u64 = monkeys.iter().map(|x| x.test).product();
    println!("Part 2: {}", solve(&mut monkeys, |x| x % modulo, 10000));
}

fn solve(monkeys: &mut Vec<Monkey>, worry_fn: impl Fn(u64) -> u64, rounds: usize) -> u64 {
    let mut inspected = vec![0 as u64; monkeys.len()];
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.items.borrow().iter() {
                inspected[i] += 1;
                let mut new_item: u64 = match monkey.op {
                    Op::Add(o) => item + o,
                    Op::Mult(o) => item * o,
                    Op::Square => item * item,
                };
                new_item = worry_fn(new_item);
                if new_item % monkey.test == 0 {
                    monkeys[monkey.test_pass_monkey]
                        .items
                        .borrow_mut()
                        .push(new_item);
                } else {
                    monkeys[monkey.test_fail_monkey]
                        .items
                        .borrow_mut()
                        .push(new_item);
                }
            }
            monkey.items.borrow_mut().clear();
        }
    }
    inspected.sort();
    println!("{:?}", inspected);
    inspected.iter().rev().take(2).product()
}

#[cfg(test)]
mod test {
    use crate::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(solve(&mut parse_input(&INPUT), |x| x / 3, 20), 10605);
    }

    #[test]
    fn test_part2() {
        let mut monkeys = parse_input(&INPUT);
        let modulo: u64 = monkeys.iter().map(|x| x.test).product();
        assert_eq!(solve(&mut monkeys, |x| x % modulo, 10000), 2713310158);
    }
}
