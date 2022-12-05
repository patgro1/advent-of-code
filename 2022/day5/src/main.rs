use std::char::ParseCharError;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Operation {
    amount: u32,
    from: u32,
    to: u32,
}

impl FromStr for Operation {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(' ').collect();
        Ok(Operation {
            amount: splitted[1].parse().unwrap(),
            from: splitted[3].parse().unwrap(),
            to: splitted[5].parse().unwrap(),
        })
    }
}

fn create_containers(containers: &str) -> Vec<Vec<String>> {
    // We assume there will always be 9 containers...
    let mut containers_vec: Vec<Vec<String>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for line in containers.lines().into_iter().rev() {
        for idx in (0..line.len()).step_by(4) {
            if line.chars().nth(idx).unwrap() == '[' {
                containers_vec[idx / 4].push(line.chars().nth(idx + 1).unwrap().to_string());
            }
        }
    }
    containers_vec
}

fn create_moves(move_list: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    for line in move_list.lines() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        operations.push(Operation {
            amount: splitted_line[1].parse().unwrap(),
            from: splitted_line[3].parse().unwrap(),
            to: splitted_line[5].parse().unwrap(),
        });
    }
    operations
}

fn solve(
    containers: &mut Vec<Vec<String>>,
    move_list: &Vec<Operation>,
    preserve_order: bool,
) -> String {
    for curr_move in move_list {
        let from_idx = curr_move.from - 1;
        let to_idx = curr_move.to - 1;
        let mut move_vector: Vec<String> = vec![];
        for _ in 0..curr_move.amount as usize {
            let item = containers[from_idx as usize].pop();
            match item {
                Some(content) => move_vector.push(content),
                _ => {}
            }
        }
        if preserve_order == true {
            move_vector.reverse();
            containers[to_idx as usize].append(&mut move_vector);
        } else {
            containers[to_idx as usize].append(&mut move_vector);
        }
    }
    let mut tops: String = "".to_string();
    for container in containers {
        match container.pop() {
            Some(c) => tops += &c,
            _ => {}
        }
    }
    tops
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    // let input = fs::read_to_string("test.txt").expect("Impossible to read file");
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    let mut containers: Vec<Vec<String>> = create_containers(splitted_input[0]);
    let moves: Vec<Operation> = create_moves(splitted_input[1]);
    println!("Part 1: {}", part1(&mut containers.clone(), &moves, false));
    println!("Part 2: {}", part2(&mut containers, &moves, true));
}

fn part1(
    containers: &mut Vec<Vec<String>>,
    move_list: &Vec<Operation>,
    preserve_order: bool,
) -> String {
    solve(containers, move_list, preserve_order)
}

fn part2(
    containers: &mut Vec<Vec<String>>,
    move_list: &Vec<Operation>,
    preserve_order: bool,
) -> String {
    solve(containers, move_list, preserve_order)
}
