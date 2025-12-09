use std::{cmp::{max, min}, collections::HashSet};

use itertools::{Itertools, all};

fn parse_input(input: String) -> Vec<(u32, u32)> {
    input.lines()
        .map(|s| s.split(',').map(|x| x.parse::<u32>().unwrap()).collect_tuple().unwrap())
        .collect()
}

fn distance(p1: (u32, u32), p2: (u32, u32), plus_one: bool) -> u64 {
    let pl_one = u32::from(plus_one);
    u64::from(max(p1.0, p2.0) - min(p1.0, p2.0) + pl_one) * u64::from(max(p1.1, p2.1) - min(p1.1, p2.1) + pl_one)
}

pub fn part1(input: String) -> u64 {
    let coords = parse_input(input);
    let mut max_dist = 0;
    for p1 in &coords {
        for p2 in &coords {
            max_dist = max(max_dist, distance(*p1, *p2, true));
        }
    }
    max_dist
}

fn find_pos(p: (u32, u32), all_x: &[&u32], all_y: &[&u32]) -> (usize, usize) {
    let mut p_i = 0;
    let mut p_j = 0;
    while *all_x[p_i] < p.0 { p_i += 1; }
    while *all_y[p_j] < p.1 { p_j += 1; }
    (p_i, p_j)
}

pub fn part2(input: String) -> u64 {
    let coords = parse_input(input);
    let mut all_x = HashSet::new();
    let mut all_y = HashSet::new();
    for p in &coords {
        all_x.insert(p.0);
        all_y.insert(p.1);
    }
    all_x.insert(0);
    all_x.insert(1_000_000);
    all_y.insert(0);
    all_y.insert(1_000_000);
    let mut all_x_sorted = all_x.iter().collect_vec();
    all_x_sorted.sort();
    let mut all_y_sorted = all_y.iter().collect_vec();
    all_y_sorted.sort();
    
    let mut grid = array2d::Array2D::filled_with(0, all_x.len(), all_y.len());
    let p1 = coords[0];
    let (p1_i, p1_j) = find_pos(p1, &all_x_sorted, &all_y_sorted);
    for p2 in &coords {
        let (p2_i, p2_j) = find_pos(*p2, &all_x_sorted, &all_y_sorted);
        for i in min(p1_i, p2_i)..max(p1_i, p2_i) {
            for j in min(p1_j, p2_j)..max(p1_j, p2_j) {
                grid[(i, j)] += 1;
            }
        }
    }
    
    let mut max_dist = 0;
    for p1 in &coords {
        let (p1_i, p1_j) = find_pos(*p1, &all_x_sorted, &all_y_sorted);
        'p2: for p2 in &coords {
            if p1 == p2 {
                continue;
            }
            let (p2_i, p2_j) = find_pos(*p2, &all_x_sorted, &all_y_sorted);
            for i in min(p1_i, p2_i)..max(p1_i, p2_i) {
                for j in min(p1_j, p2_j)..max(p1_j, p2_j) {
                    if grid[(i, j)] % 2 == 0 {
                        continue 'p2;
                    }
                }
            }
            max_dist = max(max_dist, distance(*p1, *p2, true));
        }
    }

    max_dist
}


mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3")), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3")), 24);
        assert_eq!(part2(String::from("2,1
11,1
11,7
3,7
3,5
9,5
9,3
2,3")), 30);
    }
}
