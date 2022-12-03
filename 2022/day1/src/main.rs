use std::fs;
use std::mem;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let splited_str = input.trim().split("\n\n");
    splited_str
        .into_iter()
        .map(|x| {
            x.trim()
                .split('\n')
                .into_iter()
                .map(|y| y.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let mut top = vec![0; 3];
    let splited_str = input.trim().split("\n\n");
    let sums = splited_str.into_iter().map(|x| {
        x.trim()
            .split('\n')
            .into_iter()
            .map(|y| y.parse::<i64>().unwrap())
            .sum::<i64>()
    });
    for sum in sums {
        if sum > *top.iter().min().unwrap() {
            let min_pos = top
                .iter()
                .position(|x| *x == *top.iter().min().unwrap())
                .unwrap();
            let _ = mem::replace(&mut top[min_pos], sum);
        }
        println!("{:?}", top)
    }
    top.into_iter().sum()
}
