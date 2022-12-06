use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn find_min_no_intersect(input: &str, min_chain: usize) -> u32{
    let mut min: u32 = 0;
    let input = input.to_string();
    for idx in 0..input.len() - (min_chain - 1) {
        let slice = &input[idx..idx + min_chain];
        if intersect(slice) == usize::MAX {
            min = idx as u32 + min_chain as u32;
            break;
        }
    }
    min
}

fn part1(input: &str) -> u32 {
    find_min_no_intersect(input, 4)
}

fn part2(input: &str) -> u32 {
    find_min_no_intersect(input, 14)
}

fn intersect(input: &str) -> usize {
    let input = input.to_string();
    for idx_a in 0..input.len() {
        for idx_b in idx_a+1..input.len() {
            if &input[idx_a..idx_a+1] == &input[idx_b..idx_b+1] {
                return idx_a as usize;
            }
        }
    }
    return usize::MAX;

}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_part1() { 
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn test_part2() { 
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
