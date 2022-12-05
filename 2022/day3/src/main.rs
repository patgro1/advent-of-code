use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn split_rucksack(content: &str) -> Result<Vec<&str>, String> {
    if content.len() % 2 != 0 {
        Err("This can't be divided in two".to_string())
    } else {
        let (first, last) = content.split_at(content.len() / 2);
        Ok(vec![first, last])
    }
}

fn find_common_item_priority(rucksack: Vec<&str>) -> u32 {
    let mut common: char = '\0';
    for char in rucksack[0].chars() {
        let found = match rucksack[1].find(char) {
            Some(_) => true,
            None => false,
        };
        if found {
            common = char
        }
    }
    get_item_priority(common)
}

fn get_item_priority(item: char) -> u32 {
    if item as u32 >= 'a' as u32 {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

fn find_common_item_group(rucksacks: Vec<&str>) -> u32 {
    for char in rucksacks[0].chars() {
        let found1 = match rucksacks[1].find(char) {
            Some(_) => true,
            None => false,
        };
        if found1 {
            let found2 = match rucksacks[2].find(char) {
                Some(_) => true,
                None => false,
            };
            if found2 {
                return get_item_priority(char);
            }
        }
    }
    0
}

fn split_into_group(input: &str) -> Vec<Vec<&str>> {
    let mut elves: Vec<Vec<&str>> = Vec::new();
    let lines: Vec<&str> = input.split('\n').into_iter().collect();
    for idx in 0..lines.len() / 3 {
        elves.push(vec![lines[3 * idx], lines[3 * idx + 1], lines[3 * idx + 2]])
    }
    elves
}

fn part1(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .into_iter()
        .map(|rucksack| split_rucksack(rucksack))
        .map(|rucksack| find_common_item_priority(rucksack.unwrap()))
        .sum()
}

fn part2(input: &str) -> u32 {
    split_into_group(input.trim())
        .into_iter()
        .map(|group| find_common_item_group(group))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_split_rustsack() {
        assert_eq!(split_rucksack("abcdef").unwrap(), vec! {"abc","def"});
    }
    #[test]
    fn test_find_common_item_priority() {
        assert_eq!(find_common_item_priority(vec!["abcde", "fghaj"]), 1);
        assert_eq!(find_common_item_priority(vec!["abAde", "fghaA"]), 27);
    }
}
