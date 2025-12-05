use std::cmp::min;

use regex::Regex;

fn parse_input(input: String) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut ingridients = Vec::new();
    for line in input.lines() {
        if line.contains('-') {
            let re = Regex::new(r"(\d+)-(\d+)$").unwrap();
            let captures = re.captures(&line).unwrap();
            ranges.push((
                captures[1].parse::<i64>().unwrap(),
                captures[2].parse::<i64>().unwrap(),
            ));
        } else if !line.is_empty() {
            ingridients.push(line.parse::<i64>().unwrap());
        }
    }
    return (ranges, ingridients);
}

pub fn part1(input: String) -> i32 {
    let (ranges, mut ingridients) = parse_input(input);
    let mut fresh = 0;
    let mut all_borders = Vec::new();
    for (r1, r2) in &ranges {
        all_borders.push((r1, (r1, r2)));
        all_borders.push((r2, (r1, r2)));
    }
    ingridients.sort();
    for i in ingridients {
        for (r1, r2) in &ranges {
            if *r1 <= i && i <= *r2 {
                fresh += 1;
                break;
            }
        }
    }
    return fresh;
}

pub fn part2(input: String) -> i64 {
    let (ranges, _) = parse_input(input);
    let mut fresh = 0;
    let mut all_borders = Vec::new();
    for i in 0..ranges.len() {
        let (r1, r2) = ranges[i];
        all_borders.push((r1, i));
        all_borders.push((r2 + 1, i));
    }
    all_borders.sort_by_key(|x| x.0);
    let mut cur_ranges = Vec::new();
    let mut min_value = all_borders[0].0;
    for (b, i) in all_borders {
        if b == ranges[i].1 + 1 {
            for j in 0..cur_ranges.len() {
                if cur_ranges[j] == i {
                    cur_ranges.remove(j);
                    break;
                }
            }
            if cur_ranges.is_empty() {
                fresh += b - min_value;
            }
        } else {
            if cur_ranges.is_empty() {
                min_value = b;
            }
            cur_ranges.push(i);
        }
    }
    return fresh;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("3-5
10-14
16-20
12-18

1
5
8
11
17
32")), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("3-5
10-14
16-20
12-18")), 14);
    }
}