use std::char::ParseCharError;
use std::fs;
use std::str::FromStr;

struct Region {
    start: u32,
    end: u32,
}

impl FromStr for Region {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split('-').collect();
        Ok(Region {
            start: splitted[0].parse().unwrap(),
            end: splitted[1].parse().unwrap(),
        })
    }
}

fn create_pairs(line: &str) -> Vec<Region> {
    let regions: Vec<&str> = line.split(',').collect();
    vec![
        Region::from_str(regions[0]).unwrap(),
        Region::from_str(regions[1]).unwrap(),
    ]
}

fn do_region_fully_overlap(a: &Region, b: &Region) -> bool {
    if a.start >= b.start && a.end <= b.end {
        return true;
    }
    if b.start >= a.start && b.end <= a.end {
        return true;
    }
    return false;
}

fn do_region_overlap(a: &Region, b: &Region) -> bool {
    // Check if b overlaps a
    if b.start <= a.start && b.end >= a.start {
        return true;
    }
    if b.start >= a.start && b.end <= a.end {
        return true;
    }
    // Check if a overlaps b
    if a.start <= b.start && a.end >= b.start {
        return true;
    }
    if a.start >= b.start && a.end <= b.end {
        return true;
    }
    return false;
}

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|l| create_pairs(l))
        .map(|r| do_region_fully_overlap(&r[0], &r[1]))
        .map(|o| o as u32)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|l| create_pairs(l))
        .map(|r| do_region_overlap(&r[0], &r[1]))
        .map(|o| o as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_is_fully_overlapping() {
        assert_eq!(
            do_region_fully_overlap(&Region { start: 1, end: 2 }, &Region { start: 3, end: 4 }),
            false
        );
        assert_eq!(
            do_region_fully_overlap(&Region { start: 1, end: 6 }, &Region { start: 3, end: 4 }),
            true
        );
        assert_eq!(
            do_region_fully_overlap(&Region { start: 1, end: 3 }, &Region { start: 3, end: 4 }),
            false
        );
    }
    #[test]
    fn test_is_overlapping() {
        assert_eq!(
            do_region_overlap(&Region { start: 1, end: 2 }, &Region { start: 3, end: 4 }),
            false
        );
        assert_eq!(
            do_region_overlap(&Region { start: 1, end: 6 }, &Region { start: 3, end: 4 }),
            true
        );
        assert_eq!(
            do_region_overlap(&Region { start: 1, end: 3 }, &Region { start: 3, end: 4 }),
            true
        );
    }
}
