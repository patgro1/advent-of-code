use std::fs;

fn main() {
    let input = fs::read_to_string("inputs.txt").expect("Impossible to read file");
    let map = get_map(&input);
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn get_map(input: &str) -> Vec<Vec<u8>> {
    let mut map = vec![];
    for line in input.lines() {
        let mut line_vec: Vec<u8> = vec![];
        for c in line.trim().chars() {
            line_vec.push(c.to_digit(10).unwrap() as u8);
        }
        map.push(line_vec);
    }
    map
}

fn part1(map: &Vec<Vec<u8>>) -> u64 {
    let mut visibles = 0;
    for (l_idx, line) in map.iter().enumerate() {
        for (r_idx, row) in line.iter().enumerate() {
            if l_idx == 0 || l_idx == map.len() - 1 || r_idx == 0 || r_idx == line.len() - 1 {
                visibles += 1;
            } else {
                let mut visibility: Vec<bool> = vec![true, true, true, true];
                // Naive implementation
                for i in 0..l_idx {
                    if map[i][r_idx] >= *row {
                        visibility[0] = false;
                    }
                }
                for i in l_idx + 1..map.len() {
                    if map[i][r_idx] >= *row {
                        visibility[1] = false;
                    }
                }
                for i in 0..r_idx {
                    if map[l_idx][i] >= *row {
                        visibility[2] = false;
                    }
                }
                for i in r_idx + 1..line.len() {
                    if map[l_idx][i] >= *row {
                        visibility[3] = false;
                    }
                }
                if visibility.iter().find(|x| **x) == Some(&true) {
                    visibles += 1;
                }
            }
        }
    }
    visibles
}

fn part2(map: &Vec<Vec<u8>>) -> u64 {
    let mut scene_score: Vec<Vec<u32>> = vec![];
    for (l_idx, line) in map.iter().enumerate() {
        let mut line_scene_score: Vec<u32> = vec![];
        for (r_idx, row) in line.iter().enumerate() {
            if l_idx == 0 || l_idx == map.len() - 1 || r_idx == 0 || r_idx == line.len() - 1 {
                line_scene_score.push(0);
            } else {
                let mut visibility: Vec<u32> = vec![0, 0, 0, 0];
                // Naive implementation
                for i in (0..l_idx).rev() {
                    visibility[0] += 1;
                    if map[i][r_idx] >= *row {
                        break;
                    }
                }
                for i in l_idx + 1..map.len() {
                    visibility[1] += 1;
                    if map[i][r_idx] >= *row {
                        break;
                    }
                }
                for i in (0..r_idx).rev() {
                    visibility[2] += 1;
                    if map[l_idx][i] >= *row {
                        break;
                    }
                }
                for i in r_idx + 1..line.len() {
                    visibility[3] += 1;
                    if map[l_idx][i] >= *row {
                        break;
                    }
                }
                line_scene_score
                    .push(visibility[0] * visibility[1] * visibility[2] * visibility[3]);
            }
        }
        scene_score.push(line_scene_score);
    }
    scene_score.into_iter().flatten().max().unwrap() as u64
}

#[cfg(test)]
mod test {
    use crate::{get_map, part1, part2};
    const INPUT: &str = "30373
        25512
        65332
        33549
        35390";
    #[test]
    #[ignore]
    fn test_map() {
        assert_eq!(
            get_map(INPUT),
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 4, 9, 0]
            ]
        );
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_map(INPUT)), 21);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_map(INPUT)), 8);
    }
}
