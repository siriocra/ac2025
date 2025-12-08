use std::collections::{HashMap, HashSet};

use array2d::Array2D;

fn parse_input(input: String) -> Array2D<char> {
    return Array2D::from_rows(&input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>()).unwrap();
}

pub fn part1(input: String) -> i32 {
    let mut grid = parse_input(input);
    let mut split = 0;
    for row in 1..grid.num_rows() {
        for col in 0..grid.num_columns() {
            if grid[(row-1, col)] == '.' || grid[(row-1, col)] == '^' {
                continue;
            }
            if grid[(row, col)] == '^' {
                split += 1;
                grid[(row, col-1)] = '|';
                grid[(row, col+1)] = '|';
            } else {
                grid[(row, col)] = '|';
            }
        }
    }
    return split;
}

pub fn part2(input: String) -> i64 {
    let grid = parse_input(input);
    let mut timelines = HashMap::new();
    for col in 0..grid.num_columns() {
        if grid[(0, col)] == 'S' {
            timelines.insert(col, 1);
        }
    }
    for row in 1..grid.num_rows() {
        let mut new_timelines = HashMap::new();
        for (col, v) in timelines.clone() {
            if grid[(row, col)] == '^' {
                new_timelines.entry(col - 1).and_modify(|x| *x += v).or_insert(v);
                new_timelines.entry(col + 1).and_modify(|x| *x += v).or_insert(v);
            } else {
                new_timelines.entry(col).and_modify(|x| *x += v).or_insert(v);
            }
        }
        timelines = new_timelines;
    }
    return timelines.values().sum::<i64>();
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............")), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............")), 40);
    }
}