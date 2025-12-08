use core::num;

use array2d::Array2D;

fn parse_input(input: String) -> (Array2D<i64>, Vec<char>) {
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    for line in input.lines() {
        if line.contains('*') || line.contains('+') {
            ops = line.split_ascii_whitespace().map(|x| x.chars().nth(0).unwrap()).collect::<Vec<_>>();
        } else {
            numbers.push(line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>());
        }
    }
    return(Array2D::from_rows(&numbers).unwrap(), ops);
}

fn parse_input_p2(input: String) -> (Array2D<i64>, Vec<char>) {
    let mut ops: Vec<char> = Vec::new();
    let lines_v = input.lines().collect::<Vec<&str>>();
    let w = lines_v[0].len();
    let h = lines_v.len() - 1;
    ops = lines_v[lines_v.len() - 1].split_ascii_whitespace().map(|x| x.chars().nth(0).unwrap()).collect::<Vec<_>>();
    let mut numbers: Array2D<i64> = Array2D::filled_with(0, h, ops.len());
    let mut cur_col = 0;
    let mut cur_len = w + 1;
    for col in (0..w).rev() {
        for i in 0..h {
            let c = lines_v[i].chars().nth(col).unwrap();
            if c.is_ascii_whitespace() {
                continue;
            }
            let n = c.to_digit(10).unwrap() as i64;
            numbers[(cur_len - col - 2, cur_col)] = numbers[(cur_len - col - 2, cur_col)] * 10 + n;
        }
        if !lines_v[lines_v.len() - 1].chars().nth(col).unwrap().is_ascii_whitespace() {
            cur_col += 1;
            cur_len = col;
        }
    }
    ops.reverse();
    return (numbers, ops);
}

pub fn part1(input: String) -> i64 {
    let (numbers, ops) = parse_input(input);
    let mut ans: i64 = 0;
    for i in 0..ops.len() {
        if ops[i] == '*' {
            ans += numbers.column_iter(i).unwrap().product::<i64>();
        } else {
            ans += numbers.column_iter(i).unwrap().sum::<i64>();
        }
    }
    return ans;
}

pub fn part2(input: String) -> i64 {
    let (numbers, ops) = parse_input_p2(input);
    let mut ans: i64 = 0;
    for i in 0..ops.len() {
        if ops[i] == '*' {
            ans += numbers.column_iter(i).unwrap().filter(|x| x.is_positive()).product::<i64>();
        } else {
            ans += numbers.column_iter(i).unwrap().sum::<i64>();
        }
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ")), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ")), 3263827);
        assert_eq!(part2(String::from("123 328 51 64 
 45 64  87 23 
  6 98  15 314
*   +   *  +  ")), 111902);
    }
}