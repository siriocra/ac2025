use regex::Regex;

fn parse_line(line: &str) -> (i32, i32) {
    let re = Regex::new(r"([LR])(\d+)$").unwrap();
    let captures = re.captures(&line).unwrap();
    let mut direction = 1;
    if captures[1].contains('L') {
        direction = -1;
    }
    return (
        direction,
        captures[2].parse::<i32>().unwrap(),
    );
}

pub fn part1(input: String) -> i32 {
    let mut start = 50;
    let max_num = 100;
    let mut ans = 0;
    for line in input.lines() {
        let (d, shift) = parse_line(line);
        start = (start + d * shift + max_num) % max_num;
        if start == 0 {
            ans += 1;
        }
    }
    return ans;
}

pub fn part2(input: String) -> i32 {
    let mut start = 50;
    let max_num = 100;
    let mut ans = 0;
    for line in input.lines() {
        let (d, shift) = parse_line(line);
        if start == 0 && d == -1 {
            ans -= 1;
        }
        let mut times = 0;
        if start + d * shift <= 0 || start + d * shift >= max_num {
            if start + d * shift <= 0 {
                times = -((start + d * shift) / max_num) + 1;
            } else {
                times = (start + d * shift) / max_num;
            }
        }
        ans += times;
        start = (start + d * shift + max_num * times) % max_num;
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82")), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82")), 6);
        assert_eq!(part2(String::from("R1000")), 10);
        assert_eq!(part2(String::from("L1000")), 10);
        assert_eq!(part2(String::from("L200")), 2);
        assert_eq!(part2(String::from("L50")), 1);
        assert_eq!(part2(String::from("L51")), 1);
        assert_eq!(part2(String::from("L50
L100")), 2);
        assert_eq!(part2(String::from("L50
R200")), 3);
        assert_eq!(part2(String::from("L50
R199")), 2);
        assert_eq!(part2(String::from("L50
L99
L101
")), 3);
        assert_eq!(part2(String::from("L200
L400")), 6);
    }
}